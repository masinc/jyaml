#!/usr/bin/env python3
"""Unit tests for JYAML Pydantic options."""

import pytest
from decimal import Decimal
from collections import OrderedDict
from pydantic import ValidationError

from jyaml.parser import ParseOptions, LoadOptions, JYAMLMode


class TestParseOptions:
    """Test ParseOptions Pydantic model."""
    
    def test_default_options(self):
        """Test default option values."""
        opts = ParseOptions()
        
        assert opts.strict_mode is True
        assert opts.preserve_comments is True
        assert opts.allow_duplicate_keys is False
        assert opts.max_depth == 1000
        assert opts.include_comment_positions is False
        assert opts.normalize_line_endings == "lf"
    
    def test_custom_options(self):
        """Test custom option values."""
        opts = ParseOptions(
            strict_mode=False,
            preserve_comments=False,
            max_depth=500,
            normalize_line_endings="crlf"
        )
        
        assert opts.strict_mode is False
        assert opts.preserve_comments is False
        assert opts.max_depth == 500
        assert opts.normalize_line_endings == "crlf"
    
    def test_max_depth_validation(self):
        """Test max_depth validation."""
        # Valid values
        ParseOptions(max_depth=1)
        ParseOptions(max_depth=100000)
        ParseOptions(max_depth=None)
        
        # Invalid values
        with pytest.raises(ValidationError) as exc_info:
            ParseOptions(max_depth=0)
        assert "Input should be greater than 0" in str(exc_info.value)
        
        with pytest.raises(ValidationError) as exc_info:
            ParseOptions(max_depth=-1)
        assert "Input should be greater than 0" in str(exc_info.value)
        
        with pytest.raises(ValidationError) as exc_info:
            ParseOptions(max_depth=100001)
        assert "Input should be less than or equal to 100000" in str(exc_info.value)
    
    def test_normalize_line_endings_validation(self):
        """Test normalize_line_endings validation."""
        # Valid values
        ParseOptions(normalize_line_endings="none")
        ParseOptions(normalize_line_endings="lf")
        ParseOptions(normalize_line_endings="crlf")
        
        # Invalid values
        with pytest.raises(ValidationError) as exc_info:
            ParseOptions(normalize_line_endings="invalid")
        assert "Input should be 'none', 'lf' or 'crlf'" in str(exc_info.value)
    
    def test_consistency_validation(self):
        """Test option consistency validation."""
        # strict_mode and allow_duplicate_keys are incompatible
        with pytest.raises(ValidationError) as exc_info:
            ParseOptions(strict_mode=True, allow_duplicate_keys=True)
        assert "strict_mode and allow_duplicate_keys are incompatible" in str(exc_info.value)
        
        # include_comment_positions requires preserve_comments
        with pytest.raises(ValidationError) as exc_info:
            ParseOptions(preserve_comments=False, include_comment_positions=True)
        assert "include_comment_positions requires preserve_comments=True" in str(exc_info.value)
    
    def test_runtime_validation(self):
        """Test runtime validation when updating fields."""
        opts = ParseOptions(strict_mode=False, allow_duplicate_keys=True)
        
        # This should fail due to consistency check
        with pytest.raises(ValidationError):
            opts.strict_mode = True
    
    def test_from_preset(self):
        """Test creating options from presets."""
        # Test all presets
        strict_opts = ParseOptions.from_preset('strict')
        assert strict_opts.strict_mode is True
        assert strict_opts.preserve_comments is True
        assert strict_opts.max_depth == 1000
        
        permissive_opts = ParseOptions.from_preset('permissive')
        assert permissive_opts.strict_mode is False
        assert permissive_opts.allow_duplicate_keys is True
        assert permissive_opts.max_depth == 10000
        
        fast_opts = ParseOptions.from_preset('fast')
        assert fast_opts.strict_mode is True
        assert fast_opts.preserve_comments is False
        assert fast_opts.max_depth == 100
        
        debug_opts = ParseOptions.from_preset('debug')
        assert debug_opts.strict_mode is False
        assert debug_opts.preserve_comments is True
        assert debug_opts.include_comment_positions is True
        assert debug_opts.allow_duplicate_keys is True
        
        # Invalid preset
        with pytest.raises(ValueError) as exc_info:
            ParseOptions.from_preset('invalid')
        assert "Unknown preset: invalid" in str(exc_info.value)
        assert "Available: strict, permissive, fast, debug" in str(exc_info.value)
    
    def test_serialization(self):
        """Test option serialization/deserialization."""
        opts = ParseOptions(
            strict_mode=False,
            max_depth=200,
            preserve_comments=False
        )
        
        # Serialize to dict
        data = opts.model_dump()
        assert data['strict_mode'] is False
        assert data['max_depth'] == 200
        assert data['preserve_comments'] is False
        
        # Serialize to JSON
        json_str = opts.model_dump_json()
        assert '"strict_mode":false' in json_str
        assert '"max_depth":200' in json_str
        
        # Deserialize from dict
        new_opts = ParseOptions(**data)
        assert new_opts.strict_mode == opts.strict_mode
        assert new_opts.max_depth == opts.max_depth
        assert new_opts.preserve_comments == opts.preserve_comments
    
    def test_extra_fields_forbidden(self):
        """Test that extra fields are forbidden."""
        with pytest.raises(ValidationError) as exc_info:
            ParseOptions(unknown_field=True)
        assert "Extra inputs are not permitted" in str(exc_info.value)


