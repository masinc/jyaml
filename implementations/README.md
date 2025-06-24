# JYAML Implementations

This directory contains core library implementations of the JYAML specification in various programming languages. These libraries provide the foundation for the JYAML ecosystem.

## Core Libraries Status

| Language | Status | Version | Package |
|----------|--------|---------|---------|
| Python | Planned | - | - |
| JavaScript/TypeScript | Planned | - | - |
| Go | Planned | - | - |
| Rust | Planned | - | - |

## Implementation Requirements

Each implementation should provide:

1. **Parser** - Convert JYAML text to native data structures
2. **Serializer** - Convert native data structures to JYAML text
3. **Validator** - Check if JYAML text conforms to the specification
4. **Error Handling** - Provide clear, actionable error messages
5. **Streaming API** - Support for large files (optional but recommended)
6. **Type Safety** - Language-appropriate type definitions

## Directory Structure

Each language implementation should follow its language's conventions and best practices. The only requirements are:

- Clear README with usage examples
- Source code implementing parser, serializer, and validator
- Tests including the common test suite
- Standard package/build configuration for the language

## JYAML Ecosystem

The JYAML project includes more than just core libraries:

### Tools (`/tools/`)
- **CLI** - Command-line interface for validation, conversion, and formatting
- **Formatter** - Code formatter for consistent JYAML style
- **Linter** - Static analysis tool for best practices and potential issues
- **LSP** - Language Server Protocol implementation for IDE support

### Editor Support (`/editor-support/`)
- **VS Code** - Extension with syntax highlighting and IntelliSense
- **Vim** - Plugin with syntax support and formatting
- **Emacs** - Major mode for JYAML editing
- **Sublime Text** - Package with full JYAML support

### Documentation (`/docs/`)
- **API Reference** - Detailed API documentation for each language
- **Integration Guides** - How to integrate JYAML into your project
- **Development** - Contributing guidelines and architecture

## Testing

All implementations must:
1. Pass the common test suite in `/test-suite/`
2. Achieve >90% code coverage
3. Include language-specific edge case tests
4. Provide benchmark comparisons with JSON/YAML parsers

## Performance Goals

- Parsing: Within 2x of native JSON parsing speed
- Serialization: Within 1.5x of native JSON serialization
- Memory usage: Comparable to JSON parsers
- Support for streaming large files

## Contributing

When adding a new language implementation:

1. Create a new directory under `implementations/`
2. Implement core functionality (parser, serializer, validator)
3. Ensure all common tests pass
4. Add language-specific tests and benchmarks
5. Document API with examples
6. Update this README with the implementation status

For tools and editor support, see their respective directories.