# JYAML Tools

This directory contains standalone tools for working with JYAML files.

## Planned Tools

### CLI Tool (`/cli/`)
Command-line interface for JYAML operations:
- `jyaml validate` - Validate JYAML files for syntax and structure
- `jyaml convert` - Convert between JYAML, JSON, and YAML formats
- `jyaml format` - Format JYAML files with consistent style
- `jyaml minify` - Minimize JYAML files by removing comments and whitespace

### Formatter (`/formatter/`)
Code formatter for JYAML files:
- Consistent indentation (2 spaces by default)
- Smart line wrapping for long values
- Preserve or remove comments based on configuration
- Sort object keys (optional)
- Convert between block and flow styles based on complexity

### Linter (`/linter/`)
Static analysis tool for JYAML best practices:
- Detect common mistakes and anti-patterns
- Enforce naming conventions for keys
- Check for security issues (e.g., extremely large numbers)
- Validate against custom schemas
- Configurable rule sets

### Language Server Protocol (`/lsp/`)
LSP implementation for IDE support:
- Syntax highlighting
- Error diagnostics in real-time
- Auto-completion for keys and values
- Hover information and documentation
- Go to definition for anchors/references
- Formatting and refactoring support

## Implementation Strategy

Tools will be implemented in the following priority order:
1. CLI tool (essential for basic operations)
2. Formatter (improves developer experience)
3. LSP (enables IDE integration)
4. Linter (ensures code quality)

Each tool should:
- Use the core libraries from `/implementations/`
- Provide standalone executables
- Support configuration files
- Include comprehensive documentation
- Follow Unix philosophy (do one thing well)

## Contributing

When implementing a tool:
1. Create a directory under `/tools/`
2. Include a dedicated README with usage examples
3. Provide installation instructions
4. Add tests for all functionality
5. Update this README with the tool's status