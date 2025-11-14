# Design Document

## System Design

### Component Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    CLI Interface                        │
│                   (clap-based)                          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                 Configuration                           │
│  ┌──────────────────────────────────────────────────┐  │
│  │ struct Config {                                  │  │
│  │   path: PathBuf,                                 │  │
│  │   filename: String,                              │  │
│  │   verbose: bool,                                 │  │
│  │ }                                                │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                File Operations                          │
│  - Read file as bytes                                   │
│  - Convert to UTF-8 string                              │
│  - Handle file errors                                   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Validation Pipeline                        │
│  ┌─────────────────────────────────────────────────┐   │
│  │         UTF-8 Validator                         │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │         ASCII Validator                         │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │      Unprintable Character Detector             │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │         Tree Symbol Detector                    │   │
│  └─────────────────────────────────────────────────┘   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Results Aggregator                         │
│  - Collect all validation results                       │
│  - Determine overall pass/fail                          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                 Reporter                                │
│  - Format output (verbose or normal)                    │
│  - Display errors with line numbers                     │
│  - Set exit code                                        │
└─────────────────────────────────────────────────────────┘
```

## Data Structures

### Configuration
```rust
#[derive(Debug, Clone)]
pub struct Config {
    /// Base path where file is located
    pub path: PathBuf,

    /// Name of the file to check
    pub filename: String,

    /// Enable verbose output
    pub verbose: bool,
}

impl Config {
    pub fn file_path(&self) -> PathBuf {
        self.path.join(&self.filename)
    }
}
```

### Validation Results
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub validator_name: String,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub line_number: usize,
    pub column: Option<usize>,
    pub message: String,
    pub context: Option<String>,
}
```

### Validator Trait
```rust
pub trait Validator {
    fn name(&self) -> &str;
    fn validate(&self, content: &str) -> ValidationResult;
}
```

## Validators Design

### 1. UTF-8 Validator
```rust
pub struct Utf8Validator;

impl Validator for Utf8Validator {
    fn name(&self) -> &str {
        "UTF-8 Encoding"
    }

    fn validate(&self, content: &str) -> ValidationResult {
        // Since content is already &str, it's valid UTF-8
        // This validator checks the raw bytes before conversion
        ValidationResult {
            status: ValidationStatus::Pass,
            validator_name: self.name().to_string(),
            errors: vec![],
        }
    }
}

// Note: Actual UTF-8 validation happens during file reading
// when converting bytes to string
```

### 2. ASCII Validator
```rust
pub struct AsciiValidator;

impl Validator for AsciiValidator {
    fn name(&self) -> &str {
        "ASCII Subset"
    }

    fn validate(&self, content: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch as u32 > 127 {
                    errors.push(ValidationError {
                        line_number: line_num + 1,
                        column: Some(col + 1),
                        message: format!("Non-ASCII character: '{}' (U+{:04X})", ch, ch as u32),
                        context: Some(line.to_string()),
                    });
                }
            }
        }

        ValidationResult {
            status: if errors.is_empty() { ValidationStatus::Pass } else { ValidationStatus::Fail },
            validator_name: self.name().to_string(),
            errors,
        }
    }
}
```

### 3. Unprintable Character Detector
```rust
pub struct UnprintableValidator;

impl UnprintableValidator {
    fn is_allowed_whitespace(ch: char) -> bool {
        matches!(ch, ' ' | '\t' | '\n' | '\r')
    }

    fn is_printable(ch: char) -> bool {
        let code = ch as u32;
        // Printable ASCII: 32-126
        (code >= 32 && code <= 126) || Self::is_allowed_whitespace(ch)
    }
}

impl Validator for UnprintableValidator {
    fn name(&self) -> &str {
        "Printable Characters"
    }

    fn validate(&self, content: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if !Self::is_printable(ch) {
                    errors.push(ValidationError {
                        line_number: line_num + 1,
                        column: Some(col + 1),
                        message: format!("Unprintable character: U+{:04X}", ch as u32),
                        context: Some(line.to_string()),
                    });
                }
            }
        }

        ValidationResult {
            status: if errors.is_empty() { ValidationStatus::Pass } else { ValidationStatus::Fail },
            validator_name: self.name().to_string(),
            errors,
        }
    }
}
```

### 4. Tree Symbol Detector
```rust
pub struct TreeSymbolValidator;

impl TreeSymbolValidator {
    const TREE_SYMBOLS: &[char] = &[
        '├', '└', '│', '─', '┌', '┐', '┘', '┤', '┴', '┬', '┼',
        '╭', '╮', '╯', '╰', '╱', '╲', '╳',
    ];

    fn is_tree_symbol(ch: char) -> bool {
        Self::TREE_SYMBOLS.contains(&ch) ||
        (ch as u32 >= 0x2500 && ch as u32 <= 0x257F) // Box Drawing block
    }

    fn suggest_alternative(ch: char) -> String {
        match ch {
            '├' | '┤' => "Use '+' or '|' instead",
            '└' | '┘' | '┌' | '┐' => "Use '+' or '`' instead",
            '│' => "Use '|' instead",
            '─' => "Use '-' instead",
            _ => "Use standard ASCII characters like |, +, -, `",
        }.to_string()
    }
}

