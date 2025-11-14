# Markdown Checker

A command-line tool for validating markdown files to ensure they contain only UTF-8 encoded, ASCII-subset characters without unprintable control characters or non-standard tree visualization symbols.

## Purpose

Many developers include non-ASCII characters, particularly tree-drawing symbols (like U+251C, U+2514, U+2502, U+2500), in markdown files when documenting directory structures. While these characters render nicely in some contexts, they can cause issues with:

- Basic text editors and terminals
- CI/CD pipelines expecting ASCII-only content
- Automated documentation systems
- Version control diff tools
- Screen readers and accessibility tools

Markdown Checker helps maintain clean, portable markdown files by detecting and reporting these issues.

## Features

- **UTF-8 Validation**: Ensures files are valid UTF-8 encoded text
- **ASCII Subset Checking**: Detects any character outside the ASCII range (0-127)
- **Unprintable Character Detection**: Finds control characters and other unprintable bytes
- **Tree Symbol Detection**: Specifically identifies box-drawing characters used in directory trees
- **Auto-Fix**: Automatically replaces tree symbols with ASCII equivalents
- **Dry-Run Mode**: Preview fixes before applying them
- **Glob Pattern Support**: Process multiple files using wildcard patterns
- **Detailed Error Reporting**: Shows exact line numbers and columns for violations
- **Verbose Mode**: Optional detailed output showing validation progress
- **Helpful Suggestions**: Provides alternatives for detected violations

## Quick Start

### Installation

Clone the repository and build with Cargo:

```bash
git clone <repository-url>
cd markdown-checker
cargo build --release
```

The binary will be available at `target/release/markdown-checker`.

### Basic Usage

Check the default README.md in the current directory:

```bash
markdown-checker
```

Check a README.md in a specific directory:

```bash
markdown-checker -p docs
```

Check a custom markdown file:

```bash
markdown-checker -f CONTRIBUTING.md
```

Combine path and filename:

```bash
markdown-checker -p docs/api -f overview.md
```

Enable verbose output:

```bash
markdown-checker -v
```

### Auto-Fix Mode (New in v1.1.0)

Automatically fix tree symbol violations:

```bash
markdown-checker --fix
```

Preview fixes before applying them (dry-run):

```bash
markdown-checker --dry-run
# or
markdown-checker -n
```

The `--fix` flag will:
- Replace tree symbols with ASCII equivalents (+, |, -)
- Only work if ALL violations are fixable (tree symbols only)
- Fail with an error if the file contains other Unicode characters (emojis, accents, etc.)
- Verify the fix worked before writing the file

### Glob Pattern Support (New in v1.1.0)

Process multiple files using wildcard patterns:

```bash
# All markdown files in current directory
markdown-checker -f "*.md"

# All markdown files recursively
markdown-checker -f "**/*.md"

# All markdown files in docs directory
markdown-checker -p docs -f "*.md"

# Combine with auto-fix
markdown-checker -f "docs/**/*.md" --fix
```

## Usage Examples

For detailed examples showing actual tool output (including Unicode characters for demonstration purposes), see:
- [Example Output](tests/fixtures/example_output.md) - Shows success/failure output with visual indicators
- [Tree Characters Example](tests/fixtures/tree_chars.md) - Markdown file with box-drawing characters
- [Non-ASCII Example](tests/fixtures/non_ascii.md) - File with various Unicode violations
- [Valid Example](tests/fixtures/valid.md) - Properly formatted ASCII-only markdown

### Example 1: Successful Validation

When a file passes all checks, the tool displays a success message and exits with code 0.

### Example 2: Detecting Tree Symbols

When box-drawing characters are detected (U+251C, U+2514, U+2502, U+2500, etc.), the tool reports:
- The validator that failed (ASCII Subset, Tree Symbols)
- Exact line and column numbers
- The Unicode code point
- Suggested ASCII alternatives ('+', '-', '|', '`')

### Example 3: Verbose Mode

Use the `-v` flag to see detailed progress as each validator runs, including file size and step-by-step results.

### Example 4: CI/CD Integration

Use in a CI/CD pipeline with exit codes:

```bash
#!/bin/bash
markdown-checker || exit 1
echo "Documentation validation passed!"
```

Exit codes:
- `0`: File passes all validations
- `1`: File fails one or more validations
- `2`: Usage error (invalid arguments, file not found)

