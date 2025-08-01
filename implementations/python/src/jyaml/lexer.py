"""JYAML lexer implementation."""

from collections.abc import Iterator
from enum import Enum

from pydantic import BaseModel, Field


class TokenType(Enum):
    """Token types for JYAML lexer."""

    # Literals
    NULL = "null"
    TRUE = "true"
    FALSE = "false"
    NUMBER = "number"
    STRING = "string"

    # Punctuation
    COLON = ":"
    COMMA = ","
    LEFT_BRACKET = "["
    RIGHT_BRACKET = "]"
    LEFT_BRACE = "{"
    RIGHT_BRACE = "}"
    DASH = "-"
    PIPE = "|"
    PIPE_STRIP = "|-"
    GREATER = ">"
    GREATER_STRIP = ">-"

    # Special
    NEWLINE = "newline"
    INDENT = "indent"
    COMMENT = "comment"
    EOF = "eof"


class Token(BaseModel):
    """Token representation."""

    type: TokenType
    value: str
    line: int = Field(ge=1)
    column: int = Field(ge=1)


class LexerError(Exception):
    """Lexer error with position information."""

    def __init__(self, message: str, line: int, column: int):
        super().__init__(f"{message} at line {line}, column {column}")
        self.message = message
        self.line = line
        self.column = column


