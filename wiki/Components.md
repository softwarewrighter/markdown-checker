# Key Components

This page provides a detailed overview of the major components in the Markdown Checker system.

## Component Overview Diagram

```mermaid
graph TB
    subgraph "User Interface Layer"
        CLI[CLI Parser cli.rs]
        Config[Configuration Config struct]
    end

    subgraph "Processing Layer"
        FileOps[File Operations file_ops.rs]
        Glob[Glob Processor Pattern matching]
    end

    subgraph "Validation Layer"
        ValidatorTrait[Validator Trait validators/mod.rs]
        UTF8[UTF-8 Validator validators/utf8.rs]
        ASCII[ASCII Validator validators/ascii.rs]
        Unprint[Unprintable Validator validators/unprintable.rs]
        Tree[Tree Symbol Validator validators/tree_symbols.rs]
    end

    subgraph "Modification Layer"
        Fixer[Auto-Fix System fixer.rs]
    end

    subgraph "Output Layer"
        Reporter[Reporter reporter.rs]
    end

    CLI --> Config
    Config --> FileOps
    Config --> Glob
    FileOps --> ValidatorTrait
    ValidatorTrait --> UTF8
    ValidatorTrait --> ASCII
    ValidatorTrait --> Unprint
    ValidatorTrait --> Tree
    ValidatorTrait --> Fixer
    Fixer --> Reporter
    ValidatorTrait --> Reporter

    style CLI fill:#e3f2fd
    style ValidatorTrait fill:#fff9c4
    style Fixer fill:#f3e5f5
    style Reporter fill:#e8f5e9
```

---

## Component Details

### 1. CLI Parser (cli.rs)

**Responsibility**: Parse and validate command-line arguments

**Key Features**:
- Built on `clap` crate for robust argument parsing
- Provides help text and version information
- Validates argument combinations
- Supports multiple flags and options

**Arguments**:
- `-p, --path <PATH>` - Directory path (default: current directory)
- `-f, --file-name <NAME>` - Filename or glob pattern (default: README.md)
- `-v, --verbose` - Enable verbose output
- `--fix` - Auto-fix violations
- `-n, --dry-run` - Preview fixes without applying

**Data Structures**:
```rust
pub struct Cli {
    pub path: PathBuf,
    pub file_name: String,
    pub verbose: bool,
    pub fix: bool,
    pub dry_run: bool,
}
```

**Diagram**:
```mermaid
graph LR
    Input[User Input] --> Parser[clap Parser]
    Parser --> Validator{Valid?}
    Validator -->|Yes| Config[Create Config]
    Validator -->|No| Help[Show Help/Error]
    Config --> Next[File Operations]

    style Parser fill:#bbdefb
    style Config fill:#c8e6c9
    style Help fill:#ffcdd2
```

---

### 2. Configuration (Config struct)

**Responsibility**: Store validated runtime configuration

**Key Features**:
- Immutable after creation
- Provides helper methods for path construction
- Centralized configuration access

**Data Structure**:
```rust
pub struct Config {
    pub path: PathBuf,
    pub filename: String,
    pub verbose: bool,
    pub fix: bool,
    pub dry_run: bool,
}

impl Config {
    pub fn file_path(&self) -> PathBuf {
        self.path.join(&self.filename)
    }
}
```

---

### 3. File Operations (file_ops.rs)

**Responsibility**: Handle all file I/O operations

**Key Features**:
- Safe file reading with error handling
- UTF-8 validation during read
- Glob pattern expansion
- File writing for auto-fix
- Backup creation before modifications

**Functions**:
- `read_file(path: &Path) -> Result<String, FileError>`
- `write_file(path: &Path, content: &str) -> Result<(), FileError>`
- `expand_glob(pattern: &str) -> Result<Vec<PathBuf>, GlobError>`

**Error Types**:
```rust
pub enum FileError {
    NotFound(PathBuf),
    PermissionDenied(PathBuf),
    InvalidUtf8(PathBuf),
    IoError(PathBuf, std::io::Error),
}
```

**Diagram**:
```mermaid
graph TD
    Request[Read Request] --> Exists{File Exists?}
    Exists -->|No| NotFound[FileError::NotFound]
    Exists -->|Yes| Perms{Has Permissions?}
    Perms -->|No| PermDenied[FileError::PermissionDenied]
    Perms -->|Yes| Read[Read Bytes]
    Read --> UTF8{Valid UTF-8?}
    UTF8 -->|No| InvalidUTF8[FileError::InvalidUtf8]
    UTF8 -->|Yes| Success[Return String]

    style Success fill:#c8e6c9
    style NotFound fill:#ffcdd2
    style PermDenied fill:#ffcdd2
    style InvalidUTF8 fill:#ffcdd2
```

---

