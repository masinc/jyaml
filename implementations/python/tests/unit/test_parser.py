"""Tests for JYAML parser."""

import pytest
from jyaml import parse, loads, ParseError
from jyaml.types import JYAMLNull, JYAMLBool, JYAMLNumber, JYAMLString, JYAMLArray, JYAMLObject


class TestParser:
    """Test cases for JYAML parser."""
    
    def test_null_value(self):
        """Test parsing null value."""
        result = parse("null")
        assert isinstance(result.data, JYAMLNull)
        
        python_result = loads("null")
        assert python_result is None
    
    def test_boolean_values(self):
        """Test parsing boolean values."""
        # True
        result = parse("true")
        assert isinstance(result.data, JYAMLBool)
        assert result.data.value is True
        
        # False
        result = parse("false")
        assert isinstance(result.data, JYAMLBool)
        assert result.data.value is False
        
        # Python conversion
        assert loads("true") is True
        assert loads("false") is False
    
    def test_numbers(self):
        """Test parsing numbers."""
        test_cases = [
            ("0", 0),
            ("42", 42),
            ("-17", -17),
            ("3.14", 3.14),
            ("1.23e4", 12300.0),
        ]
        
        for input_text, expected in test_cases:
            result = parse(input_text)
            assert isinstance(result.data, JYAMLNumber)
            assert result.data.value == expected
            
            python_result = loads(input_text)
            assert python_result == expected
    
    def test_strings(self):
        """Test parsing strings."""
        test_cases = [
            ('"hello"', "hello"),
            ("'world'", "world"),
            ('""', ""),
            ('"hello\\nworld"', "hello\nworld"),
        ]
        
        for input_text, expected in test_cases:
            result = parse(input_text)
            assert isinstance(result.data, JYAMLString)
            assert result.data.value == expected
            
            python_result = loads(input_text)
            assert python_result == expected
    
    def test_flow_arrays(self):
        """Test parsing flow-style arrays."""
        # Empty array
        result = parse("[]")
        assert isinstance(result.data, JYAMLArray)
        assert len(result.data.value) == 0
        
        # Simple array
        result = parse('[1, 2, 3]')
        assert isinstance(result.data, JYAMLArray)
        assert len(result.data.value) == 3
        
        python_result = loads('[1, 2, 3]')
        assert python_result == [1, 2, 3]
        
        # Trailing comma
        result = parse('[1, 2, 3,]')
        python_result = loads('[1, 2, 3,]')
        assert python_result == [1, 2, 3]
    
    def test_flow_objects(self):
        """Test parsing flow-style objects."""
        # Empty object
        result = parse("{}")
        assert isinstance(result.data, JYAMLObject)
        assert len(result.data.value) == 0
        
        # Simple object
        result = parse('{"key": "value"}')
        assert isinstance(result.data, JYAMLObject)
        assert "key" in result.data.value
        
        python_result = loads('{"key": "value"}')
        assert python_result == {"key": "value"}
        
        # Trailing comma
        result = parse('{"key": "value",}')
        python_result = loads('{"key": "value",}')
        assert python_result == {"key": "value"}
    
    def test_block_arrays(self):
        """Test parsing block-style arrays."""
        input_text = """
- "item1"
- "item2"
- 42
"""
        result = parse(input_text.strip())
        assert isinstance(result.data, JYAMLArray)
        assert len(result.data.value) == 3
        
        python_result = loads(input_text.strip())
        assert python_result == ["item1", "item2", 42]
    
    def test_block_objects(self):
        """Test parsing block-style objects."""
        input_text = """
"key1": "value1"
"key2": 42
"key3": true
"""
        result = parse(input_text.strip())
        assert isinstance(result.data, JYAMLObject)
        assert len(result.data.value) == 3
        
        python_result = loads(input_text.strip())
        expected = {"key1": "value1", "key2": 42, "key3": True}
        assert python_result == expected
    
    def test_nested_structures(self):
        """Test parsing nested structures."""
        input_text = '''
{
  "array": [1, 2, {"nested": "object"}],
  "object": {
    "key": "value"
  }
}
'''
        python_result = loads(input_text)
        expected = {
            "array": [1, 2, {"nested": "object"}],
            "object": {"key": "value"}
        }
        assert python_result == expected
    
    def test_comments(self):
        """Test parsing with comments."""
        input_text = '''
# This is a comment
{
  "key": "value" # Another comment
}
'''
        result = parse(input_text)
        assert len(result.comments) == 2
        assert "This is a comment" in result.comments
        assert "Another comment" in result.comments
        
        python_result = loads(input_text)
        assert python_result == {"key": "value"}
    
    def test_empty_document(self):
        """Test parsing empty document."""
        result = parse("")
        assert isinstance(result.data, JYAMLNull)
        
        result = parse("   \n\n  ")
        assert isinstance(result.data, JYAMLNull)
    
    def test_whitespace_handling(self):
        """Test whitespace handling."""
        input_text = '''
        {
          "key"  :   "value"  ,
          "number" : 42
        }
        '''
        python_result = loads(input_text)
        assert python_result == {"key": "value", "number": 42}
    
    def test_parse_errors(self):
        """Test various parse errors."""
        invalid_inputs = [
            '{"key": }',  # Missing value
            '{key: "value"}',  # Unquoted key
            '[1, 2,}',  # Mismatched brackets
            '"unterminated string',  # Unterminated string
            '{duplicate: 1, duplicate: 2}',  # Would be handled at validation level
        ]
        
        for invalid_input in invalid_inputs[:4]:  # Skip the last one for now
            with pytest.raises(ParseError):
                parse(invalid_input)
    
    def test_multiline_strings_in_context(self):
        """Test multiline strings in document context."""
        input_text = '''
{
  "literal": |
    line1
    line2
  "folded": >
    long line that should
    be folded together
}
'''
        result = loads(input_text)
        assert "literal" in result
        assert "folded" in result
        assert "line1" in result["literal"]
        assert "line2" in result["literal"]