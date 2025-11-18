# Workflows & Sequence Diagrams

This page documents the key workflows and operational sequences in Markdown Checker.

## Table of Contents

1. [Basic Validation Workflow](#basic-validation-workflow)
2. [Auto-Fix Workflow](#auto-fix-workflow)
3. [Glob Pattern Processing](#glob-pattern-processing)
4. [Validation Engine Sequence](#validation-engine-sequence)
5. [Error Handling Flow](#error-handling-flow)
6. [Dry-Run Mode Workflow](#dry-run-mode-workflow)

---

## Basic Validation Workflow

This sequence shows a standard file validation without fix mode:

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Parser
    participant Config as Configuration
    participant FileOps as File Operations
    participant Engine as Validation Engine
    participant Reporter as Reporter

    User->>CLI: markdown-checker -v
    CLI->>CLI: Parse arguments
    CLI->>Config: Build config object
    Config->>FileOps: Read file at path
    FileOps->>FileOps: Check file exists
    FileOps->>FileOps: Read file contents
    FileOps-->>Config: Return file contents

    Config->>Engine: Validate content
    Engine->>Engine: Run UTF-8 validator
    Engine->>Engine: Run ASCII validator
    Engine->>Engine: Run Unprintable validator
    Engine->>Engine: Run Tree Symbol validator
    Engine->>Engine: Aggregate results
    Engine-->>Config: Return validation results

    Config->>Reporter: Format results
    Reporter->>Reporter: Determine overall status
    Reporter->>Reporter: Format error messages
    Reporter->>User: Display report
    Reporter->>Reporter: Set exit code

    alt All validations pass
        Reporter->>User: Exit code 0 (Success)
    else Any validation fails
        Reporter->>User: Exit code 1 (Failure)
    end
```

---

## Auto-Fix Workflow

This sequence shows the auto-fix operation with validation:

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Parser
    participant FileOps as File Operations
    participant Engine as Validation Engine
    participant Fixer as Auto-Fix System
    participant Reporter as Reporter

    User->>CLI: markdown-checker --fix
    CLI->>CLI: Parse --fix flag
    CLI->>FileOps: Read file

    FileOps-->>CLI: File contents
    CLI->>Engine: Validate content
    Engine-->>CLI: Validation results

    alt No violations found
        CLI->>Reporter: Display "No violations"
        Reporter->>User: Exit code 0
    else Violations found
        CLI->>Fixer: Analyze violations
        Fixer->>Fixer: Check if all fixable

        alt All violations are fixable (tree symbols)
            Fixer->>Fixer: Generate replacement map
            Fixer->>Fixer: Apply character replacements
            Fixer-->>CLI: Fixed content

            CLI->>Engine: Re-validate fixed content
            Engine-->>CLI: Validation results

            alt Re-validation passes
                CLI->>FileOps: Write fixed content
                FileOps->>FileOps: Create backup
                FileOps->>FileOps: Write file
                FileOps-->>CLI: Write success
                CLI->>Reporter: Display "Fixed successfully"
                Reporter->>User: Exit code 0
            else Re-validation fails
                CLI->>Reporter: Display "Fix verification failed"
                Reporter->>User: Exit code 1
            end
        else Contains unfixable violations
            CLI->>Reporter: Display "Cannot auto-fix"
            Reporter->>Reporter: Show unfixable violations
            Reporter->>User: Exit code 1
        end
    end
```

---

## Glob Pattern Processing

This sequence shows how multiple files are processed with glob patterns:

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Parser
    participant Glob as Glob Processor
    participant FileOps as File Operations
    participant Engine as Validation Engine
    participant Reporter as Reporter

    User->>CLI: markdown-checker -f "docs/**/*.md"
    CLI->>CLI: Parse glob pattern
    CLI->>Glob: Expand pattern
    Glob->>Glob: Search filesystem
    Glob->>Glob: Match files
    Glob-->>CLI: List of file paths

    loop For each matched file
        CLI->>FileOps: Read file
        FileOps-->>CLI: File contents
        CLI->>Engine: Validate content
        Engine-->>CLI: Validation results
        CLI->>Reporter: Collect results
    end

    Reporter->>Reporter: Aggregate all results
    Reporter->>Reporter: Format summary report

    alt All files pass
        Reporter->>User: Show success for all files
        Reporter->>User: Exit code 0
    else Any file fails
        Reporter->>User: Show failures with file paths
        Reporter->>User: Exit code 1
    end
```

---

## Validation Engine Sequence

This detailed sequence shows how validators are executed:

```mermaid
sequenceDiagram
    participant Engine as Validation Engine
    participant UTF8 as UTF-8 Validator
    participant ASCII as ASCII Validator
    participant Unprint as Unprintable Validator
    participant Tree as Tree Symbol Validator
    participant Aggregator as Results Aggregator

    Engine->>UTF8: validate(content)
    UTF8->>UTF8: Check byte sequences
    UTF8-->>Engine: ValidationResult

    Engine->>ASCII: validate(content)
    ASCII->>ASCII: Iterate characters
    ASCII->>ASCII: Check char > 127
    loop For each line
        loop For each character
            ASCII->>ASCII: Check ASCII range
            alt Non-ASCII found
                ASCII->>ASCII: Record violation
            end
        end
    end
    ASCII-->>Engine: ValidationResult

    Engine->>Unprint: validate(content)
    Unprint->>Unprint: Iterate characters
    loop For each character
        Unprint->>Unprint: Check if printable
        alt Unprintable found
            Unprint->>Unprint: Check allowed whitespace
            alt Not allowed
                Unprint->>Unprint: Record violation
            end
        end
    end
    Unprint-->>Engine: ValidationResult

    Engine->>Tree: validate(content)
    Tree->>Tree: Iterate characters
    loop For each character
        Tree->>Tree: Check if tree symbol
        alt Tree symbol found
            Tree->>Tree: Generate suggestion
            Tree->>Tree: Record violation
        end
    end
    Tree-->>Engine: ValidationResult

    Engine->>Aggregator: Collect all results
    Aggregator->>Aggregator: Determine overall status
    Aggregator->>Aggregator: Count total violations
    Aggregator-->>Engine: Aggregated results
```

---

## Error Handling Flow

This sequence shows how different error types are handled:

```mermaid
sequenceDiagram
    participant User
    participant CLI as CLI Parser
    participant FileOps as File Operations
    participant Engine as Validation Engine
    participant Reporter as Reporter

    User->>CLI: markdown-checker -f nonexistent.md

    CLI->>FileOps: Read file
    FileOps->>FileOps: Check file exists

    alt File not found
        FileOps-->>CLI: FileError::NotFound
        CLI->>Reporter: Handle file error
        Reporter->>User: "Error: File not found: nonexistent.md"
        Reporter->>User: Exit code 2
    else Permission denied
        FileOps-->>CLI: FileError::PermissionDenied
        CLI->>Reporter: Handle file error
        Reporter->>User: "Error: Permission denied"
        Reporter->>User: Exit code 2
    else Invalid UTF-8
        FileOps-->>CLI: FileError::InvalidUtf8
        CLI->>Reporter: Handle encoding error
        Reporter->>User: "Error: File is not valid UTF-8"
        Reporter->>User: Exit code 1
    else I/O error
        FileOps-->>CLI: FileError::IoError
        CLI->>Reporter: Handle I/O error
        Reporter->>User: "Error: I/O error reading file"
        Reporter->>User: Exit code 2
    else File read successfully
        FileOps-->>CLI: File contents
        CLI->>Engine: Validate
        Engine-->>CLI: Results
        CLI->>Reporter: Format results
        alt Validation passes
            Reporter->>User: Success message
            Reporter->>User: Exit code 0
        else Validation fails
            Reporter->>User: Error details
            Reporter->>User: Exit code 1
        end
    end
```

---

## Dry-Run Mode Workflow

This sequence shows the preview-only mode for fixes:

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Parser
    participant FileOps as File Operations
    participant Engine as Validation Engine
    participant Fixer as Auto-Fix System
    participant Reporter as Reporter

    User->>CLI: markdown-checker --dry-run
    CLI->>CLI: Set dry_run flag
    CLI->>FileOps: Read file
    FileOps-->>CLI: File contents

    CLI->>Engine: Validate content
    Engine-->>CLI: Validation results

    alt Violations found
        CLI->>Fixer: Analyze violations
        Fixer->>Fixer: Check fixability

        alt All fixable
            Fixer->>Fixer: Generate fixes
            Fixer->>Fixer: Create preview
            Fixer-->>CLI: Preview data

            CLI->>Reporter: Format preview
            Reporter->>Reporter: Build diff view
            Reporter->>User: Display preview:
            Reporter->>User: "Would fix X violations"
            Reporter->>User: "Line Y: 'old' -> 'new'"
            Reporter->>User: "(File not modified - dry run)"
            Reporter->>User: Exit code 0
        else Contains unfixable
            CLI->>Reporter: Show unfixable items
            Reporter->>User: "Cannot auto-fix all violations"
            Reporter->>User: Exit code 1
        end
    else No violations
        CLI->>Reporter: Show success
        Reporter->>User: "No violations found"
        Reporter->>User: Exit code 0
    end
```

---

## Verbose Mode Output Sequence

This sequence shows enhanced output in verbose mode:

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Parser
    participant FileOps as File Operations
    participant Engine as Validation Engine
    participant Reporter as Reporter

    User->>CLI: markdown-checker -v
    CLI->>CLI: Set verbose flag

    CLI->>Reporter: Display "Checking file: ./README.md"
    CLI->>FileOps: Read file
    FileOps->>FileOps: Get file metadata
    FileOps-->>CLI: File contents + size
    CLI->>Reporter: Display "File size: X bytes"

    CLI->>Reporter: Display "Running validators..."

    CLI->>Engine: Run validators

    loop For each validator
        Engine->>Engine: Execute validator
        Engine->>Reporter: Display "[N/M] Validator Name..."
        Engine->>Engine: Collect results

        alt Validator passes
            Engine->>Reporter: Display "Pass"
        else Validator fails
            Engine->>Reporter: Display "Fail (X errors)"
        end
    end

    Engine-->>CLI: All results
    CLI->>Reporter: Display "Results:"
    Reporter->>Reporter: Format detailed output
    Reporter->>User: Show all violations with context

    alt All pass
        Reporter->>User: "File validation successful"
        Reporter->>User: Exit code 0
    else Any fail
        Reporter->>User: "File validation failed"
        Reporter->>User: Detailed error list
        Reporter->>User: Exit code 1
    end
```

---

## Component Interaction Matrix

This diagram shows which components interact during different operations:

```mermaid
graph TB
    subgraph "Standard Validation"
        A1[CLI] --> B1[FileOps]
        B1 --> C1[Engine]
        C1 --> D1[Reporter]
    end

    subgraph "Auto-Fix Operation"
        A2[CLI] --> B2[FileOps]
        B2 --> C2[Engine]
        C2 --> E2[Fixer]
        E2 --> C2
        E2 --> B2
        C2 --> D2[Reporter]
    end

    subgraph "Glob Processing"
        A3[CLI] --> G3[Glob]
        G3 --> B3[FileOps]
        B3 --> C3[Engine]
        C3 --> D3[Reporter]
    end

    style E2 fill:#f3e5f5
    style G3 fill:#e1f5fe
```

---

## Related Documentation

- [Architecture Overview](Architecture.md) - System architecture diagrams
- [Key Components](Components.md) - Component details
- [Validation Engine](Validation-Engine.md) - Validator implementation details
- [Auto-Fix System](Auto-Fix-System.md) - Fix mechanism details
- [CLI Interface](CLI-Interface.md) - Command-line interface details
