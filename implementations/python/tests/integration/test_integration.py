"""Integration tests for JYAML implementation."""

import json
import tempfile
import subprocess
import sys
from pathlib import Path
import pytest

from jyaml import parse, loads


class TestGeneralIntegration:
    """General integration tests (non-test-suite specific)."""
    
    def test_basic_parsing_capability(self):
        """Test that our parser can handle basic JYAML structures."""
        test_cases = [
            ('null', None),
            ('true', True),
            ('false', False),
            ('42', 42),
            ('"hello"', "hello"),
            ('[]', []),
            ('{}', {}),
            ('[1, 2, 3]', [1, 2, 3]),
            ('{"key": "value"}', {"key": "value"}),
        ]
        
        for jyaml_str, expected in test_cases:
            try:
                result = loads(jyaml_str)
                assert result == expected, f"Failed for {jyaml_str}: got {result}, expected {expected}"
            except Exception as e:
                pytest.fail(f"Failed to parse basic case '{jyaml_str}': {e}")
    
    def test_parsing_robustness(self):
        """Test parser robustness with various inputs."""
        # These should all parse successfully
        robust_cases = [
            '# Comment only\n42',
            '{\n  "key": "value"\n}',
            '[\n  1,\n  2,\n  3\n]',
            '{"trailing": "comma",}',
            '[1, 2, 3,]',
        ]
        
        for case in robust_cases:
            try:
                result = parse(case)
                assert result is not None
                python_data = loads(case)
                # python_data can be None for null, which is valid
            except Exception as e:
                pytest.fail(f"Failed robust parsing for: {case!r} - {e}")
    
    def test_error_handling(self):
        """Test that invalid inputs properly raise errors."""
        invalid_cases = [
            '{"invalid": }',  # Missing value
            '[1, 2,}',        # Mismatched brackets
            '"unclosed string',  # Unterminated string
            'invalid_identifier',  # Unknown identifier
        ]
        
        for invalid_case in invalid_cases:
            with pytest.raises(Exception):
                parse(invalid_case)
    


class TestComplexDocuments:
    """Test complex real-world-like documents."""
    
    def test_nested_configuration(self):
        """Test a complex configuration-like document."""
        config = '''
{
  "server": {
    "host": "localhost",
    "port": 8080,
    "ssl": true
  },
  "database": {
    "type": "postgresql",
    "connection": {
      "host": "db.example.com",
      "port": 5432,
      "credentials": {
        "username": "admin",
        "password": "secret123"
      }
    }
  },
  "features": [
    "authentication",
    "logging",
    "monitoring"
  ],
  "settings": {
    "debug": false,
    "timeout": 30.5,
    "max_connections": 100
  }
}
'''
        result = loads(config)
        
        assert result["server"]["host"] == "localhost"
        assert result["server"]["port"] == 8080
        assert result["server"]["ssl"] is True
        assert result["database"]["type"] == "postgresql"
        assert len(result["features"]) == 3
        assert result["settings"]["timeout"] == 30.5
    
    def test_block_style_document(self):
        """Test a block-style document."""
        document = '''
"name": "JYAML Project"
"version": "0.1.0"
"description": |
  A JSON-YAML hybrid format that combines
  the best of both worlds
"authors": [
  "masinc"
]
"dependencies": {
  "pydantic": ">=2.0.0",
  "pytest": ">=7.0.0"
}
"features": [
  "json-compatible",
  "yaml-style-comments",
  "multiline-strings"
]
'''
        result = loads(document)
        
        assert result["name"] == "JYAML Project"
        assert result["version"] == "0.1.0"
        assert "JSON-YAML hybrid" in result["description"]
        assert "masinc" in result["authors"]
        assert "json-compatible" in result["features"]
    
    def test_mixed_styles(self):
        """Test document with mixed flow and block styles."""
        mixed = '''
{
  "metadata": {
    "title": "Mixed Style Example",
    "tags": ["example", "mixed", "styles"]
  },
  "content": |
    This is a multiline string
    that preserves line breaks
    and formatting
  "items": [
    {"id": 1, "name": "First"},
    {"id": 2, "name": "Second"}
  ],
  "config": {
    "enabled": true,
    "options": [
      "option1",
      "option2"
    ]
  }
}
'''
        result = loads(mixed)
        
        assert result["metadata"]["title"] == "Mixed Style Example"
        assert len(result["metadata"]["tags"]) == 3
        assert "line breaks" in result["content"]
        assert len(result["items"]) == 2
        assert result["items"][0]["id"] == 1
        assert result["config"]["enabled"] is True
    
    def test_comments_preservation(self):
        """Test that comments are properly handled."""
        commented = '''
# Main configuration
{
  "app": {
    "name": "MyApp", # Application name
    "version": "1.0.0"
  },
  # Database settings
  "database": {
    "host": "localhost", # Local development
    "port": 5432
  }
}
'''
        # Parse and check that it works despite comments
        result = loads(commented)
        
        assert result["app"]["name"] == "MyApp"
        assert result["app"]["version"] == "1.0.0"
        assert result["database"]["host"] == "localhost"
        assert result["database"]["port"] == 5432
        
        # Check that comments are captured
        parsed_doc = parse(commented)
        assert len(parsed_doc.comments) > 0
        assert any("Main configuration" in comment for comment in parsed_doc.comments)
    
    def test_edge_cases(self):
        """Test various edge cases."""
        edge_cases = [
            # Empty structures
            ('{}', {}),
            ('[]', []),
            
            # Trailing commas
            ('{"a": 1,}', {"a": 1}),
            ('[1, 2, 3,]', [1, 2, 3]),
            
            # Scientific notation
            ('{"value": 1.23e-4}', {"value": 1.23e-4}),
            
            # Unicode strings
            ('{"unicode": "こんにちは世界"}', {"unicode": "こんにちは世界"}),
            
            # Escape sequences
            ('{"escaped": "line1\\nline2\\ttab"}', {"escaped": "line1\nline2\ttab"}),
        ]
        
        for jyaml_str, expected in edge_cases:
            result = loads(jyaml_str)
            assert result == expected, f"Failed for: {jyaml_str}"