### 4. Validator Trait (validators/mod.rs)

**Responsibility**: Define interface for all validators

**Key Features**:
- Common interface for all validation logic
- Extensible design for new validators
- Standardized result format

**Trait Definition**:
```rust
pub trait Validator {
    fn name(&self) -> &str;
    fn validate(&self, content: &str) -> ValidationResult;
}
```

**Result Structures**:
```rust
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub validator_name: String,
    pub errors: Vec<ValidationError>,
}

pub struct ValidationError {
    pub line_number: usize,
    pub column: Option<usize>,
    pub message: String,
    pub context: Option<String>,
}

pub enum ValidationStatus {
    Pass,
    Fail,
}
```

**Diagram**:
```mermaid
classDiagram
    class Validator {
        <<trait>>
        +name() String
        +validate(content) ValidationResult
    }

    class ValidationResult {
        +status: ValidationStatus
        +validator_name: String
        +errors: Vec~ValidationError~
    }

    class ValidationError {
        +line_number: usize
        +column: Option~usize~
        +message: String
        +context: Option~String~
    }

    class ValidationStatus {
        <<enumeration>>
        Pass
        Fail
    }

    Validator --> ValidationResult
    ValidationResult --> ValidationError
    ValidationResult --> ValidationStatus
```

---

### 5. Validators

#### UTF-8 Validator (validators/utf8.rs)

**Purpose**: Ensure file is valid UTF-8 encoded text

**Implementation Note**: UTF-8 validation occurs during file reading when converting bytes to string. This validator confirms the string is valid.

#### ASCII Validator (validators/ascii.rs)

**Purpose**: Detect any characters outside ASCII range (0-127)

**Algorithm**:
1. Iterate through each line
2. For each character, check if code point > 127
3. Record violations with line and column numbers
4. Provide Unicode code point in error message

**Example Violations**:
- Emojis (U+1F600, etc.)
- Accented characters (é, ñ, ü)
- Box-drawing symbols (├, │, ─)
- Mathematical symbols (∑, π, ∞)

#### Unprintable Validator (validators/unprintable.rs)

**Purpose**: Detect control characters and unprintable bytes

**Allowed Characters**:
- Printable ASCII: 32-126 (space through tilde)
- Whitespace: space (32), tab (9), newline (10), carriage return (13)

**Disallowed Characters**:
- Null bytes (0)
- Bell character (7)
- Other control characters
- Delete character (127)

**Algorithm**:
```rust
fn is_printable(ch: char) -> bool {
    let code = ch as u32;
    (code >= 32 && code <= 126) || is_allowed_whitespace(ch)
}
```

#### Tree Symbol Validator (validators/tree_symbols.rs)

**Purpose**: Detect box-drawing characters used in directory trees

**Detected Symbols**:
- Box Drawing Unicode Block: U+2500 - U+257F
- Common symbols: ├ └ │ ─ ┌ ┐ ┘ ┤ ┴ ┬ ┼

**Suggestions**:
```rust
match symbol {
    '├' | '┤' => "Use '+' or '|' instead",
    '└' | '┘' | '┌' | '┐' => "Use '+' or '`' instead",
    '│' => "Use '|' instead",
    '─' => "Use '-' instead",
    _ => "Use standard ASCII characters",
}
```

**Diagram**:
```mermaid
graph TD
    Input[File Content] --> V1[UTF-8 Validator]
    V1 --> R1{Valid UTF-8?}
    R1 -->|Pass| V2[ASCII Validator]
    R1 -->|Fail| Fail1[Report UTF-8 Error]

    V2 --> R2{All ASCII?}
    R2 -->|Pass| V3[Unprintable Validator]
    R2 -->|Fail| Fail2[Report Non-ASCII]

    V3 --> R3{All Printable?}
    R3 -->|Pass| V4[Tree Symbol Validator]
    R3 -->|Fail| Fail3[Report Unprintable]

    V4 --> R4{No Tree Symbols?}
    R4 -->|Pass| Success[All Validators Pass]
    R4 -->|Fail| Fail4[Report Tree Symbols]

    Fail1 --> Aggregate[Aggregate Results]
    Fail2 --> Aggregate
    Fail3 --> Aggregate
    Fail4 --> Aggregate
    Success --> Aggregate

    style Success fill:#c8e6c9
    style Fail1 fill:#ffcdd2
    style Fail2 fill:#ffcdd2
    style Fail3 fill:#ffcdd2
    style Fail4 fill:#ffcdd2
