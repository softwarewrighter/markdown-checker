# Product Requirements Document (PRD)

## Product Name
README Checker

## Version
1.0.0

## Purpose
A command-line tool that validates markdown files (primarily README.md files) to ensure they contain only UTF-8 encoded, ASCII-subset characters without unprintable control characters or non-standard tree visualization symbols.

## Problem Statement
Many developers include non-ASCII characters, particularly tree-drawing symbols (├, └, │, ─), in markdown files when documenting directory structures. While these characters render nicely in some contexts, they can cause issues with:
- Basic text editors and terminals
- CI/CD pipelines expecting ASCII-only content
- Automated documentation systems
- Version control diff tools
- Screen readers and accessibility tools

## Target Users
- Software developers maintaining documentation
- DevOps engineers enforcing documentation standards
- Open source project maintainers
- Technical writers
- CI/CD pipeline operators

## Use Cases

### UC-1: Validate Default README
**Actor**: Developer
**Goal**: Quickly check if ./README.md meets standards
**Flow**:
1. Run `readme-checker` with no arguments
2. Tool checks ./README.md
3. Receive pass/fail result with specific violations if any

### UC-2: Check README in Subdirectory
**Actor**: Developer
**Goal**: Check README.md in a project subdirectory
**Flow**:
1. Run `readme-checker -p docs` or `readme-checker --path docs`
2. Tool checks docs/README.md
3. Receive validation results

### UC-3: Check Custom Markdown File
**Actor**: Developer
**Goal**: Validate a non-standard markdown file
**Flow**:
1. Run `readme-checker -f CONTRIBUTING.md`
2. Tool checks ./CONTRIBUTING.md
3. Receive validation results

### UC-4: Check File with Combined Path
**Actor**: Developer
**Goal**: Check markdown file in specific location
**Flow**:
1. Run `readme-checker -p docs/api -f overview.md`
2. Tool checks docs/api/overview.md
3. Receive validation results

### UC-5: Verbose Validation Output
**Actor**: Developer debugging issues
**Goal**: See detailed validation process
**Flow**:
1. Run `readme-checker -v` or `readme-checker --verbose`
2. Tool displays each validation step as it runs
3. See detailed progress and results

### UC-6: CI/CD Integration
**Actor**: CI/CD pipeline
**Goal**: Fail build if documentation doesn't meet standards
**Flow**:
1. CI script runs `readme-checker`
2. Tool exits with code 0 (success) or 1 (failure)
3. Pipeline proceeds or fails based on exit code

## Functional Requirements

### FR-1: Command-Line Interface
- Accept `-p` or `--path` argument for directory path (default: ".")
- Accept `-f` or `--file-name` argument for filename (default: "README.md")
- Accept `-v` or `--verbose` flag for detailed output
- Display help with `-h` or `--help`
- Display version with `-V` or `--version`

### FR-2: File Location
- Construct file path as `<path>/<filename>`
- Default behavior: check ./README.md
- Handle absolute and relative paths
- Report clear error if file not found

### FR-3: UTF-8 Validation
- Verify file is valid UTF-8 encoded text
- Detect and report invalid UTF-8 byte sequences
- Report line numbers containing violations

### FR-4: ASCII Subset Validation
- Verify all characters are within ASCII range (0-127)
- Detect any character with code point > 127
- Report line numbers containing non-ASCII characters
- Show the specific characters that violate the rule

### FR-5: Unprintable Character Detection
- Detect control characters and unprintable bytes
- Allow standard whitespace: space (32), tab (9), newline (10), carriage return (13)
- Detect null bytes and other control characters
- Report line numbers with violations

### FR-6: Tree Symbol Detection
- Specifically detect common tree drawing characters:
  - Box-drawing characters (U+2500 - U+257F)
  - Common symbols: ├ (U+251C), └ (U+2514), │ (U+2502), ─ (U+2500)
  - Other tree visualization characters
- Report exact line numbers and positions
- Suggest alternatives (e.g., use standard ASCII characters like |, +, -, `)

### FR-7: Error Reporting
**Success Case**:
```
✓ File validation successful: README.md
  - UTF-8 encoding: valid
  - ASCII subset: compliant
  - No unprintable characters detected
  - No tree symbols found
```

**Failure Case**:
```
✗ File validation failed: README.md

Errors found:
1. Non-ASCII characters detected (3 violations)
   Lines: 15, 23, 45

2. Tree symbols detected (2 violations)
   Line 15: ├── src/ (contains ├)
   Line 23: │   └── main.rs (contains │, └)

3. Unprintable characters (1 violation)
   Line 45: Contains control character 0x00
```

### FR-8: Verbose Output
When `-v` flag is used, display:
- File being checked
- Each validation step as it runs
- Progress indicators
- Detailed results for each validator

### FR-9: Exit Codes
- 0: File passes all validations
- 1: File fails one or more validations
- 2: Usage error (invalid arguments, file not found)

## Non-Functional Requirements

### NFR-1: Performance
- Process files up to 10MB in under 1 second
- Minimal memory footprint (stream processing preferred)

### NFR-2: Portability
- Run on Linux, macOS, and Windows
- No platform-specific dependencies

### NFR-3: Usability
- Clear, actionable error messages
- Consistent command-line interface
- Standard Unix conventions

### NFR-4: Maintainability
- Well-documented code
- Comprehensive test coverage (>80%)
- Modular architecture for easy extension

### NFR-5: Reliability
- Handle edge cases gracefully (empty files, binary files, etc.)
- No crashes on invalid input
- Deterministic results

## Out of Scope (Future Considerations)
- Markdown syntax validation
- Link checking
- Spell checking
- Auto-fixing violations
- Multiple file processing (batch mode)
- Configuration file support
- Custom validator plugins
- Output formats (JSON, XML)

## Success Metrics
- Tool correctly identifies all test cases
- Zero false positives in validation
- Processing time < 1s for typical README files (<1MB)
- Clear error messages rated helpful by users
- Easy integration into CI/CD pipelines

## Dependencies
- Rust toolchain (1.70+)
- `clap` crate for CLI parsing
- Standard library only for core functionality

## Release Criteria
- All functional requirements implemented
- Test coverage > 80%
- Documentation complete (README, examples)
- Runs successfully on Linux, macOS, Windows
- No known critical bugs
