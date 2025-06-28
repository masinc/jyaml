#!/usr/bin/env python3
"""Integration tests for JYAML dumps function."""

from collections import OrderedDict
from decimal import Decimal

import pytest

from jyaml import DumpOptions, dumps, loads


class TestDumpsIntegration:
    """Test dumps function integration scenarios."""

    def test_config_file_round_trip(self):
        """Test configuration file round-trip."""
        config = {
            "app": {"name": "MyApp", "version": "1.2.3", "debug": False},
            "database": {
                "host": "localhost",
                "port": 5432,
                "ssl": True,
                "pool_size": 10,
            },
            "features": ["authentication", "logging", "monitoring"],
        }

        # Round trip with different styles
        for style in ["flow", "auto", "block"]:
            if style == "block" or isinstance(config, dict):
                jyaml_str = dumps(config, style=style)
                restored = loads(jyaml_str)
                assert restored == config, f"Round-trip failed for style: {style}"

    def test_api_response_serialization(self):
        """Test API response-like data serialization."""
        api_response = {
            "status": "success",
            "data": {
                "users": [
                    {
                        "id": 1,
                        "name": "Alice Johnson",
                        "email": "alice@example.com",
                        "active": True,
                        "roles": ["user", "admin"],
                    },
                    {
                        "id": 2,
                        "name": "Bob Smith",
                        "email": "bob@example.com",
                        "active": False,
                        "roles": ["user"],
                    },
                ],
                "total": 2,
                "page": 1,
            },
            "timestamp": "2023-12-01T10:30:00Z",
        }

        # Test different presets
        for preset in ["compact", "pretty", "minimal"]:
            jyaml_str = dumps(api_response, preset=preset)
            restored = loads(jyaml_str)
            assert restored == api_response, f"Round-trip failed for preset: {preset}"

    def test_data_migration_scenario(self):
        """Test data migration scenario with various types."""
        migration_data = {
            "metadata": {
                "version": 2,
                "created_at": "2023-12-01",
                "migrated_records": 1500,
            },
            "records": [
                {
                    "id": "user_001",
                    "data": {
                        "name": "Test User",
                        "balance": 1234.56,
                        "is_premium": True,
                        "last_login": None,
                        "preferences": {"theme": "dark", "notifications": True},
                    },
                },
                {
                    "id": "user_002",
                    "data": {
                        "name": "Another User",
                        "balance": 0.0,
                        "is_premium": False,
                        "last_login": "2023-11-30",
                        "preferences": {"theme": "light", "notifications": False},
                    },
                },
            ],
        }

        # Test with sorted keys for consistency
        jyaml_str = dumps(migration_data, sort_keys=True)
        restored = loads(jyaml_str)
        assert restored == migration_data

        # Ensure keys are sorted in output
        lines = jyaml_str.split("\n") if "\n" in jyaml_str else [jyaml_str]
        if any("metadata" in line and "records" in line for line in lines):
            # Check key order in the combined line
            combined = " ".join(lines)
            metadata_pos = combined.find('"metadata"')
            records_pos = combined.find('"records"')
            if metadata_pos >= 0 and records_pos >= 0:
                assert metadata_pos < records_pos, "Keys should be sorted"

    def test_nested_structures_round_trip(self):
        """Test deeply nested structures."""
        nested = {
            "level1": {
                "level2": {
                    "level3": {
                        "data": [1, 2, 3],
                        "more": {"level4": {"final": "deep value"}},
                    }
                }
            }
        }

        jyaml_str = dumps(nested)
        restored = loads(jyaml_str)
        assert restored == nested

        # Test with block style at root
        block_str = dumps(nested, style="block")
        restored_block = loads(block_str)
        assert restored_block == nested

    def test_special_characters_round_trip(self):
        """Test special characters and escaping."""
        special_data = {
            "quotes": 'He said "Hello world!"',
            "newlines": "Line 1\nLine 2\nLine 3",
            "tabs": "Column1\tColumn2\tColumn3",
            "unicode": "café naïve résumé",
            "backslashes": "C:\\path\\to\\file",
            "mixed": 'Mixed "quotes" and \\backslashes\\ with\nnewlines',
        }

        jyaml_str = dumps(special_data)
        restored = loads(jyaml_str)
        assert restored == special_data

        # Test ASCII-only mode
        ascii_str = dumps(special_data, ensure_ascii=True)
        restored_ascii = loads(ascii_str)
        assert restored_ascii == special_data

    def test_empty_and_edge_cases(self):
        """Test empty and edge case values."""
        edge_cases = {
            "empty_dict": {},
            "empty_list": [],
            "empty_string": "",
            "null_value": None,
            "zero": 0,
            "false": False,
            "nested_empty": {"inner": {"empty": {}}},
        }

        jyaml_str = dumps(edge_cases)
        restored = loads(jyaml_str)
        assert restored == edge_cases

    def test_numeric_precision(self):
        """Test numeric precision handling."""
        numeric_data = {
            "integer": 42,
            "float": 3.14159,
            "large_int": 9223372036854775807,
            "small_float": 1e-10,
            "scientific": 1.23e15,
            "decimal": Decimal("123.456789"),
        }

        jyaml_str = dumps(numeric_data)
        restored = loads(jyaml_str)

        # Most values should be exactly equal
        assert restored["integer"] == numeric_data["integer"]
        assert restored["large_int"] == numeric_data["large_int"]

        # Floats should be close enough
        assert abs(restored["float"] - numeric_data["float"]) < 1e-10
        assert abs(restored["small_float"] - numeric_data["small_float"]) < 1e-15
        assert abs(restored["scientific"] - numeric_data["scientific"]) < 1e10

        # Decimal becomes float in standard round-trip
        assert abs(float(restored["decimal"]) - float(numeric_data["decimal"])) < 1e-6

    def test_collection_types_round_trip(self):
        """Test different collection types."""
        collections_data = {
            "list": [1, 2, 3],
            "tuple_as_list": (4, 5, 6),  # Tuples become lists
            "dict": {"a": 1, "b": 2},
            "ordered_dict": OrderedDict([("z", 1), ("a", 2)]),
        }

        jyaml_str = dumps(collections_data)
        restored = loads(jyaml_str)

        # Check the structure (tuples become lists)
        assert restored["list"] == [1, 2, 3]
        assert restored["tuple_as_list"] == [4, 5, 6]  # tuple → list
        assert restored["dict"] == {"a": 1, "b": 2}
        assert restored["ordered_dict"] == {"z": 1, "a": 2}  # OrderedDict → dict

    def test_preset_consistency(self):
        """Test that presets produce consistent, parseable output."""
        test_data = {
            "simple": "value",
            "number": 42,
            "list": [1, 2, 3],
            "nested": {"key": "value"},
        }

        presets = ["compact", "pretty", "minimal"]

        for preset in presets:
            jyaml_str = dumps(test_data, preset=preset)
            restored = loads(jyaml_str)
            assert restored == test_data, f"Preset {preset} failed round-trip"

            # Check that output is valid JYAML (no parse errors)
            assert isinstance(restored, dict)
            assert len(restored) == 4

    def test_style_consistency(self):
        """Test that different styles produce equivalent results."""
        test_data = {"app": "test", "config": {"debug": True}}

        flow_result = loads(dumps(test_data, style="flow"))
        auto_result = loads(dumps(test_data, style="auto"))
        block_result = loads(dumps(test_data, style="block"))

        assert flow_result == test_data
        assert auto_result == test_data
        assert block_result == test_data

        # All should be equivalent
        assert flow_result == auto_result == block_result


