# Development Process

## Test-Driven Development (TDD) Approach

This project follows strict Test-Driven Development practices using the Red-Green-Refactor cycle.

### Red-Green-Refactor Cycle

```
┌─────────────┐
│     RED     │  Write a failing test
│ Write Test  │  Test describes desired behavior
└──────┬──────┘  Run test → Should FAIL
       │
       ▼
┌─────────────┐
│    GREEN    │  Write minimal code to pass
│  Make Pass  │  Focus on making test pass
└──────┬──────┘  Run test → Should PASS
       │
       ▼
┌─────────────┐
│  REFACTOR   │  Improve code quality
│   Improve   │  Maintain passing tests
└──────┬──────┘  Run tests → All PASS
       │
       └───────→ (repeat for next feature)
```

## Implementation Order

### Phase 1: Foundation (Documentation & Setup)
1. ✓ Create documentation
   - architecture.md
   - prd.md
   - design.md
   - process.md (this file)
   - plan.md
   - status.md

2. ✓ Create LICENSE file
3. ✓ Update README.md
4. Set up project structure
5. Add dependencies to Cargo.toml

### Phase 2: Core Types & Traits (TDD)
**RED**: Write tests for validation types
- Test ValidationResult creation
- Test ValidationError formatting
- Test Validator trait behavior

**GREEN**: Implement core types
- ValidationResult struct
- ValidationError struct
- Validator trait
- Helper functions

**REFACTOR**: Clean up type implementations

### Phase 3: CLI Argument Parsing (TDD)
**RED**: Write tests for CLI parsing
- Test default arguments
- Test path argument
- Test filename argument
- Test verbose flag
- Test argument combinations
- Test help output

**GREEN**: Implement CLI
- Add clap dependency
- Create Cli struct
- Implement argument parsing
- Create Config from Cli

**REFACTOR**: Improve CLI ergonomics

### Phase 4: File Operations (TDD)
**RED**: Write tests for file reading
- Test reading existing file
- Test file not found error
- Test permission errors
- Test UTF-8 conversion errors
- Test path construction

**GREEN**: Implement file operations
- Read file as bytes
- Convert to UTF-8 string
- Error handling
- Path utilities

**REFACTOR**: Improve error messages

### Phase 5: UTF-8 Validator (TDD)
**RED**: Write tests for UTF-8 validation
- Test valid UTF-8 content
- Test invalid UTF-8 sequences
- Test error reporting

**GREEN**: Implement UTF-8 validator
- Create Utf8Validator struct
- Implement validation logic
- Return ValidationResult

**REFACTOR**: Optimize validation

### Phase 6: ASCII Validator (TDD)
**RED**: Write tests for ASCII validation
- Test pure ASCII content (should pass)
- Test content with Unicode characters (should fail)
- Test emoji detection
- Test accented characters
- Test line number reporting
- Test character position reporting

**GREEN**: Implement ASCII validator
- Create AsciiValidator struct
- Implement Validator trait
- Check character codes <= 127
- Collect errors with line numbers

**REFACTOR**: Improve error messages and performance

### Phase 7: Unprintable Character Detector (TDD)
**RED**: Write tests for unprintable characters
- Test allowed whitespace (space, tab, newline, CR)
- Test printable ASCII (32-126)
- Test control characters (0-31, 127)
- Test null bytes
- Test line number reporting

**GREEN**: Implement unprintable validator
- Create UnprintableValidator struct
- Define printable character set
- Allow standard whitespace
- Detect and report violations

**REFACTOR**: Clarify printable rules

### Phase 8: Tree Symbol Detector (TDD)
**RED**: Write tests for tree symbols
- Test content without tree symbols (should pass)
- Test common tree characters: ├ └ │ ─
- Test box-drawing range (U+2500 - U+257F)
- Test mixed content
- Test suggestion messages
- Test line and column reporting

**GREEN**: Implement tree symbol validator
- Create TreeSymbolValidator struct
- Define tree symbol set
- Implement detection logic
- Generate helpful suggestions

**REFACTOR**: Complete symbol coverage

