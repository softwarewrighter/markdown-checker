# Markdown Checker Wiki

Welcome to the Markdown Checker documentation wiki! This tool validates markdown files for UTF-8 encoding, ASCII-subset compliance, and the absence of unprintable characters.

## Quick Navigation

### Architecture & Design
- **[Architecture Overview](Architecture)** - System architecture with component diagrams
- **[Key Components](Components)** - Detailed breakdown of major components
- **[Workflows & Sequences](Workflows)** - Sequence diagrams showing key operations

### Component Details
- **[CLI Interface](CLI-Interface)** - Command-line argument parsing and configuration
- **[Validation Engine](Validation-Engine)** - Core validation logic and validators
- **[File Operations](File-Operations)** - File reading and processing
- **[Reporter Module](Reporter-Module)** - Output formatting and error reporting
- **[Auto-Fix System](Auto-Fix-System)** - Automatic violation fixing

### Development
- **[Testing Strategy](Testing-Strategy)** - Unit and integration testing approach
- **[Development Process](Development-Process)** - TDD workflow and best practices

## Project Overview

Markdown Checker is a Rust-based CLI tool that helps maintain clean, portable markdown files by detecting and optionally fixing violations such as:

- Invalid UTF-8 encoding
- Non-ASCII Unicode characters
- Unprintable control characters
- Box-drawing tree symbols

### Key Features

- **Multiple Validators**: UTF-8, ASCII subset, unprintable characters, and tree symbols
- **Auto-Fix**: Automatically replace tree symbols with ASCII equivalents
- **Glob Pattern Support**: Process multiple files with wildcard patterns
- **Dry-Run Mode**: Preview fixes before applying them
- **Detailed Reporting**: Line and column numbers for all violations
- **Extensible Design**: Easy to add new validators

### Technology Stack

- **Language**: Rust (2024 edition)
- **CLI Framework**: clap 4.x
- **Build System**: Cargo
- **Testing**: Rust built-in test framework

## Quick Links

- [GitHub Repository](https://github.com/softwarewrighter/markdown-checker)
- [Main README](https://github.com/softwarewrighter/markdown-checker/blob/main/README.md)
- [Architecture Documentation](https://github.com/softwarewrighter/markdown-checker/blob/main/docs/architecture.md)
- [Design Documentation](https://github.com/softwarewrighter/markdown-checker/blob/main/docs/design.md)

## Getting Started

For installation and usage instructions, see the [main README](https://github.com/softwarewrighter/markdown-checker/blob/main/README.md).

For understanding the codebase architecture, start with the [Architecture Overview](Architecture) and [Key Components](Components) pages.

## Documentation Structure

This wiki is organized to provide both high-level overviews and detailed component documentation:

1. **Start Here**: Read the [Architecture Overview](Architecture) to understand the system design
2. **Understand Workflows**: Review [Workflows & Sequences](Workflows) to see how data flows
3. **Deep Dive**: Explore individual component pages for implementation details
4. **Contribute**: Review the [Testing Strategy](Testing-Strategy) and [Development Process](Development-Process)
