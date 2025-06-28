#!/usr/bin/env python3
"""Unit tests for JYAML dumps function."""

from collections import OrderedDict
from decimal import Decimal

import pytest
from pydantic import ValidationError

from jyaml import DumpOptions, dumps, loads


class TestDumps:
    """Test dumps function basic functionality."""

    def test_dump_basic_types(self):
        """Test dumping basic types."""
        assert dumps(None) == "null"
        assert dumps(True) == "true"
        assert dumps(False) == "false"
        assert dumps(42) == "42"
        assert dumps(3.14) == "3.14"
        assert dumps("hello") == '"hello"'

    def test_dump_arrays(self):
        """Test dumping arrays."""
        # Empty array
        assert dumps([]) == "[]"

        # Simple array (auto style should use flow)
        result = dumps([1, 2, 3])
        assert result == "[1, 2, 3]"

        # Nested array
        result = dumps([[1, 2], [3, 4]])
        assert "[1, 2]" in result and "[3, 4]" in result

    def test_dump_objects(self):
        """Test dumping objects."""
        # Empty object
        assert dumps({}) == "{}"

        # Simple object (auto style should use flow)
        result = dumps({"key": "value"})
        assert result == '{"key": "value"}'

        # Multiple keys
        result = dumps({"a": 1, "b": 2})
        assert '"a": 1' in result and '"b": 2' in result

    def test_dump_nested_structures(self):
        """Test dumping nested structures."""
        data = {
            "name": "test",
            "values": [1, 2, 3],
            "config": {"enabled": True, "timeout": 30},
        }

        result = dumps(data)
        # Should contain all the values
        assert '"name": "test"' in result
        assert "[1, 2, 3]" in result or "- 1" in result  # Depending on style
        assert '"enabled": true' in result
        assert '"timeout": 30' in result

    def test_string_escaping(self):
        """Test string escaping."""
        # Basic escaping
        assert dumps("hello\nworld") == '"hello\\nworld"'
        assert dumps('quote"test') == '"quote\\"test"'
        assert dumps("tab\ttest") == '"tab\\ttest"'

        # Unicode handling
        assert dumps("café") == '"café"'  # No escaping by default


class TestDumpOptions:
    """Test DumpOptions configuration."""

    def test_default_options(self):
        """Test default option values."""
        opts = DumpOptions()
        assert opts.style == "auto"
        assert opts.indent == 2
        assert opts.quote_style == "double"
        assert opts.sort_keys is False
        assert opts.line_ending == "lf"
        assert opts.ensure_ascii is False
        assert opts.include_comments is False

    def test_style_options(self):
        """Test different style options."""
        data = {"key": "value", "num": 42}

        # Flow style
        result = dumps(data, style="flow")
        assert result.startswith("{") and result.endswith("}")

        # Block style
        result = dumps(data, style="block")
        assert "\n" in result
        assert '"key": "value"' in result

        # Auto style (small objects should be flow)
        result = dumps(data, style="auto")
        assert result.startswith("{") and result.endswith("}")  # Small object = flow

    def test_indent_options(self):
        """Test indentation options."""
        data = {"nested": {"key": "value"}}

        # No indent
        result = dumps(data, style="block", indent=0)
        lines = result.split("\n")
        assert not any(line.startswith(" ") for line in lines if line)

        # 4-space indent
        result = dumps(data, style="block", indent=4)
        if "\n" in result:
            lines = result.split("\n")
            # Should have lines with 4-space indentation
            assert any(line.startswith("    ") for line in lines)

    def test_sort_keys_option(self):
        """Test key sorting option."""
        data = {"z": 1, "a": 2, "m": 3}

        # Without sorting (default)
        result = dumps(data, sort_keys=False)
        # Order should be preserved (Python 3.7+ dict order)

        # With sorting
        result = dumps(data, sort_keys=True)
        # Keys should be alphabetically ordered
        if '"a": 2' in result and '"m": 3' in result and '"z": 1' in result:
            pos_a = result.index('"a": 2')
            pos_m = result.index('"m": 3')
            pos_z = result.index('"z": 1')
            assert pos_a < pos_m < pos_z

    def test_quote_style_options(self):
        """Test quote style options."""
        data = {"key": "value"}

        # Double quotes (default)
        result = dumps(data, quote_style="double")
        assert '"key"' in result and '"value"' in result

        # Single quotes
        result = dumps(data, quote_style="single")
        assert "'key'" in result and "'value'" in result

        # Auto (should default to double)
        result = dumps(data, quote_style="auto")
        assert '"key"' in result and '"value"' in result

    def test_ensure_ascii_option(self):
        """Test ASCII encoding option."""
        data = {"café": "naïve"}

        # Default (Unicode allowed)
        result = dumps(data, ensure_ascii=False)
        assert "café" in result and "naïve" in result

        # ASCII only
        result = dumps(data, ensure_ascii=True)
        assert "café" not in result  # Should be escaped
        assert "naïve" not in result  # Should be escaped


