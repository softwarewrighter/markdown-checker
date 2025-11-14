# Implementation Plan

## Overview
This document outlines the step-by-step plan to implement the README Checker CLI tool following Test-Driven Development practices.

## Phase 1: Project Setup & Documentation ✓

### Tasks
- [x] Create project documentation
  - [x] architecture.md
  - [x] prd.md
  - [x] design.md
  - [x] process.md
  - [x] plan.md (this file)
  - [x] status.md
- [x] Create MIT LICENSE
- [x] Create comprehensive README.md
- [ ] Update Cargo.toml with dependencies
- [ ] Create initial project structure

### Acceptance Criteria
- All documentation files created and complete
- README includes purpose, quickstart, usage examples
- LICENSE file with correct copyright
- Cargo.toml includes clap dependency

## Phase 2: Core Data Structures (TDD)

### RED: Write Tests
Create `src/lib.rs` with core type tests:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_validation_error_creation() { /* ... */ }

    #[test]
    fn test_validation_result_pass() { /* ... */ }

    #[test]
    fn test_validation_result_fail() { /* ... */ }
}
```

Expected: Tests FAIL (types don't exist yet)

### GREEN: Implement Types
Create in `src/lib.rs`:
- `ValidationStatus` enum
- `ValidationError` struct
- `ValidationResult` struct
- `Validator` trait

Expected: Tests PASS

### REFACTOR
- Add documentation comments
- Implement Display traits
- Add helper methods

### Acceptance Criteria
- All type tests passing
- Types fully documented
- Clean, idiomatic Rust code

## Phase 3: CLI Argument Parsing (TDD)

### RED: Write Tests
Create `tests/cli_tests.rs`:
```rust
#[test]
fn test_default_arguments() { /* ... */ }

#[test]
fn test_custom_path() { /* ... */ }

#[test]
fn test_custom_filename() { /* ... */ }

#[test]
fn test_verbose_flag() { /* ... */ }
```

Expected: Tests FAIL (CLI not implemented)

### GREEN: Implement CLI
Create `src/cli.rs`:
- Define `Cli` struct with clap derives
- Implement argument parsing
- Create `Config` from CLI args

Update `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

Expected: Tests PASS

### REFACTOR
- Improve help text
- Add argument validation
- Clean up code

### Acceptance Criteria
- CLI tests passing
- Arguments parsed correctly
- Help text is clear
- clap integration working

## Phase 4: File Operations (TDD)

### RED: Write Tests
Create `tests/file_ops_tests.rs`:
```rust
#[test]
fn test_read_existing_file() { /* ... */ }

#[test]
fn test_file_not_found() { /* ... */ }

#[test]
fn test_invalid_utf8() { /* ... */ }
```

Create test fixtures in `tests/fixtures/`:
- `valid.txt`
- `invalid_utf8.bin`

Expected: Tests FAIL (file ops not implemented)

### GREEN: Implement File Operations
Create `src/file_ops.rs`:
- `read_file_content()` function
- Error type for file operations
- UTF-8 conversion handling

Expected: Tests PASS

### REFACTOR
- Improve error messages
- Add path utilities
- Optimize reading

### Acceptance Criteria
- File operations tests passing
- Handles all error cases
- Clear error messages

## Phase 5: ASCII Validator (TDD)