```

---

### 6. Auto-Fix System (fixer.rs)

**Responsibility**: Automatically correct fixable violations

**Key Features**:
- Analyzes validation results for fixability
- Currently supports tree symbol replacement
- Verifies fixes before writing
- Supports dry-run mode for preview

**Fixable Violations**:
- Tree symbols (box-drawing characters)

**Unfixable Violations**:
- Other Unicode characters (emojis, accents)
- Invalid UTF-8 sequences
- Control characters

**Replacement Map**:
```rust
const TREE_REPLACEMENTS: &[(char, char)] = &[
    ('├', '+'),
    ('└', '+'),
    ('│', '|'),
    ('─', '-'),
    ('┌', '+'),
    ('┐', '+'),
    ('┘', '+'),
    ('┤', '+'),
    // ... more mappings
];
```

**Fix Process**:
```mermaid
graph TD
    Start[Validation Results] --> Check{All Violations Tree Symbols?}
    Check -->|Yes| Map[Build Replacement Map]
    Check -->|No| Unfixable[Report: Cannot Fix]

    Map --> DryRun{Dry Run Mode?}
    DryRun -->|Yes| Preview[Generate Preview]
    DryRun -->|No| Apply[Apply Replacements]

    Apply --> Verify[Re-validate Content]
    Verify --> Valid{Validation Passes?}
    Valid -->|Yes| Write[Write File]
    Valid -->|No| Failed[Report: Fix Failed]

    Preview --> Display[Display Changes]
    Write --> Success[Report: Fixed]

    style Success fill:#c8e6c9
    style Unfixable fill:#ffcdd2
    style Failed fill:#ffcdd2
```

---

### 7. Reporter (reporter.rs)

**Responsibility**: Format and display validation results

**Key Features**:
- Normal and verbose output modes
- Color-coded output (pass/fail indicators)
- Detailed error messages with line numbers
- Exit code management

**Output Modes**:

**Normal Mode**:
```
✓ File validation successful: ./README.md
```

**Failure Output**:
```
✗ File validation failed: ./README.md

ASCII Subset: ✗ Fail (2 errors)
  Line 15, Column 5: Non-ASCII character: '├' (U+251C)
  Line 23, Column 3: Non-ASCII character: '│' (U+2502)

Tree Symbols: ✗ Fail (2 errors)
  Line 15, Column 5: Tree symbol '├' (U+251C) detected. Use '+' or '|' instead
  Line 23, Column 3: Tree symbol '│' (U+2502) detected. Use '|' instead
```

**Verbose Mode**:
```
Checking file: ./README.md
File size: 1,234 bytes

Running validators...
[1/4] UTF-8 Encoding... ✓ Pass
[2/4] ASCII Subset... ✗ Fail (2 errors found)
[3/4] Printable Characters... ✓ Pass
[4/4] Tree Symbols... ✗ Fail (2 errors found)

Results:
[Detailed output as above]
```

**Exit Codes**:
- `0` - All validations passed
- `1` - Validation failures detected
- `2` - File errors or invalid arguments

**Diagram**:
```mermaid
graph LR
    Results[Validation Results] --> Formatter[Result Formatter]
    Formatter --> Verbose{Verbose Mode?}

    Verbose -->|Yes| DetailOut[Detailed Output]
    Verbose -->|No| NormalOut[Normal Output]

    DetailOut --> Status[Determine Exit Code]
    NormalOut --> Status

    Status --> Success{All Pass?}
    Success -->|Yes| Exit0[Exit Code 0]
    Success -->|No| Exit1[Exit Code 1]

    style Exit0 fill:#c8e6c9
    style Exit1 fill:#ffcdd2
```

---

## Component Interaction Summary

```mermaid
graph TD
    A[User Input] --> B[CLI Parser]
    B --> C[Configuration]
    C --> D[File Operations]
    D --> E[Validation Engine]
    E --> F1[UTF-8 Validator]
    E --> F2[ASCII Validator]
    E --> F3[Unprintable Validator]
    E --> F4[Tree Symbol Validator]
    F1 --> G[Results Aggregator]
    F2 --> G
    F3 --> G
    F4 --> G
    G --> H{Fix Mode?}
    H -->|Yes| I[Auto-Fix System]
    H -->|No| J[Reporter]
    I --> K[Verify Fix]
    K --> L{Valid?}
    L -->|Yes| M[Write File]
    L -->|No| J
    M --> J
    J --> N[Console Output]
    J --> O[Exit Code]

    style B fill:#e3f2fd
    style E fill:#fff9c4
    style I fill:#f3e5f5
    style J fill:#e8f5e9
```

---

## Related Documentation

- [Architecture Overview](Architecture) - High-level system architecture
- [Workflows & Sequences](Workflows) - Detailed sequence diagrams
- [CLI Interface](CLI-Interface) - CLI component deep dive
- [Validation Engine](Validation-Engine) - Validator implementation details
- [Auto-Fix System](Auto-Fix-System) - Fix mechanism deep dive
- [Reporter Module](Reporter-Module) - Output formatting details