class TestLoadOptions:
    """Test LoadOptions Pydantic model."""
    
    def test_default_options(self):
        """Test default option values."""
        opts = LoadOptions()
        
        assert opts.as_dict is True
        assert opts.as_native_types is True
        assert opts.parse_numbers is True
        assert opts.parse_booleans is True
        assert opts.parse_null is True
        assert opts.use_decimal is False
        assert opts.use_ordered_dict is False
        assert opts.object_hook is None
        assert opts.number_hook is None
        assert opts.parse_options is None
    
    def test_custom_options(self):
        """Test custom option values."""
        def custom_hook(pairs):
            return dict(pairs)
        
        def number_hook(value):
            return round(value, 2)
        
        parse_opts = ParseOptions(max_depth=50)
        
        opts = LoadOptions(
            use_decimal=True,
            use_ordered_dict=True,
            object_hook=custom_hook,
            number_hook=number_hook,
            parse_options=parse_opts
        )
        
        assert opts.use_decimal is True
        assert opts.use_ordered_dict is True
        assert opts.object_hook is custom_hook
        assert opts.number_hook is number_hook
        assert opts.parse_options is parse_opts
    
    def test_callable_validation(self):
        """Test callable hook validation."""
        # Valid callable
        def valid_hook(pairs):
            return dict(pairs)
        
        LoadOptions(object_hook=valid_hook)
        LoadOptions(number_hook=lambda x: x)
        
        # Invalid callable
        with pytest.raises(ValidationError) as exc_info:
            LoadOptions(object_hook="not_callable")
        assert "Input should be callable" in str(exc_info.value)
        
        with pytest.raises(ValidationError) as exc_info:
            LoadOptions(number_hook=123)
        assert "Input should be callable" in str(exc_info.value)
    
    def test_consistency_validation(self):
        """Test option consistency validation."""
        # use_decimal requires parse_numbers=True
        with pytest.raises(ValidationError) as exc_info:
            LoadOptions(use_decimal=True, parse_numbers=False)
        assert "use_decimal requires parse_numbers=True" in str(exc_info.value)
        
        # use_decimal and use_ordered_dict require as_native_types=True
        with pytest.raises(ValidationError) as exc_info:
            LoadOptions(as_native_types=False, use_decimal=True)
        assert "use_decimal and use_ordered_dict require as_native_types=True" in str(exc_info.value)
        
        with pytest.raises(ValidationError) as exc_info:
            LoadOptions(as_native_types=False, use_ordered_dict=True)
        assert "use_decimal and use_ordered_dict require as_native_types=True" in str(exc_info.value)
    
    def test_auto_correction(self):
        """Test automatic option correction."""
        # use_ordered_dict should set as_dict=False
        opts = LoadOptions(as_dict=True, use_ordered_dict=True)
        assert opts.as_dict is False
        assert opts.use_ordered_dict is True
    
    def test_from_preset(self):
        """Test creating options from presets."""
        # Default preset
        default_opts = LoadOptions.from_preset('default')
        assert default_opts.as_native_types is True
        assert default_opts.parse_numbers is True
        
        # Strict types preset
        strict_opts = LoadOptions.from_preset('strict_types')
        assert strict_opts.as_native_types is True
        assert strict_opts.parse_numbers is True
        assert strict_opts.parse_booleans is True
        
        # Preserve order preset
        order_opts = LoadOptions.from_preset('preserve_order')
        assert order_opts.use_ordered_dict is True
        
        # High precision preset
        precision_opts = LoadOptions.from_preset('high_precision')
        assert precision_opts.use_decimal is True
        assert precision_opts.use_ordered_dict is True
        
        # Strings only preset
        strings_opts = LoadOptions.from_preset('strings_only')
        assert strings_opts.as_native_types is False
        assert strings_opts.parse_numbers is False
        assert strings_opts.parse_booleans is False
        assert strings_opts.parse_null is False
        
        # Invalid preset
        with pytest.raises(ValueError) as exc_info:
            LoadOptions.from_preset('invalid')
        assert "Unknown preset: invalid" in str(exc_info.value)
    
    def test_serialization_without_callables(self):
        """Test serialization excluding callable fields."""
        opts = LoadOptions(
            use_decimal=True,
            use_ordered_dict=True,
            parse_numbers=True
        )
        
        # Serialize excluding callables
        data = opts.model_dump(exclude={'object_hook', 'number_hook'})
        assert 'object_hook' not in data
        assert 'number_hook' not in data
        assert data['use_decimal'] is True
        
        # JSON serialization
        json_str = opts.model_dump_json(exclude={'object_hook', 'number_hook'})
        assert 'object_hook' not in json_str
        assert 'number_hook' not in json_str
        assert '"use_decimal":true' in json_str
    
    def test_nested_parse_options(self):
        """Test nested ParseOptions validation."""
        parse_opts = ParseOptions(max_depth=100)
        load_opts = LoadOptions(parse_options=parse_opts)
        
        assert load_opts.parse_options.max_depth == 100
        
        # Serialize with nested options
        data = load_opts.model_dump()
        assert data['parse_options']['max_depth'] == 100
        
        # Deserialize with nested options
        new_opts = LoadOptions(**data)
        assert new_opts.parse_options.max_depth == 100


