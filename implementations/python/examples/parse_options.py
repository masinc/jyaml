#!/usr/bin/env python3
"""Parse options examples for JYAML."""

from jyaml import loads, parse, ParseOptions
from jyaml.lexer import LexerError
from jyaml.parser import ParseError

def demonstrate_strict_mode():
    """Show strict vs non-strict parsing."""
    print("=== Strict vs Non-Strict Mode ===")
    
    # Input with tab character (normally forbidden)
    input_with_tab = '{\t"key": "value"}'
    
    # Strict mode (default)
    print("1. Strict mode (default):")
    try:
        result = loads(input_with_tab)
        print(f"   ✗ Unexpected success: {result}")
    except (LexerError, ParseError) as e:
        print(f"   ✓ Expected error: {e}")
    
    # Non-strict mode with tabs allowed
    print("2. Non-strict mode with tabs allowed:")
    options = ParseOptions(
        strict_syntax=False,
        allow_tabs=True
    )
    try:
        result = loads(input_with_tab, options)
        print(f"   ✓ Success: {result}")
    except (LexerError, ParseError) as e:
        print(f"   ✗ Unexpected error: {e}")
    
    print()

def demonstrate_comment_handling():
    """Show different comment handling options."""
    print("=== Comment Handling Options ===")
    
    commented_input = '''
    # Top-level comment
    {
      "app": "MyApp", # Inline comment
      "version": "1.0.0"
    }
    '''
    
    # Default: preserve comments
    print("1. Default (preserve comments):")
    doc = parse(commented_input)
    print(f"   Comments: {doc.comments}")
    
    # Don't preserve comments
    print("2. Don't preserve comments:")
    options = ParseOptions(preserve_comments=False)
    doc = parse(commented_input, options)
    print(f"   Comments: {doc.comments}")
    
    # Include comment positions
    print("3. Include comment positions:")
    options = ParseOptions(include_comment_positions=True)
    try:
        doc = parse(commented_input, options)
        print(f"   Comments with positions available in parser")
    except Exception as e:
        print(f"   Note: Feature needs implementation: {e}")
    
    print()

def demonstrate_depth_limits():
    """Show maximum depth limiting."""
    print("=== Depth Limiting ===")
    
    # Create deeply nested structure
    nested_json = "{}"
    for i in range(10):
        nested_json = f'{{"level{i}": {nested_json}}}'
    
    # Default depth limit (1000)
    print("1. Default depth limit (1000):")
    try:
        result = loads(nested_json)
        print(f"   ✓ Success: 10 levels parsed")
    except ParseError as e:
        print(f"   ✗ Error: {e}")
    
    # Low depth limit
    print("2. Low depth limit (5):")
    options = ParseOptions(max_depth=5)
    try:
        result = loads(nested_json, options)
        print(f"   ✗ Unexpected success")
    except ParseError as e:
        print(f"   ✓ Expected depth error: {e}")
    
    print()

def demonstrate_token_limits():
    """Show token count limiting."""
    print("=== Token Count Limiting ===")
    
    # Large array
    large_array = "[" + ", ".join(str(i) for i in range(100)) + "]"
    
    # Default (no limit)
    print("1. Default (no token limit):")
    try:
        result = loads(large_array)
        print(f"   ✓ Success: {len(result)} items")
    except ParseError as e:
        print(f"   ✗ Error: {e}")
    
    # Low token limit
    print("2. Low token limit (50):")
    options = ParseOptions(max_tokens=50)
    try:
        result = loads(large_array, options)
        print(f"   ✗ Unexpected success")
    except ParseError as e:
        print(f"   ✓ Expected token limit error: {e}")
    
    print()

def demonstrate_normalization():
    """Show line ending normalization."""
    print("=== Line Ending Normalization ===")
    
    # Windows line endings
    windows_input = '{\r\n  "key": "value"\r\n}'
    
    # With normalization (default)
    print("1. With normalization (default):")
    try:
        result = loads(windows_input)
        print(f"   ✓ Success: {result}")
    except Exception as e:
        print(f"   ✗ Error: {e}")
    
    # Without normalization
    print("2. Without normalization:")
    options = ParseOptions(normalize_line_endings=False)
    try:
        result = loads(windows_input, options)
        print(f"   ✓ Success: {result}")
    except Exception as e:
        print(f"   ✗ Error: {e}")
    
    print()

def demonstrate_type_parsing():
    """Show type parsing options."""
    print("=== Type Parsing Options ===")
    
    # Input with various types
    type_input = '''
    {
      "number": 42,
      "boolean": true,
      "null_value": null,
      "string": "hello"
    }
    '''
    
    # Default type parsing
    print("1. Default type parsing:")
    result = loads(type_input)
    print(f"   Number type: {type(result['number'])} = {result['number']}")
    print(f"   Boolean type: {type(result['boolean'])} = {result['boolean']}")
    print(f"   Null type: {type(result['null_value'])} = {result['null_value']}")
    
    # All types as strings (hypothetical - would need lexer changes)
    print("2. Note: String-only parsing would require lexer modifications")
    
    print()

def demonstrate_multiline_options():
    """Show multiline string handling options."""
    print("=== Multiline String Options ===")
    
    multiline_input = '''
    {
      "literal": |
        Line 1
        Line 2
        Line 3
      "folded": >
        This is a long
        paragraph that should
        be folded into one line
    }
    '''
    
    # Default multiline handling
    print("1. Default multiline handling:")
    result = loads(multiline_input)
    print(f"   Literal: {repr(result['literal'])}")
    print(f"   Folded: {repr(result['folded'])}")
    
    # Custom multiline options (demonstration of concept)
    print("2. Custom options available via ParseOptions")
    options = ParseOptions(
        preserve_literal_newlines=True,
        fold_scalar_newlines=True
    )
    result = loads(multiline_input, options)
    print(f"   Same result with explicit options")
    
    print()

def demonstrate_permissive_parsing():
    """Show permissive parsing options."""
    print("=== Permissive Parsing ===")
    
    # This would require implementation of unquoted keys and duplicate key handling
    print("Note: Advanced permissive features like unquoted keys and")
    print("duplicate key handling would require additional implementation.")
    
    # Show current capabilities
    trailing_comma_input = '{"valid": true, "array": [1, 2, 3,]}'
    
    print("Current permissive feature - trailing commas:")
    try:
        result = loads(trailing_comma_input)
        print(f"   ✓ Success with trailing comma: {result}")
    except Exception as e:
        print(f"   ✗ Error: {e}")
    
    print()

def main():
    print("JYAML Parse Options Examples")
    print("=" * 30)
    print()
    
    demonstrate_strict_mode()
    demonstrate_comment_handling()
    demonstrate_depth_limits()
    demonstrate_token_limits()
    demonstrate_normalization()
    demonstrate_type_parsing()
    demonstrate_multiline_options()
    demonstrate_permissive_parsing()
    
    print("Summary of ParseOptions:")
    print("- strict_syntax: Enable/disable strict syntax checking")
    print("- allow_tabs: Allow tab characters in indentation")
    print("- preserve_comments: Include comments in parsed document")
    print("- max_depth: Limit maximum nesting depth")
    print("- max_tokens: Limit maximum number of tokens")
    print("- normalize_line_endings: Convert \\r\\n to \\n")
    print("- And more options for fine-tuned parsing control")

if __name__ == "__main__":
    main()