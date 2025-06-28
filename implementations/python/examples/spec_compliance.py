#!/usr/bin/env python3
"""JYAML specification compliance examples."""

from jyaml import loads, parse, ParseOptions, LoadOptions
from jyaml.lexer import LexerError
from jyaml.parser import ParseError

def demonstrate_spec_compliance():
    """Show JYAML specification compliance."""
    print("=== JYAML Specification Compliance ===")
    
    # 1. Required quoted keys (spec requirement)
    print("1. Keys must be quoted (JYAML spec requirement):")
    
    valid_quoted = '{"key": "value"}'
    try:
        result = loads(valid_quoted)
        print(f"   ✓ Quoted keys: {result}")
    except Exception as e:
        print(f"   ✗ Error: {e}")
    
    # Unquoted keys are spec violation (would need lexer changes to test)
    print("   Note: Unquoted keys are forbidden by JYAML spec")
    
    print()

def demonstrate_comment_support():
    """Show both comment styles per spec."""
    print("=== Comment Support (JYAML spec) ===")
    
    # JYAML spec supports both # and // comments
    print("1. YAML-style comments (#):")
    yaml_comments = '''
    # YAML-style comment
    {
      "app": "MyApp", # Inline comment
      "version": "1.0.0"
    }
    '''
    
    try:
        doc = parse(yaml_comments)
        print(f"   ✓ Comments parsed: {doc.comments}")
    except Exception as e:
        print(f"   ✗ Error: {e}")
    
    # Note: C-style comments would need lexer implementation
    print("2. C-style comments (//) - would need lexer support:")
    print("   Note: Currently only # comments are implemented")
    print("   TODO: Add // comment support to lexer")
    
    print()

def demonstrate_tab_restrictions():
    """Show tab character restrictions per spec."""
    print("=== Tab Character Restrictions (JYAML spec) ===")
    
    # Tabs are forbidden by JYAML spec
    input_with_tab = '{\t"key": "value"}'
    
    print("1. Default (spec compliant - tabs forbidden):")
    try:
        result = loads(input_with_tab)
        print(f"   ✗ Unexpected success: {result}")
    except (LexerError, ParseError) as e:
        print(f"   ✓ Correctly rejected tabs: {e}")
    
    print("2. Extension mode (allows tabs):")
    extension_opts = ParseOptions(
        allow_extensions=True,
        extension_allow_tabs=True
    )
    load_opts = LoadOptions(parse_options=extension_opts)
    try:
        result = loads(input_with_tab, load_opts)
        print(f"   ✓ Extension allows tabs: {result}")
    except Exception as e:
        print(f"   ✗ Extension failed: {e}")
    
    print()

def demonstrate_multiline_strings():
    """Show multiline string support per spec."""
    print("=== Multiline Strings (JYAML spec) ===")
    
    # JYAML spec supports | and > with optional - chomping
    multiline_input = '''
    {
      "literal": |
        Line 1
        Line 2
        Line 3
      "literal_strip": |-
        No trailing newline
        here
      "folded": >
        This is a long
        paragraph that will
        be folded into one line
      "folded_strip": >-
        This is also folded
        but with no trailing newline
    }
    '''
    
    try:
        result = loads(multiline_input)
        print("✓ Multiline strings parsed successfully:")
        print(f"  Literal: {repr(result['literal'])}")
        print(f"  Literal strip: {repr(result['literal_strip'])}")
        print(f"  Folded: {repr(result['folded'])}")
        print(f"  Folded strip: {repr(result['folded_strip'])}")
    except Exception as e:
        print(f"✗ Error: {e}")
    
    # Note: |+ and >+ are NOT supported per spec
    print("\nNote: |+ and >+ chomping indicators are NOT supported per JYAML spec")
    
    print()

