# JYAML Python Implementation

A Python implementation of JYAML (JSON-YAML Adaptive Markup Language) using modern Python tools.

## Features

- **Complete JYAML support**: All data types, flow/block styles, comments, multiline strings
- **Type-safe**: Built with Pydantic for robust data validation
- **CLI tool**: Command-line interface for parsing and validation
- **Comprehensive tests**: Unit tests and integration tests
- **Modern tooling**: Uses `uv` for package management, `ruff` for linting

## Installation

```bash
# Clone the repository
git clone https://github.com/masinc/jyaml.git
cd jyaml/implementations/python

# Install with uv
uv sync --dev
```

## Usage

### As a Library

```python
import jyaml

# Parse JYAML text
data = jyaml.loads('{"key": "value", "number": 42}')
print(data)  # {'key': 'value', 'number': 42}

# Parse with full document info (including comments)
doc = jyaml.parse('''
# Configuration
{
  "app": "MyApp",  # Application name
  "debug": true
}
''')
print(doc.data)      # Parsed data
print(doc.comments)  # ['Configuration', 'Application name']
```

### CLI Tool

```bash
# Parse a file
uv run python -m jyaml file.jyml

# Validate a file
uv run python -m jyaml --validate file.jyml

# Parse from stdin
echo '{"test": true}' | uv run python -m jyaml
```

## Development

### Running Tests

```bash
# Run unit tests only (fast)
uv run pytest tests/unit/ -v

# Run integration tests only
uv run pytest tests/integration/ -v

# Run all tests
uv run pytest tests/ -v
```

### Code Quality

```bash
# Linting
uv run ruff check src/ tests/

# Fix auto-fixable issues
uv run ruff check src/ tests/ --fix
```

## Project Structure

```
src/jyaml/
   __init__.py     # Public API
   __main__.py     # CLI entry point
   lexer.py        # Tokenization
   parser.py       # Parsing logic
   types.py        # Pydantic data models

tests/
   test_lexer.py      # Lexer unit tests
   test_parser.py     # Parser unit tests
   test_integration.py # Integration tests
```

## Test Categories

- **Unit Tests** (`tests/unit/`): Test individual components (lexer, parser)
- **Integration Tests** (`tests/integration/`): Test real-world scenarios, CLI, performance

Directory-based test organization allows for easy selective testing.

## Requirements

- Python 3.12+
- pydantic>=2.11.7
- pytest>=8.4.1 (dev)
- ruff>=0.12.1 (dev)