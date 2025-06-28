#!/usr/bin/env python3
"""Basic JYAML usage examples."""

from jyaml import loads, parse
from jyaml.types import to_python

def main():
    # Basic JSON-style parsing
    print("=== Basic JSON-style parsing ===")
    json_style = '{"name": "JYAML", "version": 0.1, "stable": false}'
    data = loads(json_style)
    print(f"Parsed: {data}")
    print(f"Name: {data['name']}")
    print()
    
    # With comments
    print("=== JSON with comments ===")
    with_comments = '''
    {
      "app": "MyApp", # Application name
      "port": 8080,   # Server port
      "debug": true   # Debug mode
    }
    '''
    data = loads(with_comments)
    print(f"Parsed: {data}")
    print()
    
    # Block-style objects
    print("=== Block-style objects ===")
    block_style = '''
    "name": "JYAML Project"
    "description": "A JSON-YAML hybrid format"
    "features": [
      "json-compatible",
      "yaml-comments",
      "multiline-strings"
    ]
    '''
    data = loads(block_style)
    print(f"Parsed: {data}")
    print()
    
    # Multiline strings
    print("=== Multiline strings ===")
    multiline = '''
    {
      "literal": |
        Line 1
        Line 2
        Line 3
      "folded": >
        This is a long paragraph
        that will be folded
        into a single line
    }
    '''
    data = loads(multiline)
    print(f"Literal: {repr(data['literal'])}")
    print(f"Folded: {repr(data['folded'])}")
    print()
    
    # Get parsed document with metadata
    print("=== Document with metadata ===")
    commented_doc = '''
    # Configuration file
    {
      "server": {
        "host": "localhost", # Local development
        "port": 3000
      }
    }
    '''
    doc = parse(commented_doc)
    print(f"Comments: {doc.comments}")
    print(f"Data: {to_python(doc.data)}")

if __name__ == "__main__":
    main()