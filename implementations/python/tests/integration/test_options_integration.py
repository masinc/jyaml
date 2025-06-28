#!/usr/bin/env python3
"""Integration tests for JYAML options with real parsing scenarios."""

from collections import OrderedDict
from decimal import Decimal

import pytest
from pydantic import ValidationError

from jyaml import LoadOptions, ParseOptions, loads, parse
from jyaml.lexer import LexerError
from jyaml.parser import ParseError


class TestParseOptionsIntegration:
    """Integration tests for ParseOptions."""

    def test_strict_mode_enforcement(self):
        """Test strict mode enforcement in parsing."""
        invalid_jyaml = '{"incomplete": }'

        # Strict mode (default) should fail
        with pytest.raises((LexerError, ParseError)):
            parse(invalid_jyaml)

        # Non-strict mode might be more permissive (depends on implementation)
        try:
            parse(invalid_jyaml, strict_mode=False)
            # If it doesn't raise, that's also valid behavior
        except (LexerError, ParseError):
            # If it still raises, that's also fine for this level of error
            pass

    def test_comment_preservation(self):
        """Test comment preservation options."""
        commented_jyaml = """
        # Top comment
        {
          "key": "value", # Inline comment
          "number": 42
        }
        """

        # With comment preservation (default)
        doc = parse(commented_jyaml, preserve_comments=True)
        assert len(doc.comments) >= 2
        assert any("Top comment" in comment for comment in doc.comments)
        assert any("Inline comment" in comment for comment in doc.comments)

        # Without comment preservation
        doc = parse(commented_jyaml, preserve_comments=False)
        assert len(doc.comments) == 0

    def test_max_depth_limit(self):
        """Test maximum depth limiting."""
        # Create deeply nested structure
        deep_jyaml = "{}"
        for i in range(10):
            deep_jyaml = f'{{"level{i}": {deep_jyaml}}}'

        # Should parse with sufficient depth
        doc = parse(deep_jyaml, max_depth=15)
        assert doc.data is not None

        # Should fail with insufficient depth
        with pytest.raises(ParseError) as exc_info:
            parse(deep_jyaml, max_depth=5)
        assert "depth exceeded" in str(exc_info.value)

    def test_normalize_line_endings(self):
        """Test line ending normalization."""
        windows_jyaml = '{\r\n  "key": "value"\r\n}'

        # Should parse with normalization (default)
        doc = parse(windows_jyaml, normalize_line_endings="lf")
        assert doc.data is not None

        # Should parse without normalization
        doc = parse(windows_jyaml, normalize_line_endings="none")
        assert doc.data is not None

    def test_preset_behavior(self):
        """Test preset behavior in real parsing."""
        test_jyaml = """
        # Configuration
        {
          "nested": {
            "deep": {
              "value": true
            }
          },
          "list": [1, 2, 3]
        }
        """

        # Fast preset (no comments)
        doc = parse(test_jyaml, preset="fast")
        assert len(doc.comments) == 0
        assert doc.data is not None

        # Debug preset (with comments and positions)
        doc = parse(test_jyaml, preset="debug")
        assert len(doc.comments) > 0

        # Permissive preset
        doc = parse(test_jyaml, preset="permissive")
        assert doc.data is not None