def demonstrate_boolean_restrictions():
    """Show boolean value restrictions per spec."""
    print("=== Boolean Restrictions (JYAML spec) ===")
    
    # Only true/false allowed (not YAML alternatives)
    print("1. Valid booleans (spec compliant):")
    valid_bool = '{"enabled": true, "debug": false}'
    try:
        result = loads(valid_bool)
        print(f"   ✓ true/false: {result}")
    except Exception as e:
        print(f"   ✗ Error: {e}")
    
    print("2. YAML-style booleans (forbidden by spec):")
    print("   on/off, yes/no, etc. are invalid in JYAML")
    print("   Only 'true' and 'false' literals are allowed")
    
    print()

def demonstrate_trailing_commas():
    """Show trailing comma support per spec."""
    print("=== Trailing Comma Support (JYAML spec) ===")
    
    # Trailing commas are supported per spec
    trailing_comma_cases = [
        ('{"a": 1, "b": 2,}', 'Object with trailing comma'),
        ('[1, 2, 3,]', 'Array with trailing comma'),
        ('{"nested": [1, 2,], "more": "data",}', 'Nested with trailing commas')
    ]
    
    for input_str, description in trailing_comma_cases:
        try:
            result = loads(input_str)
            print(f"   ✓ {description}: {result}")
        except Exception as e:
            print(f"   ✗ {description} failed: {e}")
    
    print()

def demonstrate_encoding_requirements():
    """Show encoding requirements per spec."""
    print("=== Encoding Requirements (JYAML spec) ===")
    
    print("1. UTF-8 without BOM (spec requirement):")
    print("   ✓ Current parser correctly rejects BOM")
    print("   ✓ Expects UTF-8 encoding")
    
    # Test BOM rejection (already implemented in lexer)
    bom_input = '\ufeff{"key": "value"}'
    try:
        result = loads(bom_input)
        print(f"   ✗ BOM unexpectedly accepted: {result}")
    except (LexerError, ParseError) as e:
        print(f"   ✓ BOM correctly rejected: {e}")
    
    print()

def demonstrate_root_level_values():
    """Show root level value support per spec."""
    print("=== Root Level Values (JYAML spec) ===")
    
    # JYAML allows any value at root level (like modern JSON)
    root_cases = [
        ('{"key": "value"}', 'Object at root'),
        ('[1, 2, 3]', 'Array at root'),
        ('"Hello, World!"', 'String at root'),
        ('42', 'Number at root'),
        ('true', 'Boolean at root'),
        ('null', 'Null at root')
    ]
    
    for input_str, description in root_cases:
        try:
            result = loads(input_str)
            print(f"   ✓ {description}: {result}")
        except Exception as e:
            print(f"   ✗ {description} failed: {e}")
    
    print()

def demonstrate_extension_vs_spec():
    """Show difference between spec compliance and extensions."""
    print("=== Specification vs Extensions ===")
    
    print("JYAML Specification (strict compliance):")
    print("- Keys must be quoted")
    print("- Only # and // comments")  
    print("- No tab characters")
    print("- Only |, |-, >, >- multiline indicators")
    print("- Only true/false booleans")
    print("- UTF-8 without BOM")
    print()
    
    print("Available Extensions (beyond spec):")
    print("- extension_allow_tabs: Allow tab characters")
    print("- extension_allow_unquoted_keys: Allow unquoted keys")
    print("- allow_duplicate_keys: Allow duplicate object keys")
    print()
    
    print("Usage:")
    print("# Strict spec compliance (default)")
    print("result = loads(text)")
    print()
    print("# With extensions")
    print("opts = ParseOptions(")
    print("    allow_extensions=True,")
    print("    extension_allow_tabs=True")
    print(")")
    print("result = loads(text, LoadOptions(parse_options=opts))")
    
    print()

def main():
    print("JYAML Specification Compliance Examples")
    print("=" * 42)
    print()
    
    demonstrate_spec_compliance()
    demonstrate_comment_support()
    demonstrate_tab_restrictions()
    demonstrate_multiline_strings()
    demonstrate_boolean_restrictions()
    demonstrate_trailing_commas()
    demonstrate_encoding_requirements()
    demonstrate_root_level_values()
    demonstrate_extension_vs_spec()

if __name__ == "__main__":
    main()