"""JYAML parser implementation."""

from typing import List, Optional, Any, Dict, Union, Callable, Literal
from pydantic import BaseModel, Field, ConfigDict, field_validator, model_validator
from .lexer import Lexer, Token, TokenType, LexerError
from .types import (
    JYAMLData, JYAMLNull, JYAMLBool, JYAMLNumber, JYAMLString, 
    JYAMLArray, JYAMLObject, ParsedDocument
)


# Predefined option presets for common use cases
class JYAMLMode:
    """Predefined JYAML parsing modes for common use cases."""
    
    @staticmethod
    def strict() -> 'ParseOptions':
        """Strict JYAML spec compliance (default)."""
        return ParseOptions(
            strict_mode=True,
            preserve_comments=True,
            max_depth=1000
        )
    
    @staticmethod
    def permissive() -> 'ParseOptions':
        """Permissive mode for flexibility."""
        return ParseOptions(
            strict_mode=False,
            preserve_comments=True,
            allow_duplicate_keys=True,
            max_depth=10000
        )
    
    @staticmethod
    def fast() -> 'ParseOptions':
        """Fast parsing mode (minimal features)."""
        return ParseOptions(
            strict_mode=True,
            preserve_comments=False,
            max_depth=100
        )
    
    @staticmethod
    def debug() -> 'ParseOptions':
        """Debug mode with detailed information."""
        return ParseOptions(
            strict_mode=False,
            preserve_comments=True,
            include_comment_positions=True,
            allow_duplicate_keys=True
        )


class ParseOptions(BaseModel):
    """Simple, user-friendly JYAML parsing options with Pydantic validation."""
    
    model_config = ConfigDict(
        extra='forbid',
        str_strip_whitespace=True,
        validate_assignment=True,
        frozen=False
    )
    
    # Main mode settings
    strict_mode: bool = Field(
        default=True, 
        description="Strict JYAML spec compliance"
    )
    preserve_comments: bool = Field(
        default=True, 
        description="Keep comments in parsed document"
    )
    
    # Common flexibility options
    allow_duplicate_keys: bool = Field(
        default=False, 
        description="Allow duplicate object keys"
    )
    
    # Performance limits
    max_depth: Optional[int] = Field(
        default=1000, 
        gt=0, 
        le=100000,
        description="Maximum nesting depth (None = unlimited)"
    )
    
    # Advanced options (for power users)
    include_comment_positions: bool = Field(
        default=False, 
        description="Include line/column info for comments"
    )
    normalize_line_endings: Literal["none", "lf", "crlf"] = Field(
        default="lf", 
        description="Line ending normalization: 'none', 'lf' (\\n), or 'crlf' (\\r\\n)"
    )
    
    
    @field_validator('max_depth')
    @classmethod
    def validate_max_depth(cls, v):
        """Validate max_depth is reasonable."""
        if v is not None and v <= 0:
            raise ValueError('max_depth must be positive')
        return v
    
    
    @model_validator(mode='after')
    def validate_consistency(self):
        """Validate option consistency."""
        if self.strict_mode and self.allow_duplicate_keys:
            raise ValueError('strict_mode and allow_duplicate_keys are incompatible')
        
        if self.include_comment_positions and not self.preserve_comments:
            raise ValueError('include_comment_positions requires preserve_comments=True')
        
        return self
    
    # Create from preset
    @classmethod
    def from_preset(cls, preset: str) -> 'ParseOptions':
        """Create options from preset name."""
        presets = {
            'strict': JYAMLMode.strict(),
            'permissive': JYAMLMode.permissive(), 
            'fast': JYAMLMode.fast(),
            'debug': JYAMLMode.debug()
        }
        if preset not in presets:
            available = ', '.join(presets.keys())
            raise ValueError(f"Unknown preset: {preset}. Available: {available}")
        return presets[preset]


