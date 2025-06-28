# JYAML Python Examples

This directory contains practical examples demonstrating how to use the JYAML Python implementation.

## Examples Overview

### 1. `basic_usage.py` - Getting Started
Basic JYAML parsing and feature demonstration:
- JSON-style parsing
- Comments support
- Block-style objects
- Multiline strings
- Document metadata access

```bash
uv run python examples/basic_usage.py
```

### 2. `error_handling.py` - Robust Error Handling
Comprehensive error handling patterns:
- Lexer errors (tabs, unterminated strings, invalid escapes)
- Parser errors (missing values, mismatched brackets)
- Graceful error handling strategies
- Input validation functions

```bash
uv run python examples/error_handling.py
```

### 3. `cli_usage.py` - Command-Line Interface
CLI tool usage examples:
- File parsing and validation
- stdin input processing
- Error handling in CLI context
- Help and usage information

```bash
uv run python examples/cli_usage.py
```

### 4. `type_safety.py` - Type-Safe Data Handling
Using Pydantic models for type safety:
- Configuration validation
- Type error detection
- Partial object validation
- Optional field handling
- Nested object validation

```bash
uv run python examples/type_safety.py
```

## Running Examples

All examples can be run using `uv`:

```bash
# Run all examples
for example in examples/*.py; do
    echo "=== Running $example ==="
    uv run python "$example"
    echo
done

# Run specific example
uv run python examples/basic_usage.py
```

## Key Features Demonstrated

### JYAML Format Features
- **JSON Compatibility**: All valid JSON is valid JYAML
- **Comments**: `#` line comments
- **Block Style**: YAML-like key-value pairs without braces
- **Multiline Strings**: `|` (literal) and `>` (folded) styles
- **Trailing Commas**: Supported in arrays and objects

### Python Implementation Features
- **Type Safety**: Pydantic integration for data validation
- **Error Handling**: Detailed error messages with line/column info
- **CLI Tool**: Command-line parsing and validation
- **Metadata Access**: Comments and parsing information
- **Performance**: Efficient parsing for large documents

## Sample JYAML Files

### Basic Configuration
```jyaml
# Application configuration
{
  "app": {
    "name": "MyApp",
    "version": "1.0.0"
  },
  "server": {
    "host": "localhost", # Development server
    "port": 8080,
    "ssl": false
  }
}
```

### Block Style
```jyaml
"name": "Block Style Example"
"description": |
  This is a multiline
  description that preserves
  line breaks
"features": [
  "json-compatible",
  "yaml-comments", 
  "multiline-strings"
]
```

### Mixed Styles
```jyaml
{
  "metadata": {
    "title": "Mixed Style",
    "tags": ["example", "mixed"]
  },
  "content": |
    Multiline content
    with preserved formatting
  "config": {
    "enabled": true,
    "options": [
      "option1",
      "option2"
    ]
  }
}
```

## Integration Patterns

### Web Application Configuration
```python
from pydantic import BaseModel
from jyaml import loads

class WebConfig(BaseModel):
    host: str
    port: int
    ssl: bool = False

config_jyaml = '''
{
  "host": "0.0.0.0", # Listen on all interfaces
  "port": 8080,
  "ssl": true
}
'''

config = WebConfig(**loads(config_jyaml))
```

### Data Processing Pipeline
```python
from jyaml import parse

# Load configuration with metadata
doc = parse(config_text)
print(f"Configuration loaded with {len(doc.comments)} comments")
data = doc.data.to_python()
```

### CLI Tool Integration
```bash
# Parse and validate
python -m jyaml config.jyml --validate

# Convert to JSON
python -m jyaml config.jyml > config.json

# Process stdin
echo '{"key": "value"}' | python -m jyaml
```

## Error Handling Best Practices

```python
from jyaml import loads
from jyaml.lexer import LexerError
from jyaml.parser import ParseError

def safe_parse(text: str):
    try:
        return loads(text)
    except LexerError as e:
        print(f"Syntax error: {e}")
    except ParseError as e:
        print(f"Structure error: {e}")
    except Exception as e:
        print(f"Unexpected error: {e}")
    return None
```

## Performance Considerations

- Use `loads()` for simple data extraction
- Use `parse()` when you need metadata (comments, positions)
- Validate with Pydantic models for type safety
- Handle large files by streaming if needed

## Further Reading

- [JYAML Specification](../../spec.md)
- [Python Implementation README](../README.md)
- [Test Suite](../tests/)