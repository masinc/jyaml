#!/usr/bin/env python3
"""Pydantic-powered JYAML options examples."""

from jyaml import loads, parse, ParseOptions, LoadOptions
from pydantic import ValidationError
from decimal import Decimal
from collections import OrderedDict

def demonstrate_option_validation():
    """Show Pydantic validation for options."""
    print("=== Option Validation with Pydantic ===")
    
    print("1. Valid options:")
    try:
        opts = ParseOptions(strict_mode=True, max_depth=500)
        print(f"   ✓ Valid ParseOptions: max_depth={opts.max_depth}")
    except ValidationError as e:
        print(f"   ✗ Unexpected validation error: {e}")
    
    print("2. Invalid max_depth (negative):")
    try:
        opts = ParseOptions(max_depth=-1)
        print(f"   ✗ Unexpected success: {opts}")
    except ValidationError as e:
        print(f"   ✓ Validation caught error: {e.errors()[0]['msg']}")
    
    print("3. Invalid max_depth (too large):")
    try:
        opts = ParseOptions(max_depth=1000000)
        print(f"   ✗ Unexpected success: {opts}")
    except ValidationError as e:
        print(f"   ✓ Validation caught error: {e.errors()[0]['msg']}")
    
    print("4. Incompatible options (strict + duplicate keys):")
    try:
        opts = ParseOptions(strict_mode=True, allow_duplicate_keys=True)
        print(f"   ✗ Unexpected success: {opts}")
    except ValidationError as e:
        print(f"   ✓ Validation caught incompatibility: {e.errors()[0]['msg']}")
    
    print()

def demonstrate_load_option_validation():
    """Show LoadOptions validation."""
    print("=== LoadOptions Validation ===")
    
    print("1. Valid callable hook:")
    def custom_hook(pairs):
        return dict(pairs)
    
    try:
        opts = LoadOptions(object_hook=custom_hook)
        print(f"   ✓ Valid callable hook: {opts.object_hook.__name__}")
    except ValidationError as e:
        print(f"   ✗ Unexpected validation error: {e}")
    
    print("2. Invalid hook (not callable):")
    try:
        opts = LoadOptions(object_hook="not_callable")
        print(f"   ✗ Unexpected success: {opts}")
    except ValidationError as e:
        print(f"   ✓ Validation caught error: {e.errors()[0]['msg']}")
    
    print("3. Inconsistent options (use_decimal without parse_numbers):")
    try:
        opts = LoadOptions(use_decimal=True, parse_numbers=False)
        print(f"   ✗ Unexpected success: {opts}")
    except ValidationError as e:
        print(f"   ✓ Validation caught inconsistency: {e.errors()[0]['msg']}")
    
    print("4. Auto-correction (use_ordered_dict sets as_dict=False):")
    try:
        opts = LoadOptions(as_dict=True, use_ordered_dict=True)
        print(f"   ✓ Auto-corrected: as_dict={opts.as_dict}, use_ordered_dict={opts.use_ordered_dict}")
    except ValidationError as e:
        print(f"   ✗ Unexpected validation error: {e}")
    
    print()

def demonstrate_field_descriptions():
    """Show field descriptions and help."""
    print("=== Field Descriptions and Help ===")
    
    print("1. ParseOptions field info:")
    # Use model_fields instead of JSON schema to avoid callable issues
    for field_name, field_info in ParseOptions.model_fields.items():
        if hasattr(field_info, 'description') and field_info.description:
            print(f"   {field_name}: {field_info.description}")
    
    print("\n2. LoadOptions field info (non-callable fields):")
    for field_name, field_info in LoadOptions.model_fields.items():
        if hasattr(field_info, 'description') and field_info.description:
            if field_name not in ['object_hook', 'number_hook']:  # Skip callable fields
                print(f"   {field_name}: {field_info.description}")
    
    print()

def demonstrate_option_serialization():
    """Show option serialization/deserialization."""
    print("=== Option Serialization ===")
    
    # Create options (no callable hooks to avoid serialization issues)
    parse_opts = ParseOptions(
        strict_mode=False,
        preserve_comments=True,
        max_depth=100
    )
    
    load_opts = LoadOptions(
        use_decimal=True,
        use_ordered_dict=True,
        parse_options=parse_opts
        # No callable hooks in this example
    )
    
    print("1. Serialize to dict:")
    parse_dict = parse_opts.model_dump()
    load_dict = load_opts.model_dump(exclude={'object_hook', 'number_hook'})  # Exclude callables
    print(f"   ParseOptions: {parse_dict}")
    print(f"   LoadOptions keys: {list(load_dict.keys())}")
    
    print("2. Serialize to JSON:")
    parse_json = parse_opts.model_dump_json()
    load_json = load_opts.model_dump_json(exclude={'object_hook', 'number_hook'})
    print(f"   ParseOptions JSON: {parse_json}")
    print(f"   LoadOptions JSON length: {len(load_json)} chars")
    
    print("3. Deserialize from dict:")
    new_parse_opts = ParseOptions(**parse_dict)
    new_load_opts = LoadOptions(**load_dict)
    print(f"   Restored ParseOptions: max_depth={new_parse_opts.max_depth}")
    print(f"   Restored LoadOptions: use_decimal={new_load_opts.use_decimal}")
    
    print()