### RED: Write Tests
Create `src/validators/ascii.rs` with tests:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_pure_ascii_passes() {
        let validator = AsciiValidator;
        let result = validator.validate("Hello, World!");
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_unicode_emoji_fails() {
        let validator = AsciiValidator;
        let result = validator.validate("Hello 👋");
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_accented_chars_fail() {
        let validator = AsciiValidator;
        let result = validator.validate("café");
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].line_number, 1);
    }

    #[test]
    fn test_multiple_violations() {
        let validator = AsciiValidator;
        let result = validator.validate("Line 1: café\nLine 2: naïve\nLine 3: 日本語");
        assert!(result.errors.len() > 0);
    }
}
```

Expected: Tests FAIL (validator not implemented)

### GREEN: Implement ASCII Validator
In `src/validators/ascii.rs`:
- Create `AsciiValidator` struct
- Implement `Validator` trait
- Check char codes <= 127
- Collect line numbers and violations

Expected: Tests PASS

### REFACTOR
- Optimize character checking
- Improve error messages
- Add character code display

### Acceptance Criteria
- ASCII validator tests passing
- Detects all non-ASCII characters
- Reports correct line numbers
- Shows helpful error messages

## Phase 6: Unprintable Character Detector (TDD)

### RED: Write Tests
In `src/validators/unprintable.rs`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_allowed_whitespace() {
        let content = "Line 1\nLine 2\tTabbed\r\n";
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_null_byte_detected() {
        let content = "Hello\0World";
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_control_chars() {
        let content = "Hello\x07World"; // Bell character
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert_eq!(result.errors.len(), 1);
    }
}
```

Expected: Tests FAIL

### GREEN: Implement Unprintable Validator
- Create `UnprintableValidator`
- Define printable character set (32-126 + whitespace)
- Detect violations
- Report line numbers

Expected: Tests PASS

### REFACTOR
- Clarify printable definitions
- Improve detection logic
- Better error messages

### Acceptance Criteria
- Unprintable validator tests passing
- Allows standard whitespace
- Detects control characters
- Clear violation reporting

## Phase 7: Tree Symbol Detector (TDD)

### RED: Write Tests
In `src/validators/tree_symbols.rs`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_no_tree_symbols_passes() {
        let content = "Project structure:\n+ src/\n  + main.rs\n";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_detects_common_tree_chars() {
        let content = "├── src/\n│   └── main.rs\n";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert_eq!(result.errors.len(), 3); // ├, │, └
    }

    #[test]
    fn test_provides_suggestions() {
        let content = "├── file";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.errors[0].message.contains("instead"));
    }

    #[test]
    fn test_box_drawing_range() {
        // Test characters in U+2500 - U+257F range
        let content = "─━│┃┄┅┆┇";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.errors.len() > 0);
    }
}
```

Expected: Tests FAIL

### GREEN: Implement Tree Symbol Validator
- Create `TreeSymbolValidator`
- Define tree symbol set
- Implement detection
- Generate suggestions

Expected: Tests PASS

### REFACTOR
- Complete symbol coverage
- Better suggestion logic
- Optimize detection

### Acceptance Criteria
- Tree symbol tests passing
- Detects all box-drawing chars
- Provides helpful suggestions
- Accurate line/column reporting

## Phase 8: UTF-8 Validator (TDD)

### RED: Write Tests
Create test fixture with invalid UTF-8:
```bash
# Create a file with invalid UTF-8 bytes
echo -e "Valid line 1\n\xFF\xFE Invalid bytes\nValid line 3" > tests/fixtures/invalid_utf8.bin
```

In `src/validators/utf8.rs`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_utf8() {
        // UTF-8 validation happens during file read
        // This test ensures the validator reports success
    }
}
```

Expected: Tests FAIL or need file ops adjustment

### GREEN: Implement UTF-8 Validation
- Validate during file read
- Report UTF-8 errors clearly
- Include file path in error

Expected: Tests PASS

### REFACTOR
- Improve error context
- Better error messages

### Acceptance Criteria
- UTF-8 validation working
- Clear error messages
- Integration with file reading

## Phase 9: Validation Pipeline (TDD)