class LoadOptions(BaseModel):
    """Simple, user-friendly JYAML loading options with Pydantic validation."""
    
    model_config = ConfigDict(
        extra='forbid',
        str_strip_whitespace=True,
        validate_assignment=True,
        frozen=False,
        arbitrary_types_allowed=True  # Allow callable types
    )
    
    # Main conversion options
    as_dict: bool = Field(
        default=True, 
        description="Convert objects to dict (vs OrderedDict/custom)"
    )
    as_native_types: bool = Field(
        default=True, 
        description="Convert to Python native types"
    )
    
    # Type conversion control
    parse_numbers: bool = Field(
        default=True, 
        description="Convert numeric strings to int/float"
    )
    parse_booleans: bool = Field(
        default=True, 
        description="Convert 'true'/'false' to bool"
    )
    parse_null: bool = Field(
        default=True, 
        description="Convert 'null' to None"
    )
    
    # Advanced type options
    use_decimal: bool = Field(
        default=False, 
        description="Use Decimal instead of float for precision"
    )
    use_ordered_dict: bool = Field(
        default=False, 
        description="Use OrderedDict to preserve key order"
    )
    
    # Custom conversion hooks
    object_hook: Optional[Callable] = Field(
        default=None, 
        description="Custom object creation function"
    )
    number_hook: Optional[Callable] = Field(
        default=None, 
        description="Custom number parsing function"
    )
    
    # Include parsing options
    parse_options: Optional[ParseOptions] = Field(
        default=None, 
        description="Override default parse options"
    )
    
    @field_validator('object_hook', 'number_hook')
    @classmethod
    def validate_callable(cls, v):
        """Validate that hooks are callable."""
        if v is not None and not callable(v):
            raise ValueError('Hook must be callable')
        return v
    
    @model_validator(mode='after')
    def validate_consistency(self):
        """Validate option consistency."""
        if not self.as_native_types and (self.use_decimal or self.use_ordered_dict):
            raise ValueError('use_decimal and use_ordered_dict require as_native_types=True')
        
        if self.use_decimal and not self.parse_numbers:
            raise ValueError('use_decimal requires parse_numbers=True')
        
        if self.as_dict and self.use_ordered_dict:
            # as_dict=False implied when use_ordered_dict=True
            self.as_dict = False
        
        return self
    
    # Create from preset
    @classmethod
    def from_preset(cls, preset: str) -> 'LoadOptions':
        """Create options from preset name."""
        presets = {
            'default': cls(),
            'strict_types': cls(as_native_types=True, parse_numbers=True, parse_booleans=True),
            'preserve_order': cls(use_ordered_dict=True),
            'high_precision': cls(use_decimal=True, use_ordered_dict=True),
            'strings_only': cls(as_native_types=False, parse_numbers=False, parse_booleans=False, parse_null=False)
        }
        
        if preset not in presets:
            available = ', '.join(presets.keys())
            raise ValueError(f"Unknown preset: {preset}. Available: {available}")
        
        return presets[preset]


class ParseError(Exception):
    """Parser error with position information."""
    
    def __init__(self, message: str, token: Optional[Token] = None):
        if token:
            super().__init__(f"{message} at line {token.line}, column {token.column}")
            self.line = token.line
            self.column = token.column
        else:
            super().__init__(message)
            self.line = None
            self.column = None
        self.message = message