def demonstrate_runtime_validation():
    """Show runtime validation when changing options."""
    print("=== Runtime Validation ===")
    
    # Create mutable options
    opts = ParseOptions(strict_mode=False)
    print(f"1. Initial: strict_mode={opts.strict_mode}, allow_duplicate_keys={opts.allow_duplicate_keys}")
    
    # Valid change
    opts.allow_duplicate_keys = True
    print(f"2. After valid change: allow_duplicate_keys={opts.allow_duplicate_keys}")
    
    # Try invalid change
    print("3. Attempting invalid change (strict + duplicate keys):")
    try:
        opts.strict_mode = True  # This should fail due to model validation
        print(f"   ✗ Unexpected success: {opts}")
    except ValidationError as e:
        print(f"   ✓ Runtime validation caught error: {e.errors()[0]['msg']}")
    
    print()

def demonstrate_custom_validation():
    """Show how to extend validation."""
    print("=== Custom Validation Examples ===")
    
    from pydantic import field_validator, model_validator
    from typing import Optional
    
    class CustomLoadOptions(LoadOptions):
        """Extended LoadOptions with custom validation."""
        
        max_precision: Optional[int] = None
        
        @field_validator('max_precision')
        @classmethod
        def validate_precision(cls, v):
            if v is not None and (v < 1 or v > 50):
                raise ValueError('max_precision must be between 1 and 50')
            return v
        
        @model_validator(mode='after')
        def validate_precision_consistency(self):
            if self.max_precision is not None and not self.use_decimal:
                raise ValueError('max_precision requires use_decimal=True')
            return self
    
    print("1. Valid custom options:")
    try:
        opts = CustomLoadOptions(use_decimal=True, max_precision=10)
        print(f"   ✓ Custom validation passed: max_precision={opts.max_precision}")
    except ValidationError as e:
        print(f"   ✗ Unexpected validation error: {e}")
    
    print("2. Invalid custom options (precision without decimal):")
    try:
        opts = CustomLoadOptions(use_decimal=False, max_precision=10)
        print(f"   ✗ Unexpected success: {opts}")
    except ValidationError as e:
        print(f"   ✓ Custom validation caught error: {e.errors()[0]['msg']}")
    
    print()

def demonstrate_option_inheritance():
    """Show option inheritance and composition."""
    print("=== Option Inheritance and Composition ===")
    
    print("1. Compose options:")
    base_parse = ParseOptions(preserve_comments=False)
    debug_parse = ParseOptions.from_preset('debug')
    
    # Create composite options
    composite = ParseOptions(
        strict_mode=base_parse.strict_mode,
        preserve_comments=debug_parse.preserve_comments,
        include_comment_positions=debug_parse.include_comment_positions,
        max_depth=50  # Custom value
    )
    
    print(f"   Composite: strict={composite.strict_mode}, comments={composite.preserve_comments}, depth={composite.max_depth}")
    
    print("2. Update from another options object:")
    source_opts = ParseOptions(max_depth=200, normalize_line_endings=False)
    target_opts = ParseOptions()
    
    # Update target with source values
    update_data = source_opts.model_dump(exclude={'strict_mode', 'preserve_comments'})
    target_opts = ParseOptions(**{**target_opts.model_dump(), **update_data})
    
    print(f"   Updated: max_depth={target_opts.max_depth}, normalize={target_opts.normalize_line_endings}")
    
    print()

def demonstrate_practical_usage():
    """Show practical usage with validation."""
    print("=== Practical Usage with Validation ===")
    
    data_text = '{"price": 19.99, "name": "Product"}'
    
    print("1. Safe option creation:")
    try:
        # This will validate all options
        opts = LoadOptions(
            use_decimal=True,
            parse_numbers=True,  # Required for decimal
            use_ordered_dict=True
        )
        
        result = loads(data_text, options=opts)
        print(f"   ✓ Success: price type={type(result['price'])}, container type={type(result)}")
        
    except ValidationError as e:
        print(f"   ✗ Option validation failed: {e}")
    except Exception as e:
        print(f"   ✗ Parse error: {e}")
    
    print("2. Configuration from external source:")
    # Simulate loading config from file/environment
    config_dict = {
        'use_decimal': True,
        'parse_numbers': True,
        'use_ordered_dict': True,
        'parse_options': {
            'max_depth': 10,
            'preserve_comments': False
        }
    }
    
    try:
        opts = LoadOptions(**config_dict)
        result = loads(data_text, options=opts)
        print(f"   ✓ Config-based options: {type(result)}")
        
    except ValidationError as e:
        print(f"   ✗ Config validation failed: {e.errors()}")
    
    print()

def main():
    print("JYAML Pydantic Options Examples")
    print("=" * 35)
    print()
    
    demonstrate_option_validation()
    demonstrate_load_option_validation()
    demonstrate_field_descriptions()
    demonstrate_option_serialization()
    demonstrate_runtime_validation()
    demonstrate_custom_validation()
    demonstrate_option_inheritance()
    demonstrate_practical_usage()
    
    print("Benefits of Pydantic Options:")
    print("- ✓ Type safety and validation")
    print("- ✓ Clear error messages")
    print("- ✓ Serialization/deserialization")
    print("- ✓ Runtime validation")
    print("- ✓ Self-documenting with descriptions")
    print("- ✓ Extensible for custom validation")

if __name__ == "__main__":
    main()