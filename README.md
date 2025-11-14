# README Checker

A command-line tool for validating markdown files to ensure they contain only UTF-8 encoded, ASCII-subset characters without unprintable control characters or non-standard tree visualization symbols.

## Purpose

Many developers include non-ASCII characters, particularly tree-drawing symbols (├, └, │, ─), in markdown files when documenting directory structures. While these characters render nicely in some contexts, they can cause issues with:

- Basic text editors and terminals
- CI/CD pipelines expecting ASCII-only content
- Automated documentation systems
- Version control diff tools
- Screen readers and accessibility tools

README Checker helps maintain clean, portable markdown files by detecting and reporting these issues.

## Features

- **UTF-8 Validation**: Ensures files are valid UTF-8 encoded text
- **ASCII Subset Checking**: Detects any character outside the ASCII range (0-127)
- **Unprintable Character Detection**: Finds control characters and other unprintable bytes
- **Tree Symbol Detection**: Specifically identifies box-drawing characters used in directory trees
- **Detailed Error Reporting**: Shows exact line numbers and columns for violations
- **Verbose Mode**: Optional detailed output showing validation progress
- **Helpful Suggestions**: Provides alternatives for detected violations

## Quick Start

### Installation

Clone the repository and build with Cargo:

```bash
git clone <repository-url>
cd readme-checker
cargo build --release
```

The binary will be available at `target/release/readme-checker`.

### Basic Usage

Check the default README.md in the current directory:

```bash
readme-checker
```

Check a README.md in a specific directory:

```bash
readme-checker -p docs
```

Check a custom markdown file:

```bash
readme-checker -f CONTRIBUTING.md
```

Combine path and filename:

```bash
readme-checker -p docs/api -f overview.md
```

Enable verbose output:

```bash
readme-checker -v
```

## Usage Examples

### Example 1: Successful Validation

```bash
$ readme-checker
✓ File validation successful: ./README.md
```

### Example 2: Detecting Tree Symbols

Given a file with tree characters:

```markdown
Project structure:
├── src/
│   └── main.rs
└── tests/
```

Running the checker:

```bash
$ readme-checker
✗ File validation failed: ./README.md

ASCII Subset: ✗ Fail (3 errors)
  Line 2, Column 1: Non-ASCII character: '├' (U+251C)
  Line 3, Column 1: Non-ASCII character: '│' (U+2502)
  Line 3, Column 5: Non-ASCII character: '└' (U+2514)

Tree Symbols: ✗ Fail (3 errors)
  Line 2, Column 1: Tree symbol '├' (U+251C) detected. Use '+' or '|' instead
  Line 3, Column 1: Tree symbol '│' (U+2502) detected. Use '|' instead
  Line 3, Column 5: Tree symbol '└' (U+2514) detected. Use '+' or '`' instead
```

### Example 3: Verbose Mode

```bash
$ readme-checker -v
Checking file: ./README.md
File size: 1,234 bytes

Running validators...
[1/4] UTF-8 Encoding... ✓ Pass
[2/4] ASCII Subset... ✓ Pass
[3/4] Printable Characters... ✓ Pass
[4/4] Tree Symbols... ✓ Pass

✓ File validation successful: ./README.md
```

### Example 4: CI/CD Integration

Use in a CI/CD pipeline with exit codes:

```bash
#!/bin/bash
readme-checker || exit 1
echo "Documentation validation passed!"
```

Exit codes:
- `0`: File passes all validations
- `1`: File fails one or more validations
- `2`: Usage error (invalid arguments, file not found)

## Command-Line Options

```
Usage: readme-checker [OPTIONS]

Options:
  -p, --path <PATH>          Path to directory containing the file [default: .]
  -f, --file-name <NAME>     Name of the file to check [default: README.md]
  -v, --verbose              Enable verbose output
  -h, --help                 Print help
  -V, --version              Print version
```

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
- Box-drawing characters: `├ └ │ ─ ┌ ┐ ┘ ┤ ┴ ┬ ┼`
- Full Unicode box-drawing block (U+2500 - U+257F)
- Provides ASCII alternatives for each violation

## Suggested Alternatives

Instead of Unicode tree symbols, use standard ASCII characters:

```
# Instead of:
├── src/
│   └── main.rs

# Use:
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

Future enhancements being considered:

- Auto-fix mode to replace violations
- Configuration file support
- Multiple file processing (batch mode)
- Custom validator plugins
- JSON/XML output formats
- Integration with popular markdown linters

## Support

For issues, questions, or contributions, please use the GitHub issue tracker.

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Programming language
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing

---

Made with care by Michael A Wright
