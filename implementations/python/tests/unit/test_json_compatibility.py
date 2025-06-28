#!/usr/bin/env python3
"""Tests for JSON compatibility with JYAML dumps."""

import json
import pytest
from jyaml import dumps


class TestJSONCompatibility:
    """Test JSON compatibility with JYAML flow style."""
    
    def test_basic_types_json_compatible(self):
        """Test that basic types match JSON output."""
        test_cases = [
            None,
            True, 
            False,
            42,
            3.14,
            "hello",
            [],
            {},
        ]
        
        for obj in test_cases:
            json_result = json.dumps(obj, separators=(',', ':'))
            jyaml_result = dumps(obj, style="flow")
            assert json_result == jyaml_result, f"Mismatch for {obj}: JSON={json_result!r}, JYAML={jyaml_result!r}"
    
    def test_unicode_with_ensure_ascii(self):
        """Test Unicode handling with ensure_ascii option."""
        # Test with 3-byte Unicode (no surrogate pairs needed)
        unicode_obj = {"unicode": "カフェ"}
        
        # With ensure_ascii=True, should escape Unicode
        json_result = json.dumps(unicode_obj, separators=(',', ':'), ensure_ascii=True)
        jyaml_result = dumps(unicode_obj, style="flow", ensure_ascii=True)
        
        # Both should escape Unicode
        assert "\\u" in json_result
        assert "\\u" in jyaml_result
        
        # For 3-byte Unicode, count should match
        assert json_result.count("\\u") == jyaml_result.count("\\u")
        
        # Content should be parseable and equivalent
        assert json.loads(json_result) == json.loads(jyaml_result)
    
    def test_emoji_unicode_escaping_issue(self):
        """Document current issue with 4-byte Unicode (emoji) handling."""
        # This test documents a known issue with JYAML's Unicode escaping
        # JYAML currently generates invalid Unicode escapes for 4-byte characters
        
        emoji_obj = {"emoji": "🚀"}
        
        json_result = json.dumps(emoji_obj, separators=(',', ':'), ensure_ascii=True)
        jyaml_result = dumps(emoji_obj, style="flow", ensure_ascii=True)
        
        # JSON correctly uses surrogate pairs: "🚀" -> "\\ud83d\\ude80"
        assert json_result.count("\\u") == 2  # Surrogate pair
        
        # JYAML incorrectly uses 5-digit hex: "🚀" -> "\\u1f680" 
        # This creates invalid JSON because \u escapes must be exactly 4 hex digits
        assert jyaml_result.count("\\u") == 1
        assert "\\u1f680" in jyaml_result
        
        # The JSON result parses correctly
        json_parsed = json.loads(json_result)
        assert json_parsed["emoji"] == "🚀"
        
        # The JYAML result does NOT parse correctly (becomes garbled)
        jyaml_parsed = json.loads(jyaml_result)
        assert jyaml_parsed["emoji"] != "🚀"  # This is the bug!
    
    def test_string_escaping_compatibility(self):
        """Test string escaping matches JSON."""
        test_strings = [
            "hello\nworld",
            "quote\"test", 
            "tab\ttest",
            "backslash\\test",
            "return\rtest",
        ]
        
        for s in test_strings:
            obj = {"text": s}
            json_result = json.dumps(obj, separators=(',', ':'))
            jyaml_result = dumps(obj, style="flow")
            
            # Extract the escaped string parts
            json_escaped = json_result.split(':"')[1][:-2]  # Remove {"text":" and "}
            jyaml_escaped = jyaml_result.split(': "')[1][:-2]  # Remove {"text": " and "}
            
            assert json_escaped == jyaml_escaped, f"String escaping mismatch for {s!r}"
    
    def test_nested_structure_content(self):
        """Test that nested structures contain same content."""
        nested = {
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ],
            "meta": {
                "version": "1.0",
                "debug": True
            }
        }
        
        json_result = json.dumps(nested, separators=(',', ':'))
        jyaml_result = dumps(nested, style="flow")
        
        # Both should be valid JSON when parsed
        json_parsed = json.loads(json_result)
        jyaml_parsed = json.loads(jyaml_result)
        
        assert json_parsed == jyaml_parsed
    
    def test_round_trip_through_json(self):
        """Test that JYAML flow output can be parsed by JSON."""
        test_cases = [
            {"simple": "value"},
            [1, 2, 3],
            {"nested": {"array": [1, 2], "object": {"key": "value"}}},
            {"unicode": "test"}, # ASCII only for JSON compatibility
            {"escaped": "line1\nline2\ttab"},
        ]
        
        for obj in test_cases:
            jyaml_result = dumps(obj, style="flow")
            
            # Should be parseable by json.loads
            json_parsed = json.loads(jyaml_result)
            assert json_parsed == obj
    
    def test_spacing_differences_documented(self):
        """Document the spacing differences between JSON and JYAML."""
        obj = {"key": "value", "numbers": [1, 2, 3]}
        
        json_result = json.dumps(obj, separators=(',', ':'))
        jyaml_result = dumps(obj, style="flow")
        
        # JSON is more compact
        assert len(json_result) < len(jyaml_result)
        
        # JYAML has spaces after : and ,
        assert ": " in jyaml_result
        assert ", " in jyaml_result
        
        # JSON doesn't 
        assert ": " not in json_result
        assert ", " not in json_result
        
        # But content is equivalent
        assert json.loads(json_result) == json.loads(jyaml_result)


class TestJSONCompatibilityPresetIdea:
    """Test ideas for a JSON-compatible preset."""
    
    def test_current_compact_preset_vs_json(self):
        """Compare current compact preset with JSON."""
        obj = {"key": "value", "numbers": [1, 2, 3]}
        
        json_result = json.dumps(obj, separators=(',', ':'))
        compact_result = dumps(obj, preset="compact")
        
        # Current compact still has spaces
        assert json_result != compact_result
        assert ": " in compact_result  # JYAML still has spaces
        
        # But they parse to the same content
        assert json.loads(json_result) == json.loads(compact_result)
    
    def test_potential_json_preset_behavior(self):
        """Test what a JSON-compatible preset might look like."""
        # This test documents what we'd need to implement
        # for true JSON compatibility
        
        obj = {"key": "value", "array": [1, 2]}
        expected_json = '{"key":"value","array":[1,2]}'
        
        # Currently no preset produces this exact output
        current_compact = dumps(obj, preset="compact")
        assert current_compact != expected_json
        
        # The difference is spacing
        current_without_spaces = current_compact.replace(": ", ":").replace(", ", ",")
        assert current_without_spaces == expected_json