class Parser:
    """JYAML parser."""
    
    def __init__(self, text: str, options: Optional[ParseOptions] = None):
        """Initialize parser with input text and options."""
        self.options = options or ParseOptions()
        
        # Normalize line endings if requested
        if self.options.normalize_line_endings:
            text = text.replace('\r\n', '\n').replace('\r', '\n')
        
        self.lexer = Lexer(text)
        self.tokens: List[Token] = []
        self.position = 0
        self.comments: List[str] = []
        self.comment_positions: List[Dict[str, Any]] = []
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
                            self.comment_positions.append({
                                'text': token.value,
                                'line': token.line,
                                'column': token.column
                            })
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
    
    def current_token(self) -> Optional[Token]:
        """Get current token."""
        if self.position >= len(self.tokens):
            return None
        return self.tokens[self.position]
    
    def peek_token(self, offset: int = 1) -> Optional[Token]:
        """Peek at token at offset from current position."""
        pos = self.position + offset
        if pos >= len(self.tokens):
            return None
        return self.tokens[pos]
    
    def check_depth(self):
        """Check if max depth is exceeded."""
        if (self.options.max_depth is not None and 
            self.depth > self.options.max_depth):
            raise ParseError(f"Maximum nesting depth exceeded: {self.options.max_depth}")
    
    def enter_scope(self):
        """Enter a new parsing scope (array or object)."""
        self.depth += 1
        self.check_depth()
    
    def exit_scope(self):
        """Exit current parsing scope."""
        self.depth -= 1
    
    def advance(self) -> Optional[Token]:
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
        return self.advance()
    
    def skip_newlines(self):
        """Skip newline and indent tokens."""
        while (self.current_token() and 
               self.current_token().type in [TokenType.NEWLINE, TokenType.INDENT]):
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
                if '.' not in token.value and 'e' not in token.value.lower():
                    value = int(token.value)
                else:
                    value = float(token.value)
                return JYAMLNumber(value=value)
            except ValueError:
                raise ParseError(f"Invalid number: {token.value}", token)
        
        elif token.type == TokenType.STRING:
            # Check if it's a block object (key without bracket)
            if (self.peek_token() and self.peek_token().type == TokenType.COLON):
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
        
        elif token.type in [TokenType.PIPE, TokenType.PIPE_STRIP, TokenType.GREATER, TokenType.GREATER_STRIP]:
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
        
        items = []
        
        # Handle empty array
        if self.current_token() and self.current_token().type == TokenType.RIGHT_BRACKET:
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
                if (self.current_token() and 
                    self.current_token().type == TokenType.RIGHT_BRACKET):
                    self.advance()
                    break
            else:
                raise ParseError(f"Expected ',' or ']' in array, got {token.value}", token)
        
        self.exit_scope()
        return JYAMLArray(value=items)
    
    def parse_flow_object(self) -> JYAMLObject:
        """Parse flow-style object {"key": "value"}."""
        self.enter_scope()
        self.expect(TokenType.LEFT_BRACE)
        self.skip_newlines()
        
        items = {}
        
        # Handle empty object
        if self.current_token() and self.current_token().type == TokenType.RIGHT_BRACE:
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
                if (self.current_token() and 
                    self.current_token().type == TokenType.RIGHT_BRACE):
                    self.advance()
                    break
            elif token.type == TokenType.STRING and self.peek_token() and self.peek_token().type == TokenType.COLON:
                # Another key-value pair without comma (valid in flow style with newlines)
                continue
            else:
                raise ParseError(f"Expected ',' or '}}' in object, got {token.value}", token)
        
        self.exit_scope()
        return JYAMLObject(value=items)
    
    def parse_block_array(self) -> JYAMLArray:
        """Parse block-style array."""
        self.enter_scope()
        items = []
        base_indent = None
        
        while (self.current_token() and 
               self.current_token().type == TokenType.DASH):
            
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
            if not (self.current_token() and 
                   self.current_token().type == TokenType.DASH):
                break
        
        self.exit_scope()
        return JYAMLArray(value=items)
    
    def parse_block_object(self) -> JYAMLObject:
        """Parse block-style object."""
        self.enter_scope()
        items = {}
        
        while (self.current_token() and 
               self.current_token().type == TokenType.STRING and
               self.peek_token() and 
               self.peek_token().type == TokenType.COLON):
            
            # Parse key
            key_token = self.advance()
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
        
        if not self.current_token() or self.current_token().type == TokenType.EOF:
            # Empty document
            return ParsedDocument(
                data=JYAMLNull(),
                comments=self.comments
            )
        
        # Parse root value
        root_value = self.parse_value()
        
        # Skip any trailing newlines
        self.skip_newlines()
        
        # Ensure we've consumed all tokens
        if self.current_token() and self.current_token().type != TokenType.EOF:
            token = self.current_token()
            raise ParseError(f"Unexpected token after document: {token.value}", token)
        
        return ParsedDocument(
            data=root_value,
            comments=self.comments
        )