class TestJYAMLMode:
    """Test JYAMLMode preset factory."""
    
    def test_strict_mode(self):
        """Test strict mode preset."""
        opts = JYAMLMode.strict()
        assert isinstance(opts, ParseOptions)
        assert opts.strict_mode is True
        assert opts.preserve_comments is True
        assert opts.max_depth == 1000
    
    def test_permissive_mode(self):
        """Test permissive mode preset."""
        opts = JYAMLMode.permissive()
        assert isinstance(opts, ParseOptions)
        assert opts.strict_mode is False
        assert opts.preserve_comments is True
        assert opts.allow_duplicate_keys is True
        assert opts.max_depth == 10000
    
    def test_fast_mode(self):
        """Test fast mode preset."""
        opts = JYAMLMode.fast()
        assert isinstance(opts, ParseOptions)
        assert opts.strict_mode is True
        assert opts.preserve_comments is False
        assert opts.max_depth == 100
    
    def test_debug_mode(self):
        """Test debug mode preset."""
        opts = JYAMLMode.debug()
        assert isinstance(opts, ParseOptions)
        assert opts.strict_mode is False
        assert opts.preserve_comments is True
        assert opts.include_comment_positions is True
        assert opts.allow_duplicate_keys is True


class TestOptionIntegration:
    """Test option integration with parsing functions."""
    
    def test_parse_with_options(self):
        """Test parse function with options."""
        from jyaml import parse
        
        # Test with preset
        doc = parse('{"test": true}', preset='fast')
        assert len(doc.comments) == 0  # fast mode disables comments
        
        # Test with custom options
        opts = ParseOptions(preserve_comments=False)
        doc = parse('# comment\n{"test": true}', options=opts)
        assert len(doc.comments) == 0
        
        # Test with kwargs
        doc = parse('{"test": true}', strict_mode=False, max_depth=50)
        # Should work without errors
    
    def test_loads_with_options(self):
        """Test loads function with options."""
        from jyaml import loads
        
        # Test with preset
        data = loads('{"number": "42"}', preset='strings_only')
        assert data['number'] == "42"  # Should be string
        
        # Test with custom options
        opts = LoadOptions(parse_numbers=False)
        data = loads('{"number": 42}', options=opts)
        assert data['number'] == "42"  # Should be string
        
        # Test with kwargs
        data = loads('{"value": 3.14}', use_decimal=True, parse_numbers=True)
        assert isinstance(data['value'], Decimal)
    
    def test_convenience_functions(self):
        """Test convenience functions."""
        from jyaml import loads_strict, loads_permissive, loads_fast, loads_ordered
        
        test_data = '{"key": "value", "number": 42}'
        
        # Test strict
        data = loads_strict(test_data)
        assert data['key'] == "value"
        assert isinstance(data['number'], int)
        
        # Test permissive
        data = loads_permissive(test_data)
        assert data['key'] == "value"
        
        # Test fast
        data = loads_fast(test_data)
        assert data['key'] == "value"
        
        # Test ordered
        data = loads_ordered(test_data)
        assert isinstance(data, OrderedDict)
        assert data['key'] == "value"