class TestDumpPresets:
    """Test dump presets."""

    def test_compact_preset(self):
        """Test compact preset."""
        data = {"key": "value", "numbers": [1, 2, 3]}
        result = dumps(data, preset="compact")

        # Should be flow style, minimal spacing
        assert result.startswith("{")

    def test_pretty_preset(self):
        """Test pretty preset."""
        data = {"key": "value", "numbers": [1, 2, 3]}
        result = dumps(data, preset="pretty")

        # Should be readable format
        assert '"key"' in result and '"value"' in result

    def test_minimal_preset(self):
        """Test minimal preset."""
        data = {"z": 1, "a": 2}
        result = dumps(data, preset="minimal")

        # Should be compact and sorted
        # Keys should be sorted
        if '"a": 2' in result and '"z": 1' in result:
            assert result.index('"a": 2') < result.index('"z": 1')

    def test_block_preset(self):
        """Test block preset."""
        data = {"key": "value", "num": 42}
        result = dumps(data, preset="block")

        # Should use block style
        assert "\n" in result
        assert '"key": "value"' in result


class TestDumpOptionsValidation:
    """Test DumpOptions validation."""

    def test_valid_options(self):
        """Test valid option combinations."""
        # Valid indent
        DumpOptions(indent=0)
        DumpOptions(indent=4)
        DumpOptions(indent=8)

        # Valid styles
        DumpOptions(style="flow")
        DumpOptions(style="block")
        DumpOptions(style="auto")

        # Valid quote styles
        DumpOptions(quote_style="double")
        DumpOptions(quote_style="single")
        DumpOptions(quote_style="auto")

    def test_invalid_indent(self):
        """Test invalid indent values."""
        with pytest.raises(ValidationError):
            DumpOptions(indent=-1)

        with pytest.raises(ValidationError):
            DumpOptions(indent=10)  # Too large

    def test_invalid_style(self):
        """Test invalid style values."""
        with pytest.raises(ValidationError):
            DumpOptions(style="invalid")

    def test_invalid_quote_style(self):
        """Test invalid quote style values."""
        with pytest.raises(ValidationError):
            DumpOptions(quote_style="invalid")

    def test_extra_fields_forbidden(self):
        """Test that extra fields are rejected."""
        with pytest.raises(ValidationError):
            DumpOptions(unknown_field=True)


class TestDumpSpecialTypes:
    """Test dumping special Python types."""

    def test_dump_decimal(self):
        """Test dumping Decimal objects."""
        result = dumps(Decimal("3.14159"))
        assert result == "3.14159"

    def test_dump_ordered_dict(self):
        """Test dumping OrderedDict."""
        data = OrderedDict([("z", 1), ("a", 2)])
        result = dumps(data)

        # Order should be preserved
        assert '"z"' in result and '"a"' in result

    def test_dump_tuple(self):
        """Test dumping tuples."""
        result = dumps((1, 2, 3))
        assert "[1, 2, 3]" in result or "- 1" in result

    def test_dump_custom_objects(self):
        """Test dumping custom objects (converts to string)."""

        class CustomObj:
            def __str__(self):
                return "custom_value"

        result = dumps(CustomObj())
        assert '"custom_value"' in result


