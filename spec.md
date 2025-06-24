# JYAML Specification Version 0.3

## Overview

JYAML (JSON-YAML Adaptive Markup Language) is a data format that is upward compatible with JSON and downward compatible with YAML.

Only data that can be expressed in JSON can also be expressed in JYAML.

In other words, all JSON is suitable as JYAML data, and all JYAML data is suitable as YAML data.

JYAML is a data format that can use YAML's block style, multi-line string style, comments, etc. in JSON format.

## Advantages and Purpose of JYAML

JYAML (JSON-YAML Adaptive Markup Language) is a data format that combines the advantages of JSON and YAML, offering the convenience and flexibility of both.

- Low learning costs for those familiar with both JSON and YAML.
- The format provides the readability of YAML while maintaining the simplicity of JSON's structure.
- Single-line comments can be written in `#`.
- Trailing commas are supported for developer convenience and easier editing.
- It supports both JSON and YAML intermediate data structures, allowing for easy conversion between the two formats when exchanging data.
- Since it is upward compatible with JSON, JSON data is also suitable as JYAML data, allowing for a smooth transition.
- Backward compatible with YAML, JYAML data is suitable as YAML data; if your system or application does not take advantage of YAML-specific features, you can use JYAML as is.

## About Version

JYAML version 0.x is the version before official release.

## Extensions

JYAML usually uses one of the following extensions

- jyml
- jyaml

In addition, for interoperation with YAML, the following extensions can be used

- j.yml
- j.yaml

## Encoding

JYAML documents must be encoded in UTF-8 without BOM (Byte Order Mark).

- BOM (0xEF 0xBB 0xBF) at the beginning of the file is not allowed
- Invalid UTF-8 sequences must be rejected with an error
- The parser must not attempt to interpret files with other encodings

Example errors:
- File starting with BOM
- Incomplete multi-byte sequences
- Invalid continuation bytes
- Overlong encodings

## Document Structure

A JYAML document can have any valid JYAML value at the root level:
- Object
- Array
- String
- Number
- Boolean
- Null

This follows the modern JSON specification (RFC 7159 and later) which allows any JSON value at the root level.

Examples of valid JYAML documents:

```
# Object at root
"name": "John"
"age": 30

# Array at root
- 1
- 2
- 3

# String at root
"Hello, World!"

# Number at root
42

# Boolean at root
true

# Null at root
null
```

Note: While JYAML allows any value at the root level, some older JSON parsers may only accept objects or arrays. For maximum compatibility with legacy systems, consider using objects or arrays at the root level.

## Style Mixing Rules

Block and flow styles can be mixed in JYAML with the following rules:

1. **Block style can contain flow style** - Flow collections and values can be used within block structures
2. **Flow style cannot contain block style** - Once inside flow context (brackets or braces), only flow syntax is allowed

Examples:

```
# Valid: Block containing flow
"config":
  "servers": ["web1", "web2", "web3"]
  "database": {"host": "localhost", "port": 5432}
  "features":
    - "feature1"
    - ["sub1", "sub2"]

# Invalid: Flow containing block
{
  "servers":
    - "web1"    # Error! Block style inside flow context
    - "web2"
}

# Invalid: Mixed syntax
{"a": 1, "b": - 2 - 3}  # Error! Block style array inside flow object

# Valid: Proper flow syntax
{"a": 1, "b": [2, 3]}
```

## Indentation Rules

When dealing with Object or Array type block styles or String type multi-line strings, indentation is handled as follows:

- Use spaces for indentation (tabs are not allowed)
- The number of spaces for indentation must be consistent at the same nesting level
- Child elements must be indented more than their parent
- Recommended: 2 spaces per level (but any consistent number is valid)
- In flow style, whitespace is ignored except within strings
- One or more whitespaces are required after the colon between the key and value of the Object type

## Comments

JYAML supports two types of single-line comments:
- `#` - YAML-style comments
- `//` - C-style comments

Comments can be placed at the beginning of a line or after a value. Everything from the comment marker to the end of the line is treated as a comment and does not affect the parsing of JYAML data.

Example:

```
# YAML-style comment
"name": "John"  # inline comment

// C-style comment  
"age": 30  // another inline comment

// Both styles can be used in the same file
"config":
  "timeout": 30  # seconds
  "retries": 3   // maximum attempts
```

Note: Multi-line comments (`/* */`) are not supported in JYAML for simplicity.

### Comment Parsing Rules

Comments are only recognized outside of string contexts:

- Inside quoted strings, `#` and `//` are treated as literal characters
- Comments start only when `#` or `//` appears outside any string
- In multi-line strings, comment markers are preserved as content

Examples:

```
# Valid - comments outside strings
"url": "http://example.com"  # This is a comment
"pattern": "^#\\d+"  // This is also a comment

# String content - not comments
"url": "http://example.com"     # The // in URL is not a comment
"message": "Use # for comments"  # The # is part of the string
'path': 'C:\\dir // not comment' # The // is part of the string

# Multi-line strings preserve comment markers
description: |
  # This is not a comment
  // Neither is this
  http://example.com
```

## Data types

The following data types exist in JYAML.

- String
- Number
- Boolean
- Array
- Object
- Null

These data types are the same as JSON.

## String type

Single-line String values can be enclosed in single or double quotes.

```
"aaa"
'aaa'
```

Unlike YAML, this cannot be expressed without quotation marks.

### String Restrictions

Strings in JYAML follow JSON rules:

- No explicit length limit (implementation-dependent)
- Control characters (U+0000 through U+001F) must be escaped
- Valid escapes for control characters:
  - `\b`, `\f`, `\n`, `\r`, `\t` for common controls
  - `\uXXXX` for any Unicode character including other controls

Examples:
```
# Valid - escaped control characters
"line1\nline2"        # Newline
"name\tvalue"         # Tab  
"text\u0000data"      # NUL character

# Invalid - unescaped control characters
"line1
line2"                # Literal newline not allowed
```


### Escaping characters

JYAML has different escape rules for double-quoted and single-quoted strings.

#### Double-quoted strings

Double-quoted strings follow JSON escape rules. All escape sequences in the table below are valid:

| escape specification | result          |
| -------------------- | --------------- |
| `\"`                 | `"`             |
| `\'`                 | `'`             |
| `\\`                 | `\`             |
| `\/`                 | `/`             |
| `\b`                 | Back Space      |
| `\f`                 | Form Feed       |
| `\n`                 | Line Feed       |
| `\r`                 | Carriage Return |
| `\t`                 | Tab             |
| `\uXXXX`             | Unicode         |

`\uXXXX` can represent any Unicode character by specifying a 4-digit hexadecimal value for `XXXX`.

#### Single-quoted strings

Single-quoted strings have limited escape support:
- `\'` - single quote
- `\\` - backslash
- All other escape sequences are treated literally (not interpreted)

Examples:

```
# Double quotes - full escaping
"Hello\nWorld"        # Hello<newline>World
"Path: \"C:\\temp\""  # Path: "C:\temp"
"Unicode: \u00A9"     # Unicode: ©
"It's fine"          # It's fine

# Single quotes - limited escaping
'Hello\nWorld'        # Hello\nWorld (literal \n)
'can\'t stop'         # can't stop
'Path: C:\\temp'      # Path: C:\temp
'Unicode: \u00A9'     # Unicode: \u00A9 (literal)
```

Note: This differs from YAML, where single quotes are escaped by doubling them (`''`). JYAML uses backslash escaping for consistency with JSON and common programming languages.


### Multi-line strings

Multi-line strings support two styles with optional chomping:

- `|` (literal) - preserves newlines
- `>` (folded) - converts newlines to spaces

Chomping indicators (optional):
- `|-` or `>-` - strip final newline(s)
- `|` or `>` - single newline at end (default)

Examples:

```
# Literal style
key1: |
  Line 1
  Line 2
# Result: "Line 1\nLine 2\n"

key2: |-
  Line 1
  Line 2
# Result: "Line 1\nLine 2"

# Folded style  
key3: >
  This is a
  single line.
# Result: "This is a single line.\n"

key4: >-
  This is a
  single line.
# Result: "This is a single line."
```

Note: The `|+` and `>+` (keep) indicators are not supported in JYAML for simplicity. Leading indentation is automatically stripped based on the first content line.

## Number type

The Number type is a value written in decimal notation without leading zeros.
`.` to indicate a decimal point.
A leading `+` or `-` symbol can be added.
You can use `e` or `E` for exponentiation.

Numbers in JYAML follow the same rules as JSON:
- No explicit range limits (implementation-dependent)
- `Infinity`, `-Infinity`, and `NaN` are not allowed
- Scientific notation is supported (e.g., `1.23e10`)

Examples:

```
1
+1
-1
1.23
1e2
1e-2
```

Note: For maximum interoperability, consider:
- Integers should stay within ±2^53-1 (JavaScript safe integer range)
- Very large or very small numbers may lose precision in some implementations

## Boolean types

Booleans can be represented by `true` or `false` literals.

YAML `on`, `off`, `yes`, `no` literals are invalid in JYAML.

## Array type

Array type is a type with zero or more values.

Array type can be written in block style or flow style.

Block style can be expressed by indenting each value with a new line.
Flow style is represented by enclosing the list of values in square brackets (`[`, `]`) and separating the values with commas (`,`). Optional trailing commas are allowed for convenience.