### Phase 9: Validation Pipeline (TDD)
**RED**: Write tests for validation orchestration
- Test running all validators
- Test result aggregation
- Test overall pass/fail determination
- Test error collection

**GREEN**: Implement validation pipeline
- Create validate_all function
- Run all validators
- Aggregate results
- Determine overall status

**REFACTOR**: Optimize validator execution

### Phase 10: Reporter (TDD)
**RED**: Write tests for output formatting
- Test success message
- Test failure message with errors
- Test verbose output
- Test line number formatting
- Test error grouping

**GREEN**: Implement reporter
- Create Reporter struct
- Format success output
- Format failure output
- Implement verbose mode

**REFACTOR**: Improve output clarity

### Phase 11: Integration (TDD)
**RED**: Write end-to-end tests
- Test full CLI workflow
- Test default behavior
- Test custom paths and filenames
- Test verbose flag
- Test exit codes
- Test error scenarios

**GREEN**: Integrate all components
- Wire up main.rs
- Connect CLI to validators
- Implement error handling
- Set exit codes

**REFACTOR**: Final polish

## Testing Standards

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange: Set up test data
        let input = "test content";

        // Act: Perform the action
        let result = function_under_test(input);

        // Assert: Verify the outcome
        assert_eq!(result.status, ValidationStatus::Pass);
    }
}
```

### Test Coverage Requirements
- Minimum 80% code coverage
- Every public function must have tests
- Every error path must be tested
- Edge cases must be covered

### Test Categories

**Unit Tests**: Test individual functions and modules
- Located in same file as implementation
- Use `#[cfg(test)]` module
- Test one thing at a time

**Integration Tests**: Test component interactions
- Located in `tests/` directory
- Test realistic scenarios
- Use test fixtures

**Fixture Files**: Test data files
- Located in `tests/fixtures/`
- Represent real-world examples
- Include valid and invalid cases

## Code Quality Standards

### Rust Best Practices
- Use `clippy` for linting: `cargo clippy`
- Format code with `rustfmt`: `cargo fmt`
- No compiler warnings allowed
- Follow Rust naming conventions

### Error Handling
- Never use `.unwrap()` in production code
- Use `Result` types for operations that can fail
- Provide context in error messages
- Use custom error types for clarity

### Documentation
- Document all public APIs with doc comments
- Include examples in documentation
- Keep docs up-to-date with code

### Git Workflow

#### Commit Messages
Follow conventional commit format:
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `chore`: Maintenance tasks

Examples:
```
feat(validator): add tree symbol detection

Implements detection of box-drawing characters commonly used
in directory tree visualizations.

test(ascii): add tests for unicode emoji detection

fix(cli): correct path joining for Windows compatibility
```

#### Branch Strategy
- `main`: Stable, release-ready code
- Feature branches: `feature/description`
- Bug fix branches: `fix/description`

## Development Commands

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in specific module
cargo test module_name::

# Run with coverage (using tarpaulin)
cargo tarpaulin --out Html
```

### Linting and Formatting
```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Run clippy
cargo clippy

# Run clippy with strict settings
cargo clippy -- -D warnings
```

### Running
```bash
# Run with cargo
cargo run

# Run with arguments
cargo run -- -v -p docs -f CONTRIBUTING.md

# Run release build
cargo run --release -- -v
```

## Continuous Integration

### Pre-commit Checklist

**CRITICAL**: Before committing any code, ALL of these checks must pass. Never disable or bypass warnings - always fix the underlying problem. Do not add technical debt.

1. **Run All Tests**
   ```bash
   cargo test
   ```
   - All tests must pass
   - No ignored tests without documented reason
   - Fix any test failures before proceeding

2. **Fix All Lint Warnings**
   ```bash
   cargo clippy -- -D warnings
   ```
   - Fix ALL clippy warnings
   - Never use `#[allow()]` to suppress warnings
   - Address the root cause of each warning
   - No exceptions - zero warnings required

3. **Fix All Compiler Warnings**
   ```bash
   cargo build
   ```
   - Zero compiler warnings allowed
   - Fix unused imports, variables, etc.
   - Never suppress with `#[allow(dead_code)]` or similar
   - Clean builds only