### RED: Write Tests
In `src/validators/mod.rs`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_run_all_validators() {
        let content = "valid content";
        let results = validate_all(content);
        assert_eq!(results.len(), 3); // ASCII, Unprintable, TreeSymbols
    }

    #[test]
    fn test_aggregates_all_failures() {
        let content = "├── café";
        let results = validate_all(content);
        let failed: Vec<_> = results.iter()
            .filter(|r| !r.errors.is_empty())
            .collect();
        assert!(failed.len() > 0);
    }
}
```

Expected: Tests FAIL

### GREEN: Implement Pipeline
- Create `validate_all()` function
- Run all validators
- Aggregate results

Expected: Tests PASS

### REFACTOR
- Optimize execution
- Parallel validation (future)

### Acceptance Criteria
- Pipeline tests passing
- All validators executed
- Results aggregated

## Phase 10: Reporter (TDD)

### RED: Write Tests
In `src/reporter.rs`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_format_success() {
        let results = vec![/* passing results */];
        let output = format_results(&results, false);
        assert!(output.contains("✓"));
        assert!(output.contains("successful"));
    }

    #[test]
    fn test_format_failure() {
        let results = vec![/* failing results */];
        let output = format_results(&results, false);
        assert!(output.contains("✗"));
        assert!(output.contains("failed"));
    }

    #[test]
    fn test_verbose_output() {
        let results = vec![/* results */];
        let output = format_results(&results, true);
        assert!(output.contains("[1/"));
    }
}
```

Expected: Tests FAIL

### GREEN: Implement Reporter
- Format success messages
- Format failure messages
- Implement verbose mode
- Display line numbers

Expected: Tests PASS

### REFACTOR
- Improve formatting
- Better color/symbols (optional)
- Cleaner output

### Acceptance Criteria
- Reporter tests passing
- Clear, readable output
- Verbose mode working
- Helpful error display

## Phase 11: Main Integration (TDD)

### RED: Write Integration Tests
In `tests/integration_tests.rs`:
```rust
#[test]
fn test_default_behavior() {
    // Test running with no arguments
}

#[test]
fn test_custom_path_and_file() {
    // Test with -p and -f flags
}

#[test]
fn test_verbose_flag() {
    // Test -v output
}

#[test]
fn test_exit_codes() {
    // Test exit code 0 for success, 1 for failure
}
```

Expected: Tests FAIL

### GREEN: Implement Main
In `src/main.rs`:
- Parse CLI arguments
- Read file
- Run validators
- Format and display results
- Set exit code

Expected: Tests PASS

### REFACTOR
- Error handling
- Clean up main flow
- Final polish

### Acceptance Criteria
- Integration tests passing
- CLI fully functional
- Correct exit codes
- End-to-end working

## Phase 12: Final Testing & Documentation

### Tasks
- [ ] Run full test suite
- [ ] Achieve >80% code coverage
- [ ] Fix any clippy warnings
- [ ] Format code with rustfmt
- [ ] Test on different platforms (if available)
- [ ] Update documentation if needed
- [ ] Create example files
- [ ] Test all CLI options

### Acceptance Criteria
- All tests passing
- No clippy warnings
- Code formatted
- Documentation complete
- Ready for use

## Testing Fixtures Needed

Create these test files in `tests/fixtures/`:

1. `valid.md` - Passes all checks
2. `non_ascii.md` - Contains Unicode characters
3. `tree_chars.md` - Contains tree symbols
4. `mixed.md` - Multiple violations
5. `empty.md` - Empty file
6. `unprintable.md` - Contains control characters
7. `invalid_utf8.bin` - Invalid UTF-8 bytes

## Dependencies to Add

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"
```

## Success Criteria

The implementation is complete when:
- [ ] All phases completed
- [ ] All tests passing (>80% coverage)
- [ ] No compiler warnings
- [ ] No clippy warnings
- [ ] Code formatted with rustfmt
- [ ] Documentation complete
- [ ] README examples work
- [ ] CLI help is clear
- [ ] Error messages are helpful
- [ ] Exit codes correct

## Timeline Estimate

- Phase 1: ✓ Complete
- Phase 2: 30 minutes (types)
- Phase 3: 30 minutes (CLI)
- Phase 4: 30 minutes (file ops)
- Phase 5: 45 minutes (ASCII validator)
- Phase 6: 45 minutes (unprintable detector)
- Phase 7: 45 minutes (tree symbol detector)
- Phase 8: 30 minutes (UTF-8 validator)
- Phase 9: 30 minutes (pipeline)
- Phase 10: 45 minutes (reporter)
- Phase 11: 45 minutes (integration)
- Phase 12: 30 minutes (testing & polish)

Total: ~6 hours of focused development
