# JYAML Specification Version 0.1

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

UTF-8 is supported in JYAML.

## Indent handling

When dealing with Object or Array type block styles or String type multi-line strings, indentation is handled as follows.

- In Object and Array block styles, indentation determines the structure. In the flow style, whitespace is ignored.
- Spaces are used for indentation and tabs are not allowed.
- One or more whitespaces are required after the colon between the key and value of the Object type.

## Comments

JYAML allows comments to be written as in YAML. Comments can be added at the beginning of a line or after a value using a hash sign (`#`). The hash symbol and the rest of the line are treated as comments and do not affect the parsing of JYAML data.



```
# This is a comment.
1 # This is also a comment.
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


### Escaping characters

As with JSON, non-printable characters can be expressed by escaping them with `\`.

The following characters can be escaped in JYAML.

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


### Multi-line strings

Multi-line strings can be written in the same way as YAML format.

You can use `|` to represent a string that preserves newlines.
You can use `>` to represent a string with newlines converted to spaces.

Example:

```
key1: |
  This is a
  multi-line
  string.
key2: >
  This is a
  single line
  string.
```

These are the following data as a single line string:

```
key1: "This is a \nmulti-line \nstring.\n"
key2: "This is a single line string."
```

## Number type

The Number type is a value written in decimal notation without leading zeros.
`. ` to indicate a decimal point.
A leading `+` or `-` symbol can be added.
You can use `e` or `E` for exponentiation.



```
1
+1
-1
1.23
1e2
1e-2
````

## Boolean types

Booleans can be represented by `true` or `false` literals.

YAML `on`, `off`, `yes`, `no` literals are invalid in JYAML.

## Array type

Array type is a type with zero or more values.

Array type can be written in block style or flow style.

Block style can be expressed by indenting each value with a new line.
Flow style is represented by enclosing the list of values in square brackets (`[`, `]`) and separating the values with commas (`,`).


Example block style:

```
- 1
- 2
- 3
```

Example flow style:

```
[ 1, 2, 3 ]
```


## Object type

Object type is a type with zero or more key/value pairs.

Keys and values can be represented as pairs, separated by a colon (`:`).

Keys can only be of type String. Multi-line string expressions are not allowed, and must be represented as a single-line string using single or double quotation marks.

Object types can be written in either block style or flow style.

Block style can be expressed by indenting pairs with newlines.
Flow style is represented by enclosing pairs in braces (`{`, `}`) and separating keys and values with commas (`,`).

Example block style:

```
"a": 1
"b": 2
'c': 3
```

Example flowstyle: 

```
{ "a": 1, "b": 2, 'c': 3 }
```
