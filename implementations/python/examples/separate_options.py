#!/usr/bin/env python3
"""Demonstrate separate options for parse() and loads() functions."""

from collections import OrderedDict
from jyaml import parse, loads, ParseOptions, LoadOptions

def demonstrate_parse_options():
    """Show ParseOptions for parse() function."""
    print("=== ParseOptions for parse() function ===")
    
    commented_input = '''
    # Configuration file
    {
      "app": "MyApp", # Application name
      "version": "1.0.0"
    }
    '''
    
    # Default parsing - preserves comments
    print("1. Default parsing (preserves comments):")
    doc = parse(commented_input)
    print(f"   Comments: {doc.comments}")
    print(f"   Data keys: {list(doc.data.value.keys())}")
    
    # Parse without comments
    print("2. Parse without comments:")
    parse_opts = ParseOptions(preserve_comments=False)
    doc = parse(commented_input, parse_opts)
    print(f"   Comments: {doc.comments}")
    print(f"   Data keys: {list(doc.data.value.keys())}")
    
    # Parse with depth limit
    print("3. Parse with depth limit:")
    nested_input = '{"level1": {"level2": {"level3": "deep"}}}'
    parse_opts = ParseOptions(max_depth=2)
    try:
        doc = parse(nested_input, parse_opts)
        print("   ✗ Unexpected success")
    except Exception as e:
        print(f"   ✓ Expected depth error: {e}")
    
    print()

def demonstrate_load_options():
    """Show LoadOptions for loads() function."""
    print("=== LoadOptions for loads() function ===")
    
    typed_input = '''
    {
      "name": "MyApp",
      "version": 1.0,
      "enabled": true,
      "config": null
    }
    '''
    
    # Default loading - parses types
    print("1. Default loading (parses types):")
    data = loads(typed_input)
    print(f"   name: {type(data['name'])} = {data['name']}")
    print(f"   version: {type(data['version'])} = {data['version']}")
    print(f"   enabled: {type(data['enabled'])} = {data['enabled']}")
    print(f"   config: {type(data['config'])} = {data['config']}")
    
    # Load as strings only
    print("2. Load with types as strings:")
    load_opts = LoadOptions(
        parse_numbers=False,
        parse_booleans=False,
        parse_null=False
    )
    data = loads(typed_input, load_opts)
    print(f"   name: {type(data['name'])} = {data['name']}")
    print(f"   version: {type(data['version'])} = {data['version']}")
    print(f"   enabled: {type(data['enabled'])} = {data['enabled']}")
    print(f"   config: {type(data['config'])} = {data['config']}")
    
    print()

def demonstrate_custom_converters():
    """Show custom type converters."""
    print("=== Custom Type Converters ===")
    
    numeric_input = '''
    {
      "integers": [1, 2, 3],
      "floats": [1.1, 2.2, 3.3],
      "mixed": 42.0
    }
    '''
    
    # Custom int converter (add 1000 to all integers)
    def custom_int(value):
        return value + 1000
    
    # Custom float converter (round to 1 decimal)
    def custom_float(value):
        return round(value, 1)
    
    print("1. Default conversion:")
    data = loads(numeric_input)
    print(f"   integers: {data['integers']}")
    print(f"   floats: {data['floats']}")
    print(f"   mixed: {data['mixed']}")
    
    print("2. Custom converters:")
    load_opts = LoadOptions(
        parse_int=custom_int,
        parse_float=custom_float
    )
    data = loads(numeric_input, load_opts)
    print(f"   integers: {data['integers']}")
    print(f"   floats: {data['floats']}")
    print(f"   mixed: {data['mixed']}")
    
    print()

def demonstrate_object_pairs_hook():
    """Show object_pairs_hook for maintaining order."""
    print("=== Object Pairs Hook ===")
    
    ordered_input = '''
    {
      "first": "A",
      "second": "B", 
      "third": "C",
      "fourth": "D"
    }
    '''
    
    print("1. Default dict (order may vary in older Python):")
    data = loads(ordered_input)
    print(f"   Keys: {list(data.keys())}")
    print(f"   Type: {type(data)}")
    
    print("2. Ordered dict using object_pairs_hook:")
    load_opts = LoadOptions(object_pairs_hook=OrderedDict)
    data = loads(ordered_input, load_opts)
    print(f"   Keys: {list(data.keys())}")
    print(f"   Type: {type(data)}")
    
    # Custom object hook
    class CustomObject:
        def __init__(self, pairs):
            self.data = dict(pairs)
            self.keys_order = [k for k, v in pairs]
        
        def __repr__(self):
            return f"CustomObject({self.keys_order})"
    
    print("3. Custom object class:")
    load_opts = LoadOptions(object_pairs_hook=CustomObject)
    data = loads(ordered_input, load_opts)
    print(f"   Object: {data}")
    print(f"   Keys order: {data.keys_order}")
    
    print()

def demonstrate_combined_options():
    """Show combining ParseOptions and LoadOptions."""
    print("=== Combined Parse and Load Options ===")
    
    complex_input = '''
    # Complex configuration
    {
      "app": {
        "name": "MyApp", # Application name
        "version": 2.5,
        "enabled": true
      },
      "database": {
        "host": "localhost",
        "port": 5432,
        "ssl": false
      }
    }
    '''
    
    # Define parse options
    parse_opts = ParseOptions(
        preserve_comments=True,
        normalize_line_endings=True
    )
    
    # Define load options with parse options included
    load_opts = LoadOptions(
        parse_options=parse_opts,
        parse_numbers=True,
        parse_booleans=True,
        object_pairs_hook=OrderedDict
    )
    
    print("1. Using combined options:")
    data = loads(complex_input, load_opts)
    print(f"   App name: {data['app']['name']}")
    print(f"   App version: {type(data['app']['version'])} = {data['app']['version']}")
    print(f"   DB port: {type(data['database']['port'])} = {data['database']['port']}")
    print(f"   Type: {type(data)}")
    
    # Also get the parsed document to see comments
    print("2. Parse document separately to see comments:")
    doc = parse(complex_input, parse_opts)
    print(f"   Comments: {doc.comments}")
    
    print()

def demonstrate_backward_compatibility():
    """Show that old code still works without options."""
    print("=== Backward Compatibility ===")
    
    simple_input = '{"message": "Hello, JYAML!", "count": 42}'
    
    print("1. Old-style usage (no options):")
    data = loads(simple_input)
    print(f"   Data: {data}")
    
    doc = parse(simple_input)
    print(f"   Comments: {doc.comments}")
    print(f"   Data type: {type(doc.data)}")
    
    print("2. Mixed usage (some with options, some without):")
    # Parse with options
    parse_opts = ParseOptions(preserve_comments=False)
    doc = parse(simple_input, parse_opts)
    
    # Load without options
    data = loads(simple_input)
    print(f"   Parse with options, load without: OK")
    
    print()

def main():
    print("JYAML Separate Options Examples")
    print("=" * 35)
    print()
    print("ParseOptions: Control parsing behavior")
    print("LoadOptions: Control type conversion and data structure")
    print()
    
    demonstrate_parse_options()
    demonstrate_load_options()
    demonstrate_custom_converters()
    demonstrate_object_pairs_hook()
    demonstrate_combined_options()
    demonstrate_backward_compatibility()
    
    print("Summary:")
    print("- ParseOptions: Comments, depth limits, syntax strictness")
    print("- LoadOptions: Type conversion, custom converters, object hooks")
    print("- LoadOptions can include ParseOptions for full control")
    print("- Backward compatibility maintained - options are optional")

if __name__ == "__main__":
    main()