# Architecture Overview

## System Architecture

Markdown Checker follows a layered architecture with clear separation of concerns:

```mermaid
graph TB
    User[User/CLI] --> CLI[CLI Interface Layer]
    CLI --> Config[Configuration]
    Config --> FileOps[File Operations]
    FileOps --> VEngine[Validation Engine]
    VEngine --> Reporter[Reporter Module]
    Reporter --> Output[Console Output]

    VEngine --> UTF8[UTF-8 Validator]
    VEngine --> ASCII[ASCII Validator]
    VEngine --> Unprint[Unprintable Validator]
    VEngine --> Tree[Tree Symbol Validator]

    FileOps --> Fixer[Auto-Fix System]
    Fixer --> FileOps

    style CLI fill:#e1f5ff
    style VEngine fill:#fff4e1
    style Reporter fill:#e8f5e9
    style Fixer fill:#fce4ec
```

## Component Diagram

```mermaid
graph LR
    subgraph "CLI Layer"
        A[clap Parser] --> B[Argument Validator]
        B --> C[Config Builder]
    end

    subgraph "File Operations"
        D[File Reader] --> E[Path Resolver]
        D --> F[Glob Processor]
        G[File Writer] --> H[Backup Manager]
    end

    subgraph "Validation Engine"
        I[Validator Trait] --> J[UTF-8 Validator]
        I --> K[ASCII Validator]
        I --> L[Unprintable Validator]
        I --> M[Tree Symbol Validator]
        N[Results Aggregator] --> I
    end

    subgraph "Auto-Fix System"
        O[Fix Analyzer] --> P[Character Replacer]
        P --> Q[Verification Engine]
    end

    subgraph "Reporter"
        R[Formatter] --> S[Error Display]
        R --> T[Verbose Output]
        R --> U[Exit Code Handler]
    end

    C --> D
    D --> N
    N --> O
    O --> G
    N --> R

    style I fill:#ffeb3b
    style N fill:#ff9800
    style R fill:#4caf50
```

## Data Flow Architecture

```mermaid
flowchart TD
    Start([CLI Arguments]) --> Parse[Parse Arguments]
    Parse --> ValidateArgs{Valid Args?}
    ValidateArgs -->|No| UsageError[Exit Code 2: Usage Error]
    ValidateArgs -->|Yes| BuildConfig[Build Configuration]

    BuildConfig --> ResolveGlob{Glob Pattern?}
    ResolveGlob -->|Yes| ExpandGlob[Expand Glob Pattern]
    ResolveGlob -->|No| SingleFile[Single File Path]

    ExpandGlob --> FileLoop[For Each File]
    SingleFile --> FileLoop

    FileLoop --> ReadFile[Read File]
    ReadFile --> FileExists{File Exists?}
    FileExists -->|No| FileError[Exit Code 2: File Error]
    FileExists -->|Yes| UTF8Check{Valid UTF-8?}

    UTF8Check -->|No| UTF8Error[Report UTF-8 Error]
    UTF8Check -->|Yes| RunValidators[Run All Validators]

    RunValidators --> CollectResults[Collect Validation Results]
    CollectResults --> CheckFix{Fix Mode?}

    CheckFix -->|Yes| Fixable{All Violations Fixable?}
    CheckFix -->|No| Report[Generate Report]

    Fixable -->|Yes| DryRun{Dry Run?}
    Fixable -->|No| CannotFix[Report: Cannot Fix]

    DryRun -->|Yes| PreviewFix[Preview Fixes]
    DryRun -->|No| ApplyFix[Apply Fixes]

    ApplyFix --> VerifyFix[Verify Fix]
    VerifyFix --> FixSuccess{Fix Valid?}
    FixSuccess -->|Yes| WriteFile[Write File]
    FixSuccess -->|No| FixFailed[Report: Fix Failed]

    PreviewFix --> Report
    WriteFile --> ReValidate[Re-validate File]
    ReValidate --> Report
    CannotFix --> Report
    UTF8Error --> Report
    FixFailed --> Report

    Report --> AllPass{All Files Pass?}
    AllPass -->|Yes| Success[Exit Code 0: Success]
    AllPass -->|No| Failure[Exit Code 1: Validation Failed]

    style Parse fill:#e3f2fd
    style RunValidators fill:#fff9c4
    style ApplyFix fill:#f3e5f5
    style Report fill:#e8f5e9
    style Success fill:#c8e6c9
    style Failure fill:#ffcdd2
```

## Module Structure

