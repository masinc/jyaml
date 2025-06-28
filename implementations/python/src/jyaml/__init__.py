"""JYAML (JSON-YAML Adaptive Markup Language) parser for Python."""

from .parser import parse, Parser, ParseError
from .loader import loads, loads_strict, loads_permissive, loads_fast, loads_ordered
from .options import ParseOptions, LoadOptions, DumpOptions, JYAMLMode
from .dumper import dumps
from .types import (
    JYAMLData,
    JYAMLNull,
    JYAMLBool,
    JYAMLNumber,
    JYAMLString,
    JYAMLArray,
    JYAMLObject,
    ParsedDocument,
    to_python,
    from_python,
)
from .lexer import Lexer, Token, TokenType, LexerError

__all__ = [
    "parse",
    "loads",
    "dumps",
    "ParseOptions",
    "LoadOptions",
    "DumpOptions",
    "JYAMLMode",
    "loads_strict",
    "loads_permissive",
    "loads_fast",
    "loads_ordered",
    "JYAMLData",
    "JYAMLNull",
    "JYAMLBool",
    "JYAMLNumber",
    "JYAMLString",
    "JYAMLArray",
    "JYAMLObject",
    "ParsedDocument",
    "to_python",
    "from_python",
    "Lexer",
    "Token",
    "TokenType",
    "LexerError",
    "Parser",
    "ParseError",
]


def main():
    """CLI entry point."""
    import sys
    import argparse
    import json

    parser = argparse.ArgumentParser(description="JYAML parser")
    parser.add_argument("file", nargs="?", help="JYAML file to parse")
    parser.add_argument("--validate", action="store_true", help="Validate only")
    parser.add_argument("--json", action="store_true", help="Output in JSON format")

    args = parser.parse_args()

    if args.file:
        try:
            with open(args.file, "r", encoding="utf-8") as f:
                content = f.read()
        except IOError as e:
            print(f"Error reading file: {e}", file=sys.stderr)
            sys.exit(1)
    else:
        content = sys.stdin.read()

    try:
        if args.validate:
            parse(content)
            print("Valid JYAML")
        else:
            data = loads(content)
            if args.json:
                print(json.dumps(data, indent=2, ensure_ascii=False))
            else:
                print(dumps(data, style="block", indent=2))
    except (LexerError, ParseError) as e:
        print(f"Parse error: {e}", file=sys.stderr)
        sys.exit(1)