class TestLoadOptionsIntegration:
    """Integration tests for LoadOptions."""

    def test_type_conversion_control(self):
        """Test type conversion control options."""
        typed_jyaml = """
        {
          "string": "hello",
          "number": 42,
          "float": 3.14,
          "boolean": true,
          "null": null
        }
        """

        # Default (native types)
        data = loads(typed_jyaml)
        assert isinstance(data["string"], str)
        assert isinstance(data["number"], int)
        assert isinstance(data["float"], float)
        assert isinstance(data["boolean"], bool)
        assert data["null"] is None

        # Strings only
        data = loads(typed_jyaml, preset="strings_only")
        assert isinstance(data["string"], str)
        assert isinstance(data["number"], str)
        assert isinstance(data["boolean"], str)
        assert isinstance(data["null"], str)

        # Custom control
        data = loads(
            typed_jyaml, parse_numbers=False, parse_booleans=True, parse_null=True
        )
        assert isinstance(data["number"], str)  # Not parsed
        assert isinstance(data["boolean"], bool)  # Parsed
        assert data["null"] is None  # Parsed

    def test_decimal_precision(self):
        """Test decimal precision handling."""
        precision_jyaml = """
        {
          "price": 19.99,
          "rate": 0.125,
          "total": 123.456789
        }
        """

        # Default float
        data = loads(precision_jyaml)
        assert isinstance(data["price"], float)

        # Decimal precision
        data = loads(precision_jyaml, use_decimal=True)
        assert isinstance(data["price"], Decimal)
        assert isinstance(data["rate"], Decimal)
        assert isinstance(data["total"], Decimal)

        # Verify precision preservation
        assert str(data["price"]) == "19.99"
        assert str(data["rate"]) == "0.125"

    def test_ordered_dict_preservation(self):
        """Test key order preservation."""
        ordered_jyaml = """
        {
          "first": 1,
          "second": 2,
          "third": 3,
          "fourth": 4,
          "fifth": 5
        }
        """

        # Default dict
        data = loads(ordered_jyaml)
        assert isinstance(data, dict)

        # Ordered dict
        data = loads(ordered_jyaml, use_ordered_dict=True)
        assert isinstance(data, OrderedDict)
        keys = list(data.keys())
        assert keys == ["first", "second", "third", "fourth", "fifth"]

    def test_custom_hooks(self):
        """Test custom conversion hooks."""
        hook_jyaml = """
        {
          "user": {"name": "Alice", "role": "admin"},
          "score": 95.7,
          "values": [1.1, 2.2, 3.3]
        }
        """

        # Custom object hook
        def make_namespace(pairs):
            class Namespace:
                def __init__(self, **kwargs):
                    self.__dict__.update(kwargs)

                def __repr__(self):
                    return f"Namespace({self.__dict__})"

            data = dict(pairs)
            if "name" in data and "role" in data:
                return Namespace(**data)
            return data

        data = loads(hook_jyaml, object_hook=make_namespace)
        assert hasattr(data["user"], "name")
        assert data["user"].name == "Alice"
        assert isinstance(data["score"], float)

        # Custom number hook
        def round_numbers(value):
            if isinstance(value, float):
                return round(value, 1)
            return value

        data = loads(hook_jyaml, number_hook=round_numbers)
        assert data["score"] == 95.7  # Already rounded
        assert all(isinstance(v, float) for v in data["values"])

    def test_combined_options(self):
        """Test combining multiple options."""
        complex_jyaml = """
        # Complex configuration
        {
          "precision": 3.141592653589793,
          "config": {
            "timeout": 30.5,
            "retries": 3
          },
          "features": ["auth", "cache", "monitor"]
        }
        """

        # Combine decimal + ordered dict + custom parse options
        parse_opts = ParseOptions(preserve_comments=True, max_depth=50)
        load_opts = LoadOptions(
            use_decimal=True, use_ordered_dict=True, parse_options=parse_opts
        )

        data = loads(complex_jyaml, options=load_opts)

        # Check types
        assert isinstance(data, OrderedDict)
        assert isinstance(data["precision"], Decimal)
        assert isinstance(data["config"], OrderedDict)
        assert isinstance(data["config"]["timeout"], Decimal)

        # Check precision
        assert str(data["precision"]) == "3.141592653589793"


class TestOptionValidationIntegration:
    """Integration tests for option validation in real scenarios."""

    def test_invalid_options_prevent_parsing(self):
        """Test that invalid options prevent parsing."""
        test_jyaml = '{"test": "data"}'

        # Invalid ParseOptions should fail before parsing
        with pytest.raises(ValidationError):
            parse(test_jyaml, max_depth=-1)

        with pytest.raises(ValidationError):
            parse(test_jyaml, strict_mode=True, allow_duplicate_keys=True)

        # Invalid LoadOptions should fail before parsing
        with pytest.raises(ValidationError):
            loads(test_jyaml, use_decimal=True, parse_numbers=False)

    def test_option_conflicts_detected(self):
        """Test that option conflicts are detected."""
        test_jyaml = '{"number": 42}'

        # These combinations should be detected as invalid
        invalid_combinations = [
            {"as_native_types": False, "use_decimal": True},
            {"as_native_types": False, "use_ordered_dict": True},
            {"use_decimal": True, "parse_numbers": False},
        ]

        for combo in invalid_combinations:
            with pytest.raises(ValidationError):
                loads(test_jyaml, **combo)

    def test_auto_correction_in_practice(self):
        """Test auto-correction in practical scenarios."""
        test_jyaml = '{"a": 1, "b": 2, "c": 3}'

        # This should auto-correct as_dict to False
        data = loads(test_jyaml, as_dict=True, use_ordered_dict=True)
        assert isinstance(data, OrderedDict)

    def test_nested_option_validation(self):
        """Test validation of nested ParseOptions in LoadOptions."""
        test_jyaml = '{"test": true}'

        # Invalid nested ParseOptions
        with pytest.raises(ValidationError):
            invalid_parse_opts = ParseOptions(max_depth=-1)
            loads(test_jyaml, options=LoadOptions(parse_options=invalid_parse_opts))