class TestCLIIntegration:
    """Test CLI tool integration."""
    
    def test_cli_parse_valid_file(self):
        """Test CLI parsing of valid file."""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.jyml', delete=False) as f:
            f.write('{"message": "hello", "count": 42}')
            temp_file = f.name
        
        try:
            # Test parsing
            result = subprocess.run([
                sys.executable, "-m", "jyaml", temp_file
            ], capture_output=True, text=True, cwd=Path(__file__).parent.parent)
            
            assert result.returncode == 0
            output = json.loads(result.stdout)
            assert output["message"] == "hello"
            assert output["count"] == 42
            
        finally:
            Path(temp_file).unlink()
    
    def test_cli_validate_valid_file(self):
        """Test CLI validation of valid file."""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.jyml', delete=False) as f:
            f.write('{"valid": true}')
            temp_file = f.name
        
        try:
            result = subprocess.run([
                sys.executable, "-m", "jyaml", "--validate", temp_file
            ], capture_output=True, text=True, cwd=Path(__file__).parent.parent)
            
            assert result.returncode == 0
            assert "Valid JYAML" in result.stdout
            
        finally:
            Path(temp_file).unlink()
    
    def test_cli_invalid_file(self):
        """Test CLI with invalid file."""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.jyml', delete=False) as f:
            f.write('{"invalid": }')  # Invalid JSON/JYAML
            temp_file = f.name
        
        try:
            result = subprocess.run([
                sys.executable, "-m", "jyaml", temp_file
            ], capture_output=True, text=True, cwd=Path(__file__).parent.parent)
            
            assert result.returncode != 0
            assert "Parse error" in result.stderr
            
        finally:
            Path(temp_file).unlink()
    
    def test_cli_stdin(self):
        """Test CLI reading from stdin."""
        input_data = '{"from": "stdin", "works": true}'
        
        result = subprocess.run([
            sys.executable, "-m", "jyaml"
        ], input=input_data, capture_output=True, text=True, 
           cwd=Path(__file__).parent.parent)
        
        assert result.returncode == 0
        output = json.loads(result.stdout)
        assert output["from"] == "stdin"
        assert output["works"] is True


class TestRoundTrip:
    """Test round-trip compatibility."""
    
    def test_json_to_jyaml_round_trip(self):
        """Test that valid JSON can be parsed as JYAML."""
        json_data = {
            "string": "value",
            "number": 42,
            "float": 3.14,
            "bool": True,
            "null": None,
            "array": [1, 2, 3],
            "object": {"nested": "value"}
        }
        
        json_str = json.dumps(json_data)
        parsed_data = loads(json_str)
        
        assert parsed_data == json_data
    
    def test_jyaml_with_comments_to_clean_json(self):
        """Test that JYAML with comments produces clean JSON output."""
        jyaml_with_comments = '''
# Configuration file
{
  "app": "MyApp", # Application name
  "version": "1.0.0",
  # Settings
  "debug": false
}
'''
        result = loads(jyaml_with_comments)
        
        # Should parse cleanly without comments in data
        expected = {
            "app": "MyApp",
            "version": "1.0.0", 
            "debug": False
        }
        
        assert result == expected
    
    def test_multiline_string_formatting(self):
        """Test multiline string behavior."""
        literal_test = '''
{
  "literal": |
    Line 1
    Line 2
    Line 3
  "folded": >
    This is a long line
    that should be folded
    into a single line
}
'''
        result = loads(literal_test)
        
        # Literal should preserve line breaks
        assert "Line 1\nLine 2\nLine 3" in result["literal"]
        
        # Folded should create single line  
        assert result["folded"].count('\n') < result["literal"].count('\n')


class TestPerformance:
    """Basic performance tests."""
    
    def test_large_array_parsing(self):
        """Test parsing of large arrays."""
        large_array = "[" + ", ".join(str(i) for i in range(1000)) + "]"
        
        result = loads(large_array)
        
        assert len(result) == 1000
        assert result[0] == 0
        assert result[999] == 999
    
    def test_deep_nesting(self):
        """Test parsing of deeply nested structures."""
        # Create nested structure
        nested = "{}"
        for i in range(50):
            nested = f'{{"level{i}": {nested}}}'
        
        result = loads(nested)
        
        # Navigate to deepest level
        current = result
        for i in range(50):
            current = current[f"level{49-i}"]  # Reverse order since we built from inside out
        
        assert current == {}
    
    def test_many_keys_object(self):
        """Test parsing object with many keys."""
        many_keys = "{" + ", ".join(f'"key{i}": {i}' for i in range(100)) + "}"
        
        result = loads(many_keys)
        
        assert len(result) == 100
        assert result["key0"] == 0
        assert result["key99"] == 99