Example block style:

```
- 1
- 2
- 3
```

Example flow style:

```
[ 1, 2, 3 ]

# Trailing comma allowed
[ 1, 2, 3, ]
```

Example of arrays containing objects:

```
# Block style
- "name": "Alice"
  "age": 30
- "name": "Bob"
  "age": 25

# Flow style with block style objects
[
  {
    "name": "Alice",
    "age": 30
  },
  {
    "name": "Bob",
    "age": 25
  }
]
```


## Object type

Object type is a type with zero or more key/value pairs.

Keys and values can be represented as pairs, separated by a colon (`:`).

Keys can only be of type String. Multi-line string expressions are not allowed, and must be represented as a single-line string using single or double quotation marks.

Object types can be written in either block style or flow style.

Block style can be expressed by indenting pairs with newlines.
Flow style is represented by enclosing pairs in braces (`{`, `}`) and separating keys and values with commas (`,`). Optional trailing commas are allowed for convenience.

Example block style:

```
"a": 1
"b": 2
'c': 3
```

Example flow style: 

```
{ "a": 1, "b": 2, 'c': 3 }

# Trailing comma allowed
{ "a": 1, "b": 2, 'c': 3, }
```

Example of objects containing arrays:

```
# Block style
"users":
  - "Alice"
  - "Bob"
"scores":
  - 95
  - 87

# Mixed styles
{
  "users": ["Alice", "Bob"],
  "scores": [95, 87]
}
```

Example of deeply nested structures:

```
# Complex nested structure
"company":
  "name": "TechCorp"
  "departments":
    - "name": "Engineering"
      "employees":
        - "name": "Alice"
          "skills":
            - "Python"
            - "JavaScript"
        - "name": "Bob"
          "skills":
            - "Java"
            - "Go"
    - "name": "Sales"
      "employees":
        - "name": "Charlie"
          "regions":
            - "North"
            - "South"

# Mixing block and flow styles
"config":
  "servers": ["web1", "web2"]
  "database":
    "host": "localhost"
    "port": 5432
    "options": {"ssl": true, "pool": 10}
```

## Null type

Null values are represented by the literal `null` (case-sensitive).

Example:

```
"key": null
```

YAML's `~` notation and empty values are not valid in JYAML.

## Error Cases

JYAML parsers must detect and report the following error conditions:

### Syntax Errors
- Mismatched quotes: `"name': "value"`
- Unclosed strings: `"name": "value`
- Unclosed collections: `[1, 2, 3`
- Tab characters in indentation
- Inconsistent indentation in block structures

### Type Errors
- Invalid number format: `01234`, `1.2.3`, `123abc`
- Invalid boolean values: `yes`, `no`, `on`, `off` (use `true`/`false` only)
- Invalid null values: `~`, `Null`, `NULL` (use `null` only)
- Non-string keys: `123: "value"`, `null: "value"`

### Structure Errors
- Block syntax inside flow context: `{"items": - 1}`
- Invalid escape in single quotes (except `\'` and `\\`)
- Missing indentation in multi-line strings
- Duplicate keys in the same object

### Examples of Invalid JYAML

```
# Invalid: Mixed indentation
"users":
  - "alice"
   - "bob"      # Error: Inconsistent indentation

# Invalid: Block in flow
{"config": "timeout": 30}  # Error: Missing colon after "config"

# Invalid: Wrong boolean
"active": yes   # Error: Use true/false only

# Invalid: Leading zeros
"count": 0123   # Error: No leading zeros allowed

# Invalid: Tab indentation
"name":	"value"  # Error: Tabs not allowed

# Invalid: Non-string key
123: "value"    # Error: Keys must be strings

# Invalid: Unclosed string
"message": "Hello world  # Error: Missing closing quote
```

## File Structure

A JYAML file contains exactly one JYAML value:

- Multiple documents (YAML's `---` separator) are not supported
- Empty files are invalid - a value is required
- Trailing newline at end of file is optional
- Files should not contain content after the root value except comments

Examples:

```
# Valid single document
"value": 42

# Invalid - empty file
(empty)

# Invalid - multiple documents
"doc1": 1
---
"doc2": 2
```

## Compatibility

- **JSON → JYAML**: All valid JSON is valid JYAML (full compatibility)
- **JYAML → YAML**: Most JYAML is valid YAML, with one exception:
  - Single-quoted strings using `\'` escape syntax (YAML requires `''` for escaping)
  - For full YAML compatibility, use double-quoted strings when escape sequences are needed

Example:
```
# JYAML single-quote escape (not YAML compatible)
'can\'t stop'

# YAML compatible alternatives
"can't stop"    # Use double quotes
'can''t stop'   # Use YAML-style escaping (not valid JYAML)
```