impl Validator for TreeSymbolValidator {
    fn name(&self) -> &str {
        "Tree Symbols"
    }

    fn validate(&self, content: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if Self::is_tree_symbol(ch) {
                    errors.push(ValidationError {
                        line_number: line_num + 1,
                        column: Some(col + 1),
                        message: format!(
                            "Tree symbol '{}' (U+{:04X}) detected. {}",
                            ch,
                            ch as u32,
                            Self::suggest_alternative(ch)
                        ),
                        context: Some(line.to_string()),
                    });
                }
            }
        }

        ValidationResult {
            status: if errors.is_empty() { ValidationStatus::Pass } else { ValidationStatus::Fail },
            validator_name: self.name().to_string(),
            errors,
        }
    }
}
```

## CLI Design

### Argument Structure
```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "markdown-checker")]
#[command(author = "Michael A Wright")]
#[command(version = "1.0.0")]
#[command(about = "Validates markdown files for UTF-8, ASCII-subset, and unprintable characters", long_about = None)]
pub struct Cli {
    /// Path to directory containing the file (default: current directory)
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    /// Name of the file to check (default: README.md)
    #[arg(short = 'f', long, value_name = "NAME", default_value = "README.md")]
    pub file_name: String,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
```

## Reporter Design

### Output Formatting

**Normal Mode (Success)**:
```
✓ File validation successful: ./README.md
```

**Normal Mode (Failure)**:
```
✗ File validation failed: ./README.md

UTF-8 Encoding: ✓ Pass
ASCII Subset: ✗ Fail (3 errors)
  Line 15, Column 5: Non-ASCII character: '├' (U+251C)
  Line 23, Column 3: Non-ASCII character: '│' (U+2502)
  Line 23, Column 7: Non-ASCII character: '└' (U+2514)

Printable Characters: ✓ Pass
Tree Symbols: ✗ Fail (3 errors)
  Line 15, Column 5: Tree symbol '├' (U+251C) detected. Use '+' or '|' instead
  Line 23, Column 3: Tree symbol '│' (U+2502) detected. Use '|' instead
  Line 23, Column 7: Tree symbol '└' (U+2514) detected. Use '+' or '`' instead
```

**Verbose Mode**:
```
Checking file: ./README.md
File size: 1,234 bytes

Running validators...
[1/4] UTF-8 Encoding... ✓ Pass
[2/4] ASCII Subset... ✗ Fail (3 errors found)
[3/4] Printable Characters... ✓ Pass
[4/4] Tree Symbols... ✗ Fail (3 errors found)

Results:
✗ File validation failed: ./README.md

[Full error details as above...]
```

## Error Handling

### File Errors
```rust
pub enum FileError {
    NotFound(PathBuf),
    PermissionDenied(PathBuf),
    InvalidUtf8(PathBuf),
    IoError(PathBuf, std::io::Error),
}

impl FileError {
    pub fn to_user_message(&self) -> String {
        match self {
            Self::NotFound(path) => format!("File not found: {}", path.display()),
            Self::PermissionDenied(path) => format!("Permission denied: {}", path.display()),
            Self::InvalidUtf8(path) => format!("File is not valid UTF-8: {}", path.display()),
            Self::IoError(path, err) => format!("I/O error reading {}: {}", path.display(), err),
        }
    }
}
```

## Testing Strategy

### Unit Tests Structure
```
tests/
├── unit/
│   ├── utf8_validator_tests.rs
│   ├── ascii_validator_tests.rs
│   ├── unprintable_validator_tests.rs
│   └── tree_symbol_validator_tests.rs
├── integration/
│   ├── cli_tests.rs
│   └── end_to_end_tests.rs
└── fixtures/
    ├── valid.md
    ├── invalid_utf8.txt
    ├── non_ascii.md
    ├── tree_chars.md
    └── mixed_violations.md
```

### Test Cases

**UTF-8 Validator**:
- Valid UTF-8 text
- Invalid byte sequences
- Partial UTF-8 characters

**ASCII Validator**:
- Pure ASCII text
- Unicode characters (emojis, accents, symbols)
- Mixed ASCII and Unicode

**Unprintable Validator**:
- Normal text with allowed whitespace
- Control characters (null, bell, etc.)
- Tab and newline handling

**Tree Symbol Validator**:
- Text without tree symbols
- Common tree symbols (├, └, │, ─)
- Full box-drawing character range
- Mixed content

## Performance Optimization

### Single-Pass Processing
```rust
pub fn validate_all(content: &str) -> Vec<ValidationResult> {
    let validators: Vec<Box<dyn Validator>> = vec![
        Box::new(AsciiValidator),
        Box::new(UnprintableValidator),
        Box::new(TreeSymbolValidator),
    ];

    validators.iter()
        .map(|v| v.validate(content))
        .collect()
}
```

### Memory Efficiency
- Read file once into memory
- Process line-by-line for error collection
- No intermediate buffering
- Stream output for large error sets