class Lexer:
    """JYAML lexer."""

    def __init__(self, text: str):
        """Initialize lexer with input text."""
        # Check for BOM
        if text.startswith("\ufeff"):
            raise LexerError("BOM not allowed", 1, 1)

        self.text = text
        self.position = 0
        self.line = 1
        self.column = 1
        self.at_line_start = True

    def current_char(self) -> str | None:
        """Get current character."""
        if self.position >= len(self.text):
            return None
        return self.text[self.position]

    def peek_char(self, offset: int = 1) -> str | None:
        """Peek at character at offset from current position."""
        pos = self.position + offset
        if pos >= len(self.text):
            return None
        return self.text[pos]

    def advance(self) -> str | None:
        """Advance to next character and return current."""
        if self.position >= len(self.text):
            return None

        char = self.text[self.position]
        self.position += 1

        if char == "\n":
            self.line += 1
            self.column = 1
            self.at_line_start = True
        else:
            self.column += 1

        return char

    def skip_whitespace(self) -> None:
        """Skip whitespace characters except newlines."""
        while True:
            char = self.current_char()
            if not (char and char in " \t\r"):
                break
            if char == "\t":
                raise LexerError("Tab character in indentation", self.line, self.column)
            self.advance()

    def count_indent(self) -> int:
        """Count indentation at start of line."""
        indent = 0
        while self.current_char() == " ":
            indent += 1
            self.advance()

        # Check for tab after spaces
        if self.current_char() == "\t":
            raise LexerError("Tab character in indentation", self.line, self.column)

        return indent

    def read_string(self, quote: str) -> str:
        """Read quoted string."""
        value = ""
        self.advance()  # Skip opening quote

        while self.current_char() and self.current_char() != quote:
            char = self.current_char()
            if char == "\\":
                self.advance()
                escaped = self.current_char()
                if escaped is None:
                    raise LexerError("Unterminated string", self.line, self.column)

                escape_map = {
                    '"': '"',
                    "'": "'",
                    "\\": "\\",
                    "/": "/",
                    "b": "\b",
                    "f": "\f",
                    "n": "\n",
                    "r": "\r",
                    "t": "\t",
                }

                if escaped in escape_map:
                    value += escape_map[escaped]
                elif escaped == "u":
                    # Unicode escape sequence
                    self.advance()
                    hex_digits = ""
                    for _ in range(4):
                        digit = self.current_char()
                        if digit is None or digit not in "0123456789abcdefABCDEF":
                            raise LexerError(
                                "Invalid unicode escape", self.line, self.column
                            )
                        hex_digits += digit
                        self.advance()

                    code_point = int(hex_digits, 16)

                    # Check if this is a high surrogate (D800-DBFF)
                    if 0xD800 <= code_point <= 0xDBFF:
                        # This is a high surrogate, we need to read the low surrogate
                        low_surrogate = self._read_low_surrogate()
                        # Convert surrogate pair to Unicode code point
                        combined_code_point = (
                            0x10000
                            + ((code_point - 0xD800) << 10)
                            + (low_surrogate - 0xDC00)
                        )
                        value += chr(combined_code_point)
                    elif 0xDC00 <= code_point <= 0xDFFF:
                        # This is a low surrogate without a high surrogate
                        raise LexerError(
                            f"Unexpected low surrogate U+{code_point:04X}",
                            self.line,
                            self.column,
                        )
                    else:
                        # Regular Unicode character
                        value += chr(code_point)
                    continue
                else:
                    raise LexerError(
                        f"Invalid escape sequence: \\{escaped}", self.line, self.column
                    )
            else:
                if char is not None:
                    value += char
            self.advance()

        if self.current_char() != quote:
            raise LexerError("Unterminated string", self.line, self.column)

        self.advance()  # Skip closing quote
        return value

    def read_multiline_string(self, indicator: str) -> str:
        """Read multiline string with | or > indicator."""
        self.advance()  # Skip indicator

        # Check for chomping indicators
        strip_final = False
        keep_final = False
        if self.current_char() == "-":
            strip_final = True
            self.advance()
        elif self.current_char() == "+":
            keep_final = True
            self.advance()

        # Skip to newline
        self.skip_whitespace()
        if self.current_char() != "\n":
            raise LexerError(
                "Multiline string must start on new line", self.line, self.column
            )
        self.advance()

        lines = []
        base_indent = None

        while self.current_char() is not None:
            # Count indentation
            line_start_pos = self.position
            indent = self.count_indent()

            # Empty line or end of multiline string
            if self.current_char() in ["\n", None] or indent == 0:
                if self.current_char() == "\n":
                    lines.append("")
                    self.advance()
                    continue
                else:
                    # Reset position if we went too far
                    self.position = line_start_pos
                    break

            # Set base indentation from first non-empty line
            if base_indent is None:
                base_indent = indent
            elif indent < base_indent:
                # Less indentation means end of multiline string
                self.position = line_start_pos
                break

            # Read line content
            line_content = ""
            while self.current_char() and self.current_char() != "\n":
                char = self.current_char()
                if char is not None:
                    line_content += char
                self.advance()

            lines.append(line_content)

            if self.current_char() == "\n":
                self.advance()

        # Process lines based on indicator
        if indicator == "|":
            # Literal style - preserve line breaks
            result = "\n".join(lines)
        else:  # indicator == '>'
            # Folded style - fold line breaks to spaces
            result = " ".join(line.strip() for line in lines if line.strip())

        # Apply chomping indicators
        if strip_final:
            result = result.rstrip("\n")
        elif keep_final:
            # Keep final newlines - add extra newline if not present
            if result and not result.endswith("\n"):
                result += "\n"

        return result

    def read_number(self) -> str:
        """Read number literal."""
        value = ""

        # Handle negative sign
        if self.current_char() == "-":
            char = self.advance()
            if char is not None:
                value += char

        # Read integer part
        current = self.current_char()
        if current == "0":
            char = self.advance()
            if char is not None:
                value += char
        elif current and current.isdigit():
            while True:
                char = self.current_char()
                if not (char and char.isdigit()):
                    break
                char = self.advance()
                if char is not None:
                    value += char
        else:
            raise LexerError("Invalid number format", self.line, self.column)

        # Read decimal part
        current = self.current_char()
        if current == ".":
            char = self.advance()
            if char is not None:
                value += char
            current = self.current_char()
            if not (current and current.isdigit()):
                raise LexerError("Invalid number format", self.line, self.column)
            while True:
                char = self.current_char()
                if not (char and char.isdigit()):
                    break
                char = self.advance()
                if char is not None:
                    value += char

        # Read exponent part
        current = self.current_char()
        if current and current.lower() == "e":
            char = self.advance()
            if char is not None:
                value += char
            current = self.current_char()
            if current in ["+", "-"]:
                char = self.advance()
                if char is not None:
                    value += char
            current = self.current_char()
            if not (current and current.isdigit()):
                raise LexerError("Invalid number format", self.line, self.column)
            while True:
                char = self.current_char()
                if not (char and char.isdigit()):
                    break
                char = self.advance()
                if char is not None:
                    value += char

        return value

    def _read_low_surrogate(self) -> int:
        """Read the low surrogate part of a surrogate pair."""
        # Expect "\u" for the low surrogate
        if self.current_char() != "\\":
            raise LexerError("Expected '\\' for low surrogate", self.line, self.column)
        self.advance()

        if self.current_char() != "u":
            raise LexerError("Expected 'u' for low surrogate", self.line, self.column)
        self.advance()

        # Read the low surrogate hex digits
        hex_digits = ""
        for _ in range(4):
            digit = self.current_char()
            if digit is None or digit not in "0123456789abcdefABCDEF":
                raise LexerError(
                    "Invalid unicode escape in low surrogate", self.line, self.column
                )
            hex_digits += digit
            self.advance()

        low_code = int(hex_digits, 16)

        # Validate that it's actually a low surrogate
        if not (0xDC00 <= low_code <= 0xDFFF):
            raise LexerError(
                f"Expected low surrogate (DC00-DFFF), got U+{low_code:04X}",
                self.line,
                self.column,
            )

        return low_code

    def read_comment(self) -> str:
        """Read comment."""
        self.advance()  # Skip #
        comment = ""
        while self.current_char() and self.current_char() != "\n":
            char = self.advance()
            if char is not None:
                comment += char
        return comment.strip()

    def read_identifier(self) -> str:
        """Read identifier (for true, false, null)."""
        value = ""
        while True:
            char = self.current_char()
            if not (char and (char.isalnum() or char == "_")):
                break
            char = self.advance()
            if char is not None:
                value += char
        return value

    def next_token(self) -> Token:
        """Get next token."""
        # Handle indentation at line start
        if self.at_line_start:
            self.at_line_start = False
            indent = self.count_indent()
            if indent > 0:
                return Token(
                    type=TokenType.INDENT,
                    value=str(indent),
                    line=self.line,
                    column=self.column - indent,
                )

        self.skip_whitespace()

        if self.current_char() is None:
            return Token(
                type=TokenType.EOF, value="", line=self.line, column=self.column
            )

        char = self.current_char()
        line, column = self.line, self.column

        # Newline
        if char == "\n":
            self.advance()
            return Token(type=TokenType.NEWLINE, value=char, line=line, column=column)

        # Comment
        if char == "#":
            comment = self.read_comment()
            return Token(
                type=TokenType.COMMENT, value=comment, line=line, column=column
            )

        # String literals
        if char in ['"', "'"]:
            value = self.read_string(char)
            return Token(type=TokenType.STRING, value=value, line=line, column=column)

        # Multiline string indicators
        if char == "|":
            if self.peek_char() == "-":
                value = self.read_multiline_string("|-")
                return Token(
                    type=TokenType.STRING, value=value, line=line, column=column
                )
            else:
                value = self.read_multiline_string("|")
                return Token(
                    type=TokenType.STRING, value=value, line=line, column=column
                )

        if char == ">":
            if self.peek_char() == "-":
                value = self.read_multiline_string(">-")
                return Token(
                    type=TokenType.STRING, value=value, line=line, column=column
                )
            else:
                value = self.read_multiline_string(">")
                return Token(
                    type=TokenType.STRING, value=value, line=line, column=column
                )

        # Numbers
        peek = self.peek_char()
        if (char and char.isdigit()) or (char == "-" and peek and peek.isdigit()):
            value = self.read_number()
            return Token(type=TokenType.NUMBER, value=value, line=line, column=column)

        # Identifiers (true, false, null)
        if char and char.isalpha():
            value = self.read_identifier()
            if value == "true":
                return Token(type=TokenType.TRUE, value=value, line=line, column=column)
            elif value == "false":
                return Token(
                    type=TokenType.FALSE, value=value, line=line, column=column
                )
            elif value == "null":
                return Token(type=TokenType.NULL, value=value, line=line, column=column)
            else:
                raise LexerError(f"Unknown identifier: {value}", line, column)

        # Single character tokens
        single_char_tokens = {
            ":": TokenType.COLON,
            ",": TokenType.COMMA,
            "[": TokenType.LEFT_BRACKET,
            "]": TokenType.RIGHT_BRACKET,
            "{": TokenType.LEFT_BRACE,
            "}": TokenType.RIGHT_BRACE,
            "-": TokenType.DASH,
        }

        if char in single_char_tokens:
            self.advance()
            return Token(
                type=single_char_tokens[char], value=char, line=line, column=column
            )

        raise LexerError(f"Unexpected character: {char}", line, column)

    def tokenize(self) -> Iterator[Token]:
        """Tokenize entire input."""
        while True:
            token = self.next_token()
            yield token
            if token.type == TokenType.EOF:
                break
