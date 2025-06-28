"""JYAML parser implementation."""

from typing import Any, Literal, TypedDict, Unpack

from .lexer import Lexer, LexerError, Token, TokenType
from .options import ParseOptions
from .types import (
    JYAMLArray,
    JYAMLBool,
    JYAMLData,
    JYAMLNull,
    JYAMLNumber,
    JYAMLObject,
    JYAMLString,
    ParsedDocument,
)


class ParseOptionsKwargs(TypedDict, total=False):
    """Type definition for parse function kwargs."""

    strict_mode: bool
    preserve_comments: bool
    allow_duplicate_keys: bool
    max_depth: int | None
    include_comment_positions: bool
    normalize_line_endings: Literal["none", "lf", "crlf"]


class ParseError(Exception):
    """Parser error with position information."""

    def __init__(self, message: str, token: Token | None = None):
        if token:
            super().__init__(f"{message} at line {token.line}, column {token.column}")
        else:
            super().__init__(message)

        self.line: int | None = token.line if token else None
        self.column: int | None = token.column if token else None
        self.message = message


class Parser:
    """JYAML parser."""

    def __init__(self, text: str, options: ParseOptions | None = None):
        """Initialize parser with input text and options."""
        self.options = options or ParseOptions()

        # Normalize line endings if requested
        if self.options.normalize_line_endings == "lf":
            text = text.replace("\r\n", "\n").replace("\r", "\n")
        elif self.options.normalize_line_endings == "crlf":
            text = text.replace("\r\n", "\n").replace("\r", "\n").replace("\n", "\r\n")
        # "none" - no normalization

        self.lexer = Lexer(text)
        self.tokens: list[Token] = []
        self.position = 0
        self.comments: list[str] = []
        self.comment_positions: list[dict[str, Any]] = []
        self.depth = 0
        self.token_count = 0

        # Tokenize the input
        try:
            for token in self.lexer.tokenize():
                self.token_count += 1

                if token.type == TokenType.COMMENT:
                    if self.options.preserve_comments:
                        self.comments.append(token.value)
                        if self.options.include_comment_positions:
                            self.comment_positions.append(
                                {
                                    "text": token.value,
                                    "line": token.line,
                                    "column": token.column,
                                }
                            )
                else:
                    self.tokens.append(token)
        except LexerError as e:
            # Handle tab characters (forbidden by JYAML spec)
            if "Tab character" in e.message:
                # Tabs are forbidden by JYAML spec
                raise ParseError(f"Lexer error: {e.message}", None) from e
            elif self.options.strict_mode:
                raise ParseError(f"Lexer error: {e.message}", None) from e
            else:
                # In non-strict mode, try to continue parsing other errors
                pass

    def current_token(self) -> Token | None:
        """Get current token."""
        if self.position >= len(self.tokens):
            return None
        return self.tokens[self.position]

    def peek_token(self, offset: int = 1) -> Token | None:
        """Peek at token at offset from current position."""
        pos = self.position + offset
        if pos >= len(self.tokens):
            return None
        return self.tokens[pos]

    def check_depth(self):
        """Check if max depth is exceeded."""
        if self.options.max_depth is not None and self.depth > self.options.max_depth:
            raise ParseError(
                f"Maximum nesting depth exceeded: {self.options.max_depth}"
            )

    def enter_scope(self):
        """Enter a new parsing scope (array or object)."""
        self.depth += 1
        self.check_depth()

    def exit_scope(self):
        """Exit current parsing scope."""
        self.depth -= 1

    def advance(self) -> Token | None:
        """Advance to next token and return current."""
        if self.position >= len(self.tokens):
            return None
        token = self.tokens[self.position]
        self.position += 1
        return token

    def expect(self, token_type: TokenType) -> Token:
        """Expect a specific token type and advance."""
        token = self.current_token()
        if not token or token.type != token_type:
            expected = token_type.value
            actual = token.type.value if token else "EOF"
            raise ParseError(f"Expected {expected}, got {actual}", token)
        token = self.advance()
        if token is None:
            raise ParseError(f"Expected {expected}, got EOF")
        return token

    def skip_newlines(self) -> None:
        """Skip newline and indent tokens."""
        while True:
            current = self.current_token()
            if not (
                current
                and current.type
                in [
                    TokenType.NEWLINE,
                    TokenType.INDENT,
                ]
            ):
                break
            self.advance()

    def parse_value(self) -> JYAMLData:
        """Parse a JYAML value."""
        self.skip_newlines()

        token = self.current_token()
        if not token:
            raise ParseError("Unexpected end of input")

        if token.type == TokenType.NULL:
            self.advance()
            return JYAMLNull()

        elif token.type == TokenType.TRUE:
            self.advance()
            return JYAMLBool(value=True)

        elif token.type == TokenType.FALSE:
            self.advance()
            return JYAMLBool(value=False)

        elif token.type == TokenType.NUMBER:
            self.advance()
            try:
                # Try integer first
                if "." not in token.value and "e" not in token.value.lower():
                    int_value = int(token.value)
                    return JYAMLNumber(value=int_value)
                else:
                    float_value = float(token.value)
                    return JYAMLNumber(value=float_value)
            except ValueError:
                raise ParseError(f"Invalid number: {token.value}", token) from None

        elif token.type == TokenType.STRING:
            # Check if it's a block object (key without bracket)
            peek = self.peek_token()
            if peek and peek.type == TokenType.COLON:
                return self.parse_block_object()
            else:
                self.advance()
                return JYAMLString(value=token.value)

        elif token.type == TokenType.LEFT_BRACKET:
            return self.parse_flow_array()

        elif token.type == TokenType.LEFT_BRACE:
            return self.parse_flow_object()

        elif token.type == TokenType.DASH:
            return self.parse_block_array()

        elif token.type in [
            TokenType.PIPE,
            TokenType.PIPE_STRIP,
            TokenType.GREATER,
            TokenType.GREATER_STRIP,
        ]:
            # These will be handled as multiline strings by the lexer
            # and come as STRING tokens, so this shouldn't normally be reached
            raise ParseError(f"Unexpected multiline indicator: {token.value}", token)

        else:
            raise ParseError(f"Unexpected token: {token.value}", token)

    def parse_flow_array(self) -> JYAMLArray:
        """Parse flow-style array [1, 2, 3]."""
        self.enter_scope()
        self.expect(TokenType.LEFT_BRACKET)
        self.skip_newlines()

        items: list[JYAMLData] = []

        # Handle empty array
        current = self.current_token()
        if current and current.type == TokenType.RIGHT_BRACKET:
            self.advance()
            self.exit_scope()
            return JYAMLArray(value=items)

        while True:
            items.append(self.parse_value())
            self.skip_newlines()

            token = self.current_token()
            if not token:
                raise ParseError("Unexpected end of input in array")

            if token.type == TokenType.RIGHT_BRACKET:
                self.advance()
                break
            elif token.type == TokenType.COMMA:
                self.advance()
                self.skip_newlines()
                # Allow trailing comma
                current = self.current_token()
                if current and current.type == TokenType.RIGHT_BRACKET:
                    self.advance()
                    break
            else:
                raise ParseError(
                    f"Expected ',' or ']' in array, got {token.value}", token
                )

        self.exit_scope()
        return JYAMLArray(value=items)

    def parse_flow_object(self) -> JYAMLObject:
        """Parse flow-style object {"key": "value"}."""
        self.enter_scope()
        self.expect(TokenType.LEFT_BRACE)
        self.skip_newlines()

        items: dict[str, JYAMLData] = {}

        # Handle empty object
        current = self.current_token()
        if current and current.type == TokenType.RIGHT_BRACE:
            self.advance()
            self.exit_scope()
            return JYAMLObject(value=items)

        while True:
            # Parse key
            key_token = self.expect(TokenType.STRING)
            key = key_token.value

            self.skip_newlines()
            self.expect(TokenType.COLON)
            self.skip_newlines()

            # Parse value
            value = self.parse_value()
            items[key] = value

            self.skip_newlines()

            token = self.current_token()
            if not token:
                raise ParseError("Unexpected end of input in object")

            if token.type == TokenType.RIGHT_BRACE:
                self.advance()
                break
            elif token.type == TokenType.COMMA:
                self.advance()
                self.skip_newlines()
                # Allow trailing comma
                current = self.current_token()
                if current and current.type == TokenType.RIGHT_BRACE:
                    self.advance()
                    break
            elif token.type == TokenType.STRING:
                peek = self.peek_token()
                if peek and peek.type == TokenType.COLON:
                    # Another key-value pair without comma (valid in flow style with newlines)
                    continue
            else:
                raise ParseError(
                    f"Expected ',' or '}}' in object, got {token.value}", token
                )

        self.exit_scope()
        return JYAMLObject(value=items)

    def parse_block_array(self) -> JYAMLArray:
        """Parse block-style array."""
        self.enter_scope()
        items: list[JYAMLData] = []
        base_indent = None

        while True:
            current = self.current_token()
            if not (current and current.type == TokenType.DASH):
                break
            # Get current indentation level
            if base_indent is None:
                # Find the indentation before the dash
                base_indent = 0  # We'll set this based on the dash position

            self.advance()  # Skip dash
            self.skip_newlines()

            # Parse array item
            item = self.parse_value()
            items.append(item)

            self.skip_newlines()

            # Check if next line has the same indentation and dash
            current = self.current_token()
            if not (current and current.type == TokenType.DASH):
                break

        self.exit_scope()
        return JYAMLArray(value=items)

    def parse_block_object(self) -> JYAMLObject:
        """Parse block-style object."""
        self.enter_scope()
        items = {}

        while True:
            current = self.current_token()
            peek = self.peek_token()
            if not (
                current
                and current.type == TokenType.STRING
                and peek
                and peek.type == TokenType.COLON
            ):
                break
            # Parse key
            key_token = self.advance()
            if key_token is None:
                raise ParseError("Unexpected end of input while parsing object key")
            key = key_token.value

            self.expect(TokenType.COLON)

            # Skip whitespace but not newlines here
            while self.current_char_is_whitespace():
                self.advance()

            # Parse value
            value = self.parse_value()
            items[key] = value

            # Skip newlines to get to next key-value pair
            self.skip_newlines()

        self.exit_scope()
        return JYAMLObject(value=items)

    def current_char_is_whitespace(self) -> bool:
        """Check if current token is whitespace (space, not newline)."""
        return False  # We don't have space tokens, only handle in lexer

    def parse(self) -> ParsedDocument:
        """Parse the JYAML document."""
        self.skip_newlines()

        current = self.current_token()
        if not current or current.type == TokenType.EOF:
            # Empty document
            return ParsedDocument(data=JYAMLNull(), comments=self.comments)

        # Parse root value
        root_value = self.parse_value()

        # Skip any trailing newlines
        self.skip_newlines()

        # Ensure we've consumed all tokens
        current = self.current_token()
        if current and current.type != TokenType.EOF:
            raise ParseError(
                f"Unexpected token after document: {current.value}", current
            )

        return ParsedDocument(data=root_value, comments=self.comments)


