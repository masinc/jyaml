#!/usr/bin/env python3
"""User-friendly JYAML API examples."""

from jyaml import (
    loads, parse, 
    loads_strict, loads_permissive, loads_fast, loads_ordered,
    ParseOptions, LoadOptions, JYAMLMode
)
from decimal import Decimal
from collections import OrderedDict

def demonstrate_simple_usage():
    """Show the simplest ways to use JYAML."""
    print("=== Simple Usage ===")
    
    json_data = '{"name": "Alice", "age": 30, "active": true}'
    
    print("1. Basic usage (just like json.loads):")
    data = loads(json_data)
    print(f"   Result: {data}")
    print(f"   Types: name={type(data['name'])}, age={type(data['age'])}, active={type(data['active'])}")
    
    print()

def demonstrate_convenience_functions():
    """Show convenience functions for common cases."""
    print("=== Convenience Functions ===")
    
    jyaml_data = '''
    # User configuration
    {
      "name": "Bob",
      "settings": {
        "theme": "dark",
        "notifications": true
      },
      "scores": [95, 87, 92]
    }
    '''
    
    print("1. loads_strict() - strict type checking:")
    data = loads_strict(jyaml_data)
    print(f"   Result: {data['name']} has {len(data['scores'])} scores")
    
    print("2. loads_ordered() - preserve key order:")
    data = loads_ordered(jyaml_data)
    print(f"   Type: {type(data)}")
    print(f"   Keys: {list(data.keys())}")
    
    print("3. loads_fast() - no comments, faster:")
    data = loads_fast(jyaml_data)
    print(f"   Result: {data['settings']['theme']} theme")
    
    print("4. loads_permissive() - flexible parsing:")
    data = loads_permissive(jyaml_data)
    print(f"   Result: {data}")
    
    print()

def demonstrate_preset_usage():
    """Show preset-based configuration."""
    print("=== Preset Usage ===")
    
    data_text = '{"price": 19.99, "quantity": 3, "total": 59.97}'
    
    print("1. Default preset:")
    data = loads(data_text)
    print(f"   price: {type(data['price'])} = {data['price']}")
    
    print("2. High precision preset:")
    data = loads(data_text, preset='high_precision')
    print(f"   price: {type(data['price'])} = {data['price']}")
    
    print("3. Strings only preset:")
    data = loads(data_text, preset='strings_only')
    print(f"   price: {type(data['price'])} = {data['price']}")
    
    print("4. Preserve order preset:")
    data = loads(data_text, preset='preserve_order')
    print(f"   type: {type(data)}")
    
    print()

def demonstrate_kwargs_usage():
    """Show direct kwargs usage."""
    print("=== Direct Options (kwargs) ===")
    
    data_text = '{"enabled": true, "count": 42, "rate": 3.14}'
    
    print("1. Disable number parsing:")
    data = loads(data_text, parse_numbers=False)
    print(f"   count: {type(data['count'])} = {data['count']}")
    
    print("2. Use OrderedDict:")
    data = loads(data_text, use_ordered_dict=True)
    print(f"   type: {type(data)}")
    
    print("3. Use Decimal for precision:")
    data = loads(data_text, use_decimal=True)
    print(f"   rate: {type(data['rate'])} = {data['rate']}")
    
    print("4. Everything as strings:")
    data = loads(data_text, parse_numbers=False, parse_booleans=False)
    print(f"   enabled: {type(data['enabled'])} = {data['enabled']}")
    print(f"   count: {type(data['count'])} = {data['count']}")
    
    print()

def demonstrate_custom_hooks():
    """Show custom conversion hooks."""
    print("=== Custom Hooks ===")
    
    data_text = '''
    {
      "user": {"name": "Alice", "role": "admin"},
      "metrics": {"score": 95.5, "rating": 4.8}
    }
    '''
    
    print("1. Custom object hook:")
    class User:
        def __init__(self, pairs):
            self.__dict__.update(dict(pairs))
        def __repr__(self):
            return f"User({self.__dict__})"
    
    def custom_object_hook(pairs):
        data = dict(pairs)
        if 'name' in data and 'role' in data:
            return User(pairs)
        return data
    
    data = loads(data_text, object_hook=custom_object_hook)
    print(f"   user: {data['user']}")
    print(f"   metrics: {data['metrics']}")
    
    print("2. Custom number hook:")
    def round_numbers(value):
        if isinstance(value, float):
            return round(value, 1)
        return value
    
    data = loads(data_text, number_hook=round_numbers)
    print(f"   score: {data['metrics']['score']}")
    print(f"   rating: {data['metrics']['rating']}")
    
    print()