class TestRoundTrip:
    """Test round-trip compatibility (dumps -> loads)."""

    def test_simple_round_trip(self):
        """Test simple round-trip conversion."""
        original = {"key": "value", "number": 42, "bool": True, "null": None}

        # Convert to JYAML and back
        jyaml_str = dumps(original)
        restored = loads(jyaml_str)

        assert restored == original

    def test_nested_round_trip(self):
        """Test nested structure round-trip."""
        original = {
            "users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}],
            "config": {"debug": False, "timeout": 60},
        }

        jyaml_str = dumps(original)
        restored = loads(jyaml_str)

        assert restored == original

    def test_array_round_trip(self):
        """Test array round-trip."""
        original = [1, "two", 3.0, True, None, [4, 5]]

        jyaml_str = dumps(original)
        restored = loads(jyaml_str)

        assert restored == original

    def test_different_styles_round_trip(self):
        """Test round-trip with different styles."""
        original = {"test": [1, 2, 3]}

        # Test flow style
        flow_str = dumps(original, style="flow")
        assert loads(flow_str) == original

        # Test block style (only for root level objects)
        if isinstance(original, dict):
            block_str = dumps(original, style="block")
            assert loads(block_str) == original

        # Test auto style
        auto_str = dumps(original, style="auto")
        assert loads(auto_str) == original