4. **Format Code**
   ```bash
   cargo fmt
   ```
   - Apply standard Rust formatting
   - Check with `cargo fmt --check` first
   - Commit formatted code only

5. **Validate .gitignore**
   ```bash
   git status
   ```
   - Ensure no build artifacts in staging
   - Verify `target/` is ignored
   - Check for accidental IDE files (.idea/, .vscode/ configs)
   - No sensitive files (API keys, credentials)

6. **Update Documentation**
   - Update relevant docs/ files if architecture changed
   - Update README.md if CLI interface changed
   - Update inline code documentation (doc comments)
   - Verify examples still work
   - Update status.md with progress

7. **Stage and Commit**
   ```bash
   # Review changes
   git diff

   # Stage files
   git add <files>

   # Commit with detailed message
   git commit -m "type(scope): description

   - Detailed change 1
   - Detailed change 2
   - Fixes #issue-number (if applicable)
   "
   ```

8. **Push**
   ```bash
   git push
   ```
   - If push fails, **STOP** and ask for help
   - Do not force push without consultation
   - Do not bypass branch protection rules

### Pre-commit Workflow

```
┌─────────────────────────────────┐
│   Make code changes             │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│   cargo test                    │
│   All tests pass?               │
└────────────┬────────────────────┘
             │ NO → Fix failures
             ▼ YES
┌─────────────────────────────────┐
│   cargo clippy -- -D warnings   │
│   Zero warnings?                │
└────────────┬────────────────────┘
             │ NO → Fix warnings (never suppress)
             ▼ YES
┌─────────────────────────────────┐
│   cargo build                   │
│   Zero warnings?                │
└────────────┬────────────────────┘
             │ NO → Fix warnings (never suppress)
             ▼ YES
┌─────────────────────────────────┐
│   cargo fmt                     │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│   git status                    │
│   Validate .gitignore           │
└────────────┬────────────────────┘
             │ Issues? → Fix .gitignore
             ▼ Clean
┌─────────────────────────────────┐
│   Update docs if needed         │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│   git add <files>               │
│   git commit -m "..."           │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│   git push                      │
└────────────┬────────────────────┘
             │ Failed? → ASK FOR HELP
             ▼ Success
┌─────────────────────────────────┐
│   ✓ Changes pushed              │
└─────────────────────────────────┘
```

### Warning Suppression Policy

**NEVER** use any of these without explicit approval:
- `#[allow(dead_code)]`
- `#[allow(unused_variables)]`
- `#[allow(clippy::...)]`
- `--allow-dirty` flag
- `git commit --no-verify`

**ALWAYS** fix the underlying issue:
- **Investigate unused code warnings carefully**: Code may be used by tests (e.g., wasm-bindgen tests) or other configurations
- If code appears unused, check:
  - Is it used in tests with special attributes?
  - Is it used in different build configurations?
  - Is there a test configuration issue that needs fixing?
- Only remove code after confirming it's truly unused
- Use underscores for intentionally unused variables (`_result`)
- Refactor to address clippy suggestions
- Clean working directory before commits
- Let pre-commit hooks run

### Technical Debt Policy

- Zero tolerance for new technical debt
- All warnings represent potential issues
- Suppressing warnings hides problems
- Fix issues properly, don't hide them
- Code quality is non-negotiable

### CI Pipeline (Future)
1. Build on Linux, macOS, Windows
2. Run full test suite
3. Check code formatting
4. Run clippy
5. Generate coverage report
6. Build documentation

## Release Process

### Version Numbering
Follow Semantic Versioning (semver):
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

### Release Checklist
1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Run full test suite
4. Build release binaries
5. Tag release in git
6. Create GitHub release
7. Publish to crates.io (future)

## Metrics and Goals

### Performance Targets
- Process 1MB file in < 100ms
- Process 10MB file in < 1s
- Memory usage < 10MB for typical files

### Quality Targets
- Test coverage > 80%
- Zero clippy warnings
- Zero compiler warnings
- All documentation examples working

### User Experience Goals
- Clear, actionable error messages
- Helpful suggestions for violations
- Consistent CLI interface
- Fast execution time