def demonstrate_parse_options():
    """Show parse function with options."""
    print("=== Parse Function Options ===")
    
    commented_data = '''
    # Configuration file
    {
      "app": "MyApp", # Application name
      "version": "1.0.0"
    }
    '''
    
    print("1. Default parse (with comments):")
    doc = parse(commented_data)
    print(f"   Comments: {doc.comments}")
    print(f"   Data: {doc.data}")
    
    print("2. Fast parse (no comments):")
    doc = parse(commented_data, preset='fast')
    print(f"   Comments: {doc.comments}")
    
    print("3. Debug parse (detailed info):")
    doc = parse(commented_data, preset='debug')
    print(f"   Comments: {doc.comments}")
    print(f"   Has detailed info: {hasattr(doc, 'source_info')}")
    
    print("4. Custom parse options:")
    doc = parse(commented_data, preserve_comments=False, strict_mode=False)
    print(f"   Comments: {doc.comments}")
    
    print()

def demonstrate_error_handling():
    """Show error handling with different modes."""
    print("=== Error Handling ===")
    
    invalid_data = '{"incomplete": }'
    
    print("1. Strict mode (default):")
    try:
        data = loads(invalid_data)
        print(f"   Unexpected success: {data}")
    except Exception as e:
        print(f"   ✓ Caught error: {type(e).__name__}: {e}")
    
    print("2. Permissive mode:")
    try:
        data = loads_permissive(invalid_data)
        print(f"   Unexpected success: {data}")
    except Exception as e:
        print(f"   Still caught error: {type(e).__name__}: {e}")
    
    print("3. Custom error handling:")
    try:
        data = loads(invalid_data, strict_mode=False)
        print(f"   Success: {data}")
    except Exception as e:
        print(f"   Error: {type(e).__name__}: {e}")
    
    print()

def demonstrate_advanced_usage():
    """Show advanced usage patterns."""
    print("=== Advanced Usage Patterns ===")
    
    print("1. Combining parse and load options:")
    parse_opts = ParseOptions.from_preset('debug')
    load_opts = LoadOptions(
        parse_options=parse_opts,
        use_decimal=True,
        use_ordered_dict=True
    )
    
    data_text = '# Test\n{"value": 3.14159}'
    data = loads(data_text, options=load_opts)
    print(f"   Result: {type(data)} with value {type(data['value'])}")
    
    print("2. Creating custom presets:")
    class MyCustomOptions:
        @staticmethod
        def scientific():
            return LoadOptions(
                use_decimal=True,
                parse_options=ParseOptions(max_depth=50)
            )
    
    data = loads('{"precision": 2.71828}', options=MyCustomOptions.scientific())
    print(f"   Scientific: {type(data['precision'])} = {data['precision']}")
    
    print("3. Runtime option selection:")
    mode = 'production'  # Could come from environment
    
    if mode == 'production':
        options = LoadOptions.from_preset('default')
    elif mode == 'debug':
        options = LoadOptions.from_preset('high_precision')
    else:
        options = LoadOptions()
    
    data = loads('{"mode": "example"}', options=options)
    print(f"   Runtime mode '{mode}': {data}")
    
    print()

def main():
    print("JYAML User-Friendly API Examples")
    print("=" * 35)
    print()
    
    demonstrate_simple_usage()
    demonstrate_convenience_functions()
    demonstrate_preset_usage()
    demonstrate_kwargs_usage()
    demonstrate_custom_hooks()
    demonstrate_parse_options()
    demonstrate_error_handling()
    demonstrate_advanced_usage()
    
    print("Summary of API styles:")
    print("1. loads(text) - Simple, like json.loads")
    print("2. loads_strict(text) - Convenience functions")
    print("3. loads(text, preset='preserve_order') - Preset-based")
    print("4. loads(text, parse_numbers=False) - Direct kwargs")
    print("5. loads(text, options=custom_opts) - Full control")
    print()
    print("Choose the style that fits your use case!")

if __name__ == "__main__":
    main()