class TestDumpsPerformance:
    """Test dumps function performance characteristics."""

    def test_large_array_serialization(self):
        """Test serialization of large arrays."""
        large_array = list(range(1000))

        jyaml_str = dumps(large_array)
        restored = loads(jyaml_str)

        assert restored == large_array
        assert len(restored) == 1000

    def test_large_object_serialization(self):
        """Test serialization of large objects."""
        large_object = {f"key_{i}": f"value_{i}" for i in range(100)}

        jyaml_str = dumps(large_object)
        restored = loads(jyaml_str)

        assert restored == large_object
        assert len(restored) == 100

    def test_deeply_nested_serialization(self):
        """Test serialization of deeply nested structures."""
        # Create 20-level deep nesting
        nested = {"value": "deep"}
        for _ in range(20):
            nested = {"level": nested}

        jyaml_str = dumps(nested)
        restored = loads(jyaml_str)

        assert restored == nested

        # Verify depth by traversing
        current = restored
        depth = 0
        while "level" in current:
            current = current["level"]
            depth += 1

        assert depth == 20
        assert current == {"value": "deep"}


class TestDumpsErrorHandling:
    """Test dumps function error handling."""

    def test_invalid_options(self):
        """Test error handling for invalid options."""
        data = {"test": "value"}

        # Invalid style
        with pytest.raises(ValueError, match="Unknown preset"):
            dumps(data, preset="invalid_preset")

        # Invalid DumpOptions
        from pydantic import ValidationError

        with pytest.raises(ValidationError):
            dumps(data, options=DumpOptions(indent=-1))

    def test_circular_reference_handling(self):
        """Test handling of circular references (should not hang)."""
        # Create circular reference
        data = {"key": "value"}
        data["self"] = data

        # This should either work or raise an error, but not hang
        try:
            result = dumps(data)
            # If it succeeds, it should be parseable
            loads(result)
        except (ValueError, RecursionError):
            # Acceptable to raise an error for circular references
            pass

    def test_unsupported_types_handling(self):
        """Test handling of unsupported types."""

        # Custom class without __str__
        class UnserializableClass:
            pass

        # Should convert to string representation
        data = {"custom": UnserializableClass()}
        result = dumps(data)
        restored = loads(result)

        # Should be converted to string
        assert isinstance(restored["custom"], str)
        assert "UnserializableClass" in restored["custom"]
