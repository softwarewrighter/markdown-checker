use clap::Parser;
use std::path::PathBuf;

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\n\nCopyright (c) 2025 Michael A Wright\n",
    "License: MIT\n",
    "Repository: https://github.com/softwarewrighter/markdown-checker"
);

#[derive(Parser, Debug)]
#[command(name = "markdown-checker")]
#[command(author = "Michael A Wright")]
#[command(version = VERSION_INFO)]
#[command(about = "Validates markdown files for UTF-8, ASCII-subset, and unprintable characters")]
#[command(long_about = "\
Markdown Checker - Validate and Auto-Fix Markdown Files

PURPOSE:
  Ensures markdown files contain only UTF-8 encoded, ASCII-subset characters
  without unprintable control characters or non-standard tree visualization symbols.
  Helps maintain portable, accessible documentation that works across all editors,
  terminals, CI/CD pipelines, and accessibility tools.

CAPABILITIES:
  • UTF-8 Validation: Ensures files are valid UTF-8 encoded text
  • ASCII Subset: Detects any character outside ASCII range (0-127)
  • Unprintable Detection: Finds control characters and other unprintable bytes
  • Tree Symbol Detection: Identifies box-drawing characters (U+2500-U+257F)
  • Auto-Fix: Automatically replaces tree symbols with ASCII equivalents
  • Dry-Run Mode: Preview fixes before applying them
  • Glob Patterns: Process multiple files matching a pattern
  • Detailed Reporting: Shows exact line/column numbers for violations

VALIDATION RULES:
  ✓ Valid: ASCII characters (32-126) plus whitespace (space, tab, LF, CR)
  ✗ Invalid: Unicode characters, emojis, accents, box-drawing symbols

  Auto-fixable: Tree symbols (├ → +, └ → +, │ → |, ─ → -)
  Not auto-fixable: Other Unicode, unprintable characters (unclear intent)

USAGE PATTERNS:
  # Validate single file
  markdown-checker                          # Checks ./README.md
  markdown-checker -f CONTRIBUTING.md       # Checks ./CONTRIBUTING.md
  markdown-checker -p docs -f overview.md   # Checks docs/overview.md

  # Validate multiple files with glob patterns
  markdown-checker -f \"*.md\"              # All .md files in current dir
  markdown-checker -p docs -f \"**/*.md\"   # All .md files in docs/ recursively

  # Auto-fix tree symbols
  markdown-checker --fix                    # Fix violations in README.md
  markdown-checker -f \"*.md\" --fix        # Fix all .md files

  # Preview fixes (dry-run)
  markdown-checker --dry-run                # Show what would be fixed
  markdown-checker -n -f \"*.md\"           # Preview fixes for all files

  # Verbose output
  markdown-checker -v                       # Detailed validation progress
  markdown-checker -v --dry-run             # Verbose dry-run mode

EXIT CODES:
  0 - Success: All files pass validation (or fixed successfully)
  1 - Failure: Validation errors found
  2 - Error: File not found, invalid arguments, or other errors

SAFETY:
  • --fix only works if ALL violations are auto-fixable (tree symbols only)
  • Files with ASCII or unprintable violations cannot be auto-fixed
  • Dry-run mode never modifies files, only shows what would change
  • Fix changes are verified before writing

For more information and examples, visit:
https://github.com/softwarewrighter/markdown-checker
")]
pub struct Cli {
    /// Path to directory containing the file (default: current directory)
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    /// Name of the file to check or glob pattern (default: README.md)
    #[arg(short = 'f', long, value_name = "NAME", default_value = "README.md")]
    pub file_name: String,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Automatically fix violations where possible (tree symbols only)
    #[arg(long)]
    pub fix: bool,

    /// Preview fixes without applying them (dry-run mode)
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub filename: String,
    pub verbose: bool,
    pub fix: bool,
    pub dry_run: bool,
}

impl Config {
    pub fn from_cli(cli: Cli) -> Self {
        Self {
            path: cli.path,
            filename: cli.file_name,
            verbose: cli.verbose,
            fix: cli.fix,
            dry_run: cli.dry_run,
        }
    }

    pub fn file_path(&self) -> PathBuf {
        self.path.join(&self.filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_file_path() {
        let config = Config {
            path: PathBuf::from("/tmp"),
            filename: "test.md".to_string(),
            verbose: false,
            fix: false,
            dry_run: false,
        };
        assert_eq!(config.file_path(), PathBuf::from("/tmp/test.md"));
    }

    #[test]
    fn test_config_file_path_current_dir() {
        let config = Config {
            path: PathBuf::from("."),
            filename: "README.md".to_string(),
            verbose: false,
            fix: false,
            dry_run: false,
        };
        assert_eq!(config.file_path(), PathBuf::from("./README.md"));
    }
}