class TestRealWorldScenarios:
    """Test options in real-world usage scenarios."""

    def test_configuration_file_processing(self):
        """Test processing configuration files with different option sets."""
        config_jyaml = """
        # Application Configuration
        {
          "app": {
            "name": "MyService",
            "version": "1.2.3",
            "debug": false
          },
          "database": {
            "host": "localhost",
            "port": 5432,
            "timeout": 30.0,
            "pool_size": 10
          },
          "features": {
            "caching": true,
            "monitoring": true,
            "auth": {
              "enabled": true,
              "timeout": 3600.0
            }
          }
        }
        """

        # Development mode (preserve comments, permissive)
        dev_data = loads(config_jyaml, preset="preserve_order")
        assert isinstance(dev_data, OrderedDict)
        assert dev_data["app"]["name"] == "MyService"

        # Production mode (fast, strict types)
        prod_data = loads(config_jyaml, preset="strict_types")
        assert isinstance(prod_data["database"]["port"], int)
        assert isinstance(prod_data["database"]["timeout"], float)

        # High precision mode (for financial data)
        precision_data = loads(config_jyaml, preset="high_precision")
        assert isinstance(precision_data["database"]["timeout"], Decimal)
        assert isinstance(precision_data["features"]["auth"]["timeout"], Decimal)

    def test_api_response_processing(self):
        """Test processing API responses with different requirements."""
        api_response = """
        {
          "status": "success",
          "data": {
            "users": [
              {"id": 1, "name": "Alice", "active": true},
              {"id": 2, "name": "Bob", "active": false}
            ],
            "pagination": {
              "page": 1,
              "per_page": 10,
              "total": 2
            }
          },
          "timestamp": 1234567890.123
        }
        """

        # Standard processing
        data = loads(api_response)
        assert data["status"] == "success"
        assert len(data["data"]["users"]) == 2
        assert isinstance(data["data"]["users"][0]["id"], int)

        # Preserve field order for consistent output
        ordered_data = loads(api_response, use_ordered_dict=True)
        assert isinstance(ordered_data, OrderedDict)
        assert list(ordered_data.keys())[0] == "status"

        # High precision timestamps
        precise_data = loads(api_response, use_decimal=True)
        assert isinstance(precise_data["timestamp"], Decimal)
        assert str(precise_data["timestamp"]) == "1234567890.123"

    def test_data_migration_scenarios(self):
        """Test data migration with different parsing strategies."""
        legacy_data = """
        {
          "records": [
            {"id": "001", "value": "123.45", "active": "true"},
            {"id": "002", "value": "67.89", "active": "false"}
          ]
        }
        """

        # Parse everything as strings for validation
        string_data = loads(legacy_data, preset="strings_only")
        assert isinstance(string_data["records"][0]["value"], str)
        assert isinstance(string_data["records"][0]["active"], str)

        # Parse with type conversion for processing
        # Note: Quoted numbers remain strings (correct JYAML behavior)
        typed_data = loads(legacy_data)
        assert isinstance(
            typed_data["records"][0]["value"], str
        )  # "123.45" stays string
        assert isinstance(
            typed_data["records"][0]["active"], str
        )  # "true" stays string

        # Use custom hooks for complex transformation
        def transform_record(pairs):
            data = dict(pairs)
            if "id" in data and "value" in data:
                # Transform legacy format
                data["id"] = int(data["id"])
                if isinstance(data["value"], str):
                    data["value"] = float(data["value"])
                if isinstance(data["active"], str):
                    data["active"] = data["active"].lower() == "true"
            return data

        transformed_data = loads(legacy_data, object_hook=transform_record)
        assert isinstance(transformed_data["records"][0]["id"], int)
        assert transformed_data["records"][0]["id"] == 1


if __name__ == "__main__":
    pytest.main([__file__])