def parse(
    text: str,
    *,
    preset: str | None = None,
    options: ParseOptions | None = None,
    **kwargs: Unpack[ParseOptionsKwargs],
) -> ParsedDocument:
    """Parse JYAML text and return ParsedDocument.

    Args:
        text: JYAML text to parse
        preset: Preset name ('strict', 'permissive', 'fast', 'debug')
        options: Custom ParseOptions (overrides preset)
        **kwargs: Direct option overrides (strict_mode=False, preserve_comments=True, etc.)

    Returns:
        ParsedDocument with data and metadata

    Examples:
        # Simple usage
        doc = parse('{"key": "value"}')

        # Using preset
        doc = parse(text, preset='debug')

        # Quick options
        doc = parse(text, strict_mode=False, max_depth=50)

        # Custom options
        opts = ParseOptions(preserve_comments=False)
        doc = parse(text, options=opts)
    """

    # Determine options to use
    if options is not None:
        # Use provided options
        parse_opts = options
    elif preset is not None:
        # Use preset
        parse_opts = ParseOptions.from_preset(preset)
    elif kwargs:
        # Create options from kwargs
        parse_opts = ParseOptions(**kwargs)
    else:
        # Use defaults
        parse_opts = ParseOptions()

    parser = Parser(text, parse_opts)
    return parser.parse()
