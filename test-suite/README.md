# JYAML Test Suite

Common test suite for all JYAML implementations to ensure compatibility and correctness.

## Structure

- `valid/` - Valid JYAML files that should parse successfully
  - `basic/` - Basic data types and structures
  - `complex/` - Complex nested structures
  - `edge-cases/` - Edge cases and boundary conditions
  
- `invalid/` - Invalid JYAML files that should produce errors
  - `syntax/` - Syntax errors (unclosed quotes, brackets, etc.)
  - `structure/` - Structure errors (block in flow, mixed indentation, etc.)
  - `types/` - Type errors (invalid numbers, booleans, etc.)
  
- `expected/` - Expected parse results in JSON format

## Test File Naming

- Valid tests: `<feature>_<variant>.jyml`
- Invalid tests: `<error-type>_<description>.jyml`
- Expected results: Same name with `.json` extension

## Example

```
valid/basic/string_single_quote.jyml  # Input
expected/basic/string_single_quote.json  # Expected output
```

## Running Tests

Each language implementation should:

1. Parse files in `valid/` and compare with `expected/`
2. Attempt to parse files in `invalid/` and verify they produce errors
3. Report pass/fail for each test case