```mermaid
graph TD
    subgraph "src/"
        Main[main.rs Entry Point] --> CLI[cli.rs CLI Parsing]
        Main --> Lib[lib.rs Library Interface]

        CLI --> FileOps[file_ops.rs File I/O]
        CLI --> Fixer[fixer.rs Auto-Fix Logic]

        Lib --> Validators[validators/ Validation Logic]
        Validators --> ModV[mod.rs Trait Definition]
        Validators --> UTF8V[utf8.rs]
        Validators --> ASCIIV[ascii.rs]
        Validators --> UnprintV[unprintable.rs]
        Validators --> TreeV[tree_symbols.rs]

        Main --> Reporter[reporter.rs Output Formatting]
    end

    style Main fill:#42a5f5
    style Validators fill:#ffa726
    style Reporter fill:#66bb6a
```

## Technology Stack

```mermaid
graph LR
    subgraph "Core Technologies"
        A[Rust 2024 Edition] --> B[Cargo Build System]
        A --> C[std Library]
    end

    subgraph "Dependencies"
        D[clap 4.x] --> E[CLI Parsing]
        F[glob] --> G[Pattern Matching]
    end

    subgraph "Development Tools"
        H[cargo test] --> I[Unit Testing]
        H --> J[Integration Testing]
        K[cargo clippy] --> L[Linting]
        M[cargo fmt] --> N[Formatting]
    end

    B --> D
    B --> F
    B --> H

    style A fill:#dea584
    style D fill:#66bb6a
    style H fill:#42a5f5
```

## Layered Architecture

The system follows a clear layered architecture:

### Layer 1: CLI Interface
- **Responsibility**: Parse user input and validate arguments
- **Technology**: clap crate
- **Output**: Configuration object

### Layer 2: File Operations
- **Responsibility**: Read files, resolve paths, handle glob patterns
- **Technology**: Rust std::fs, glob crate
- **Output**: File contents as strings

### Layer 3: Validation Engine
- **Responsibility**: Run validators, collect results
- **Technology**: Custom validator trait implementation
- **Output**: Validation results with error details

### Layer 4: Auto-Fix System
- **Responsibility**: Analyze fixable violations and apply corrections
- **Technology**: Custom character replacement logic
- **Output**: Fixed file contents

### Layer 5: Reporter
- **Responsibility**: Format output, display errors, set exit codes
- **Technology**: Rust std I/O
- **Output**: Console output and process exit code

## Key Design Patterns

### Strategy Pattern
Each validator implements the `Validator` trait, allowing easy extension and testing:

```rust
pub trait Validator {
    fn name(&self) -> &str;
    fn validate(&self, content: &str) -> ValidationResult;
}
```

### Builder Pattern
Configuration is built from CLI arguments with validation:

```rust
pub struct Config {
    pub path: PathBuf,
    pub filename: String,
    pub verbose: bool,
    pub fix: bool,
    pub dry_run: bool,
}
```

### Chain of Responsibility
Multiple validators process content in sequence, each adding their results to the collection.

## Error Handling Strategy

```mermaid
graph TD
    Error[Error Occurs] --> Type{Error Type?}

    Type -->|File Error| FileErr[FileError]
    Type -->|Validation Error| ValErr[ValidationError]
    Type -->|CLI Error| CLIErr[ArgumentError]

    FileErr --> FileTypes{Specific Type?}
    FileTypes -->|Not Found| Exit2A[Exit Code 2]
    FileTypes -->|Permission| Exit2B[Exit Code 2]
    FileTypes -->|IO Error| Exit2C[Exit Code 2]

    ValErr --> Collect[Collect All Violations]
    Collect --> Report[Display Report]
    Report --> Exit1[Exit Code 1]

    CLIErr --> ShowUsage[Display Usage Help]
    ShowUsage --> Exit2D[Exit Code 2]

    style FileErr fill:#ef5350
    style ValErr fill:#ffa726
    style CLIErr fill:#ab47bc
```

## Performance Characteristics

- **Single-pass validation**: File content is read once
- **Lazy evaluation**: Validators don't run if earlier errors prevent it
- **Memory efficient**: Line-by-line processing for error reporting
- **Fast glob expansion**: Uses efficient pattern matching
- **Minimal allocations**: Reuses buffers where possible

## Extensibility Points

The architecture supports easy extension:

1. **New Validators**: Implement the `Validator` trait
2. **New Fix Strategies**: Extend the `Fixer` module
3. **New Output Formats**: Add to the `Reporter` module
4. **New File Sources**: Extend `FileOps` module

## Security Considerations

- **Path traversal prevention**: Validates file paths
- **Safe file operations**: Uses Rust's safe file I/O
- **No code execution**: Pure data validation
- **No network access**: Fully offline tool

## Related Documentation

- [Key Components](Components) - Detailed component documentation
- [Workflows & Sequences](Workflows) - Sequence diagrams for operations
- [Validation Engine](Validation-Engine) - Deep dive into validators
- [Testing Strategy](Testing-Strategy) - Testing approach
