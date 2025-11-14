# Architecture

## Overview

The README Checker is a command-line tool written in Rust that validates markdown files for UTF-8 encoding, ASCII-subset compliance, and the absence of unprintable characters (particularly non-standard tree visualization symbols).

## System Components

### 1. CLI Interface Layer
- **Technology**: `clap` crate for argument parsing
- **Responsibilities**:
  - Parse command-line arguments (`-p/--path`, `-f/--file-name`, `-v/--verbose`)
  - Validate argument combinations
  - Construct file paths from user input
  - Display help and usage information

### 2. File Operations Module
- **Responsibilities**:
  - Locate and read target files
  - Handle file not found errors
  - Read file contents as bytes for validation
  - Report file I/O errors with clear messages

### 3. Validation Engine
The core validation logic is organized into separate, testable validators:

#### UTF-8 Validator
- Validates that file contents are valid UTF-8
- Reports specific byte positions of invalid sequences
- Returns line numbers where violations occur

#### ASCII Subset Validator
- Checks that all characters fall within the ASCII range (0-127)
- Identifies non-ASCII Unicode characters
- Reports line numbers with violations

#### Unprintable Character Detector
- Detects control characters and other unprintable bytes
- Allows exceptions for common whitespace (tabs, newlines, carriage returns)
- Special focus on tree drawing characters like:
  - Box-drawing characters (U+2500 - U+257F)
  - Tree symbols (├, └, │, ─, etc.)

#### Tree Symbol Detector
- Specifically identifies common tree visualization characters
- Reports exact positions and line numbers
- Provides suggestions for standard alternatives

### 4. Reporting Module
- **Responsibilities**:
  - Aggregate validation results
  - Format error messages with line numbers
  - Display verbose output when requested
  - Produce final pass/fail summary

## Data Flow

```
User Input (CLI args)
    ↓
Argument Parser
    ↓
File Path Constructor
    ↓
File Reader
    ↓
Validation Engine
    ├→ UTF-8 Validator
    ├→ ASCII Validator
    ├→ Unprintable Detector
    └→ Tree Symbol Detector
    ↓
Results Aggregator
    ↓
Reporter (console output)
    ↓
Exit Code (0 = success, 1 = failure)
```

## Module Structure

```
src/
├── main.rs              # Entry point, CLI setup
├── cli.rs               # Argument parsing and configuration
├── file_ops.rs          # File reading operations
├── validators/
│   ├── mod.rs           # Validator trait and common types
│   ├── utf8.rs          # UTF-8 validation
│   ├── ascii.rs         # ASCII subset validation
│   ├── unprintable.rs   # Unprintable character detection
│   └── tree_symbols.rs  # Tree symbol detection
├── reporter.rs          # Result formatting and output
└── lib.rs               # Library exports for testing
```

## Error Handling Strategy

### Error Types
1. **File Errors**: File not found, permission denied, I/O errors
2. **Validation Errors**: UTF-8 violations, non-ASCII characters, unprintable chars
3. **Argument Errors**: Invalid combinations, missing required values

### Error Reporting
- All errors include contextual information (file path, line numbers, character positions)
- Errors are accumulated and reported together (fail-fast not required)
- Exit codes: 0 for success, 1 for validation failures, 2 for usage errors

## Testing Strategy

### Unit Tests
- Each validator has comprehensive unit tests
- Tests use example files with known violations
- Edge cases: empty files, binary files, mixed content

### Integration Tests
- End-to-end CLI tests
- Tests for argument parsing combinations
- Verbose output verification

### Test Data
- Sample markdown files in `tests/fixtures/`:
  - `valid.md` - passes all checks
  - `invalid_utf8.md` - UTF-8 violations
  - `non_ascii.md` - contains Unicode
  - `tree_chars.md` - contains tree symbols
  - `mixed.md` - multiple violation types

## Performance Considerations

- Stream-based reading for large files
- Single-pass validation where possible
- Early exit on file errors
- Minimal memory footprint (no full file buffering unless necessary)

## Extensibility

The validator trait allows easy addition of new validation rules:

```rust
pub trait Validator {
    fn validate(&self, content: &str) -> ValidationResult;
}
```

Future validators could check:
- Line length limits
- Markdown syntax compliance
- Link validity
- Image reference existence