def parse(text: str, *,
          preset: Optional[str] = None,
          options: Optional[ParseOptions] = None,
          **kwargs) -> ParsedDocument:
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


# Convenience functions for common use cases
def loads_strict(text: str) -> Any:
    """Parse JYAML with strict compliance."""
    return loads(text, options=LoadOptions.from_preset('strict_types'))

def loads_permissive(text: str) -> Any:
    """Parse JYAML with permissive settings."""
    parse_opts = ParseOptions.from_preset('permissive')
    return loads(text, options=LoadOptions(parse_options=parse_opts))

def loads_fast(text: str) -> Any:
    """Parse JYAML with fast settings (no comments)."""
    parse_opts = ParseOptions.from_preset('fast')
    return loads(text, options=LoadOptions(parse_options=parse_opts))

def loads_ordered(text: str) -> Any:
    """Parse JYAML preserving key order."""
    return loads(text, options=LoadOptions.from_preset('preserve_order'))


def loads(text: str, *, 
          preset: Optional[str] = None,
          options: Optional[LoadOptions] = None,
          **kwargs) -> Any:
    """Parse JYAML text and return native Python data.
    
    Args:
        text: JYAML text to parse
        preset: Preset name ('default', 'strict_types', 'preserve_order', etc.)
        options: Custom LoadOptions (overrides preset)
        **kwargs: Direct option overrides (as_dict=True, parse_numbers=False, etc.)
    
    Returns:
        Parsed Python data
        
    Examples:
        # Simple usage
        data = loads('{"key": "value"}')
        
        # Using preset
        data = loads(text, preset='preserve_order')
        
        # Quick options
        data = loads(text, parse_numbers=False, use_ordered_dict=True)
        
        # Custom options
        opts = LoadOptions(use_decimal=True)
        data = loads(text, options=opts)
    """
    
    # Determine options to use
    if options is not None:
        # Use provided options
        load_opts = options
    elif preset is not None:
        # Use preset
        load_opts = LoadOptions.from_preset(preset)
    elif kwargs:
        # Create options from kwargs
        load_opts = LoadOptions(**kwargs)
    else:
        # Use defaults
        load_opts = LoadOptions()
    
    # Parse with ParseOptions
    if load_opts.parse_options:
        document = parse(text, options=load_opts.parse_options)
    else:
        document = parse(text)
    
    # Convert to Python with type conversion options
    return _convert_with_options(document.data, load_opts)


def _convert_with_options(value: 'JYAMLData', options: LoadOptions) -> Any:
    """Convert JYAML data to Python with LoadOptions."""
    from .types import (
        JYAMLNull, JYAMLBool, JYAMLNumber, JYAMLString, 
        JYAMLArray, JYAMLObject
    )
    from decimal import Decimal
    from collections import OrderedDict
    
    if isinstance(value, JYAMLNull):
        return None if options.parse_null else "null"
    
    elif isinstance(value, JYAMLBool):
        if options.parse_booleans and options.as_native_types:
            return value.value
        else:
            return "true" if value.value else "false"
    
    elif isinstance(value, JYAMLNumber):
        if options.parse_numbers and options.as_native_types:
            if options.number_hook:
                return options.number_hook(value.value)
            elif options.use_decimal and isinstance(value.value, float):
                return Decimal(str(value.value))
            else:
                return value.value
        else:
            return str(value.value)
    
    elif isinstance(value, JYAMLString):
        return value.value
    
    elif isinstance(value, JYAMLArray):
        return [_convert_with_options(item, options) for item in value.value]
    
    elif isinstance(value, JYAMLObject):
        # Convert key-value pairs
        pairs = [(key, _convert_with_options(val, options)) 
                for key, val in value.value.items()]
        
        # Choose container type
        if options.object_hook:
            return options.object_hook(pairs)
        elif options.use_ordered_dict:
            return OrderedDict(pairs)
        else:
            return dict(pairs)
    
    else:
        raise ValueError(f"Unknown JYAML type: {type(value)}")