## Command-Line Options

```
Usage: markdown-checker [OPTIONS]

Options:
  -p, --path <PATH>          Path to directory containing the file [default: .]
  -f, --file-name <NAME>     Name of the file to check or glob pattern [default: README.md]
  -v, --verbose              Enable verbose output
      --fix                  Automatically fix violations where possible (tree symbols only)
  -n, --dry-run              Preview fixes without applying them (dry-run mode)
  -h, --help                 Print help (use --help for extended documentation)
  -V, --version              Print version
```

**Note**: Use `--help` to see extended documentation with detailed usage examples and safety information.

## Validation Rules

### UTF-8 Encoding
- Files must be valid UTF-8 encoded text
- Invalid byte sequences are reported

### ASCII Subset
- All characters must be within ASCII range (code points 0-127)
- Unicode characters (emojis, accents, etc.) are flagged

### Printable Characters
- Only printable ASCII characters allowed (32-126)
- Standard whitespace is permitted:
  - Space (32)
  - Tab (9)
  - Newline (10)
  - Carriage return (13)
- Control characters and null bytes are flagged

### Tree Symbols
Detects common tree visualization characters:
- Box-drawing characters in the Unicode range U+2500 - U+257F
- Common examples: U+251C, U+2514, U+2502, U+2500, U+250C, U+2510, U+2518, U+2524, U+2534, U+252C, U+253C
- Provides ASCII alternatives for each violation

## Suggested Alternatives

Instead of Unicode tree symbols, use standard ASCII characters.

See [tests/fixtures/tree_chars.md](tests/fixtures/tree_chars.md) for an example of invalid tree characters.

ASCII-only alternative (valid):

```
# Good - uses only ASCII:
+ src/
  + main.rs

# Or:
|-- src/
    |-- main.rs
```

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Development Build

```bash
cargo run -- -v
```

### Code Quality

Format code:
```bash
cargo fmt
```

Run linter:
```bash
cargo clippy
```

## Testing

This project follows Test-Driven Development (TDD) practices. See `docs/process.md` for details.

Run the test suite:

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name
```

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- **[Architecture](docs/architecture.md)**: System design and component structure
- **[PRD](docs/prd.md)**: Product requirements and use cases
- **[Design](docs/design.md)**: Detailed technical design and data structures
- **[Process](docs/process.md)**: Development process and TDD methodology
- **[Plan](docs/plan.md)**: Step-by-step implementation plan
- **[Status](docs/status.md)**: Current project status and progress

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Follow TDD practices (write tests first)
4. Ensure all tests pass
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

## License

MIT License - Copyright (c) 2025 Michael A Wright

See [LICENSE](LICENSE) file for details.

## Use Cases

### For Developers
- Validate documentation before commits
- Ensure cross-platform compatibility
- Maintain consistent documentation standards

### For DevOps
- Integrate into CI/CD pipelines
- Enforce documentation quality gates
- Automated pre-merge checks

### For Open Source Maintainers
- Ensure accessible documentation
- Maintain portability across platforms
- Consistent contributor documentation

## FAQ

**Q: Why not allow Unicode in markdown?**
A: While Unicode is valid, ASCII-only content ensures maximum compatibility across tools, platforms, and accessibility devices.

**Q: Can I use this for files other than README.md?**
A: Yes! Use the `-f` flag to check any markdown file.

**Q: What if I have legitimate Unicode content?**
A: This tool is designed for documentation that should be ASCII-only. If your use case requires Unicode, this tool may not be appropriate.

**Q: How do I fix tree symbol violations?**
A: Replace Unicode tree characters with ASCII alternatives like `+`, `-`, `|`, and backticks.

**Q: Can I use this in automated workflows?**
A: Absolutely! The tool returns appropriate exit codes for scripting and CI/CD integration.

## Roadmap

### Completed (v1.1.0)
- [x] Auto-fix mode to replace violations (tree symbols)
- [x] Dry-run mode to preview fixes
- [x] Multiple file processing with glob patterns

### Future Enhancements
- Configuration file support (.markdown-checker.toml)
- Custom validator plugins
- JSON/XML output formats
- Integration with popular markdown linters
- Pre-built binaries for releases
- Homebrew formula for easy installation

## Support

For issues, questions, or contributions, please use the GitHub issue tracker.

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Programming language
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing

---

Made with care by Michael A Wright
