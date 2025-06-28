#!/usr/bin/env python3
"""JYAML error handling examples."""

from jyaml import loads, parse
from jyaml.lexer import LexerError
from jyaml.parser import ParseError

def demonstrate_lexer_errors():
    """Show various lexer errors."""
    print("=== Lexer Errors ===")
    
    # Tab character error
    try:
        loads('{\t"key": "value"}')
    except (LexerError, ParseError) as e:
        print(f"Tab error: {e}")
    
    # Unterminated string
    try:
        loads('{"unterminated": "string')
    except (LexerError, ParseError) as e:
        print(f"Unterminated string: {e}")
    
    # Invalid escape sequence
    try:
        loads('{"invalid": "\\q"}')
    except (LexerError, ParseError) as e:
        print(f"Invalid escape: {e}")
    
    # Unknown identifier
    try:
        loads('{"value": undefined}')
    except (LexerError, ParseError) as e:
        print(f"Unknown identifier: {e}")
    
    print()

def demonstrate_parser_errors():
    """Show various parser errors."""
    print("=== Parser Errors ===")
    
    # Missing value
    try:
        loads('{"key": }')
    except ParseError as e:
        print(f"Missing value: {e}")
    
    # Mismatched brackets
    try:
        loads('[1, 2, 3}')
    except ParseError as e:
        print(f"Mismatched brackets: {e}")
    
    # Missing comma
    try:
        loads('{"a": 1 "b": 2}')
    except ParseError as e:
        print(f"Missing comma: {e}")
    
    print()

def demonstrate_graceful_handling():
    """Show how to handle errors gracefully."""
    print("=== Graceful Error Handling ===")
    
    test_inputs = [
        '{"valid": "json"}',
        '{"invalid": }',
        'null',
        '[1, 2, 3,]',  # Valid with trailing comma
        '{\t"tab": "error"}',
    ]
    
    for i, input_str in enumerate(test_inputs, 1):
        print(f"Test {i}: {input_str[:20]}...")
        try:
            result = loads(input_str)
            print(f"  ✓ Success: {result}")
        except (LexerError, ParseError) as e:
            print(f"  ✗ Error: {type(e).__name__}: {e}")
        except Exception as e:
            print(f"  ✗ Unexpected error: {type(e).__name__}: {e}")
        print()

def validate_input(input_str: str) -> bool:
    """Validate JYAML input without raising exceptions."""
    try:
        loads(input_str)
        return True
    except (LexerError, ParseError):
        return False

def main():
    demonstrate_lexer_errors()
    demonstrate_parser_errors()
    demonstrate_graceful_handling()
    
    # Validation example
    print("=== Input Validation ===")
    test_cases = [
        '{"valid": true}',
        '{"invalid": }',
        'null',
    ]
    
    for test in test_cases:
        is_valid = validate_input(test)
        print(f"{test[:20]}... → {'Valid' if is_valid else 'Invalid'}")

if __name__ == "__main__":
    main()