class TestOptionErrors:
    """Test error handling in options."""
    
    def test_invalid_preset_errors(self):
        """Test error messages for invalid presets."""
        with pytest.raises(ValueError) as exc_info:
            ParseOptions.from_preset('nonexistent')
        
        error_msg = str(exc_info.value)
        assert "Unknown preset: nonexistent" in error_msg
        assert "Available:" in error_msg
        assert "strict" in error_msg
        assert "permissive" in error_msg
        assert "fast" in error_msg
        assert "debug" in error_msg
    
    def test_validation_error_details(self):
        """Test detailed validation error messages."""
        with pytest.raises(ValidationError) as exc_info:
            ParseOptions(
                max_depth=-5,
                max_size=0,
                strict_mode=True,
                allow_duplicate_keys=True
            )
        
        errors = exc_info.value.errors()
        
        # Should have multiple validation errors
        assert len(errors) >= 2
        
        # Check error types
        error_fields = [error['loc'][0] for error in errors]
        assert 'max_depth' in error_fields
        assert 'max_size' in error_fields
    
    def test_field_descriptions(self):
        """Test that all fields have descriptions."""
        # Check ParseOptions
        for field_name, field_info in ParseOptions.model_fields.items():
            assert hasattr(field_info, 'description')
            assert field_info.description is not None
            assert len(field_info.description) > 0
        
        # Check LoadOptions (exclude callable fields from description check)
        for field_name, field_info in LoadOptions.model_fields.items():
            if field_name not in ['object_hook', 'number_hook']:
                assert hasattr(field_info, 'description')
                assert field_info.description is not None
                assert len(field_info.description) > 0


if __name__ == "__main__":
    pytest.main([__file__])