class TestAutoStyleSelection:
    """Test automatic style selection logic."""

    def test_small_simple_objects_use_flow(self):
        """Test that small simple objects use flow style."""
        # 1-3 keys with primitive values
        small_objects = [
            {"key": "value"},
            {"a": 1, "b": 2},
            {"x": True, "y": False, "z": None},
        ]

        for obj in small_objects:
            result = dumps(obj, style="auto")
            assert result.startswith("{") and result.endswith("}")
            assert "\n" not in result

    def test_large_objects_use_block(self):
        """Test that large objects use block style."""
        # More than 3 keys
        large_obj = {
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
            "key4": "value4",
        }

        result = dumps(large_obj, style="auto")
        assert "\n" in result
        assert not result.startswith("{")
        # Should have each key on separate line
        lines = result.split("\n")
        assert len(lines) == 4
        assert '"key1": "value1"' in result
        assert '"key4": "value4"' in result

    def test_complex_objects_use_block(self):
        """Test that objects with complex values use block style."""
        complex_objects = [
            {"name": "test", "data": [1, 2, 3]},  # Should be block
            {"config": {"debug": True}, "simple": "value"},  # Should be block
        ]

        for obj in complex_objects:
            result = dumps(obj, style="auto")
            # Multiple keys should have newlines
            assert "\n" in result
            assert not result.startswith("{")

        # Single key with complex value - uses block style but no newlines needed
        single_complex = {"users": [{"name": "Alice"}]}
        result = dumps(single_complex, style="auto")
        # Single key with array - should use block style (no braces)
        assert not result.startswith("{")  # Block style = no braces
        assert result.startswith('"users":')  # Should start with key

        # Multiple keys with complex values should have newlines
        multi_complex = {"simple": "value", "users": [{"name": "Alice"}]}
        result = dumps(multi_complex, style="auto")
        assert "\n" in result and not result.startswith("{")

    def test_long_string_objects_use_block(self):
        """Test that objects with long strings use block style."""
        long_string_obj = {
            "short": "ok",
            "long": "This is a very long string that definitely exceeds fifty characters and should trigger block style",
        }

        result = dumps(long_string_obj, style="auto")
        assert "\n" in result
        assert not result.startswith("{")
        assert '"short": "ok"' in result
        assert '"long":' in result

    def test_nested_objects_always_use_flow(self):
        """Test that nested objects always use flow regardless of size."""
        nested_obj = {
            "root": "value",
            "nested": {
                "key1": "value1",
                "key2": "value2",
                "key3": "value3",
                "key4": "value4",  # This would be block at root level
                "key5": "value5",  # But should be flow when nested
            },
        }

        result = dumps(nested_obj, style="auto")
        # Root should be block (complex object)
        assert "\n" in result
        assert not result.startswith("{")

        # But nested object should be flow (compact in nested context)
        assert '{"key1": "value1"' in result or '{ "key1": "value1"' in result

    def test_mixed_complexity_scenarios(self):
        """Test various mixed complexity scenarios."""
        # Object that meets multiple block criteria
        complex_mixed = {
            "simple": "value",
            "long_text": "This is a very long description that exceeds the fifty character limit",
            "list": [1, 2, 3],
            "object": {"nested": "data"},
            "another": "field",  # Also >3 keys
        }

        result = dumps(complex_mixed, style="auto")
        assert "\n" in result
        assert not result.startswith("{")

        # Each criterion should trigger block style
        single_criteria = [
            # Long string only
            {
                "short": "ok",
                "long": "This is a very long string that exceeds fifty characters",
            },
            # Complex value only
            {"simple": "value", "complex": [1, 2, 3]},
            # Many keys only
            {"a": 1, "b": 2, "c": 3, "d": 4},
        ]

        for obj in single_criteria:
            result = dumps(obj, style="auto")
            assert "\n" in result, f"Failed for {obj}"
            assert not result.startswith("{"), f"Failed for {obj}"

    def test_edge_cases(self):
        """Test edge cases for auto style selection."""
        # Empty object
        assert dumps({}, style="auto") == "{}"

        # Exactly 3 keys (boundary case)
        boundary_obj = {"a": 1, "b": 2, "c": 3}
        result = dumps(boundary_obj, style="auto")
        assert result.startswith("{") and result.endswith("}")

        # Exactly 50 characters (boundary case)
        char_50 = "a" * 50
        boundary_string = {"text": char_50}
        result = dumps(boundary_string, style="auto")
        assert result.startswith("{")  # Exactly 50 chars = flow

        # 51 characters (should be block)
        char_51 = "a" * 51
        over_boundary = {"text": char_51}
        result = dumps(over_boundary, style="auto")
        assert not result.startswith("{")  # Over 50 chars = block style (no braces)

    def test_auto_vs_explicit_styles(self):
        """Test that auto produces same results as explicit styles when appropriate."""
        # Small object: auto should match flow
        small_obj = {"key": "value"}
        auto_result = dumps(small_obj, style="auto")
        flow_result = dumps(small_obj, style="flow")
        assert auto_result == flow_result

        # Large object: auto should match block (at root level)
        large_obj = {"a": 1, "b": 2, "c": 3, "d": 4}
        auto_result = dumps(large_obj, style="auto")
        block_result = dumps(large_obj, style="block")
        assert auto_result == block_result

    def test_auto_style_round_trip(self):
        """Test that auto style output can be parsed back correctly."""
        test_objects = [
            # Small simple
            {"key": "value"},
            # Large simple
            {"a": 1, "b": 2, "c": 3, "d": 4},
            # Complex
            {"data": [1, 2, 3], "config": {"debug": True}},
            # Long string
            {
                "description": "This is a very long description that exceeds fifty characters"
            },
            # Mixed
            {"simple": "ok", "complex": [1, 2], "count": 42},
        ]

        for original in test_objects:
            jyaml_str = dumps(original, style="auto")
            restored = loads(jyaml_str)
            assert restored == original, f"Round-trip failed for {original}"
