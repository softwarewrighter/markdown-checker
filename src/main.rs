use clap::Parser;
use glob::glob;
use markdown_checker::cli::{Cli, Config};
use markdown_checker::file_ops::{read_file_content, write_file_content};
use markdown_checker::fixer::fix_tree_symbols;
use markdown_checker::reporter::{format_results, should_exit_with_error};
use markdown_checker::validators::validate_all;
use std::path::PathBuf;
use std::process;

fn main() {
    let cli = Cli::parse();
    let config = Config::from_cli(cli);

    // Resolve file pattern to list of files
    let files = match resolve_files(&config) {
        Ok(f) if f.is_empty() => {
            eprintln!("No files found matching pattern: {}", config.filename);
            process::exit(2);
        }
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error resolving file pattern: {}", e);
            process::exit(2);
        }
    };

    let mut overall_success = true;
    let mut files_processed = 0;

    for file_path in &files {
        // Read file content
        let content = match read_file_content(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path.display(), e);
                overall_success = false;
                continue;
            }
        };

        // Run all validators
        let results = validate_all(&content);

        // Check if we need to fix anything
        let needs_fixing = results.iter().any(|r| r.is_fail());

        if config.fix || config.dry_run {
            if needs_fixing {
                // Try to fix by replacing tree symbols
                let fixed_content = fix_tree_symbols(&content);

                // Re-validate the fixed content to see if all violations are resolved
                let fixed_results = validate_all(&fixed_content);
                let all_fixed = fixed_results.iter().all(|r| r.is_pass());

                if all_fixed {
                    // All violations were tree symbols and have been fixed
                    let tree_result = results.iter().find(|r| r.validator_name == "Tree Symbols");
                    let violation_count = tree_result.map(|r| r.errors.len()).unwrap_or(0);

                    if config.dry_run {
                        // Dry-run mode: show what would be changed
                        println!("🔍 Dry-run mode for: {}", file_path.display());
                        println!("   Would fix {} tree symbol violation(s)", violation_count);
                        if config.verbose {
                            println!("\nOriginal violations:");
                            let output = format_results(&results, &file_path.display().to_string(), false);
                            print!("{}", output);
                            println!("\n✓ After fix: All violations would be resolved");
                        }
                        println!();
                    } else {
                        // Apply the fix
                        match write_file_content(file_path, &fixed_content) {
                            Ok(_) => {
                                println!("✓ Fixed {} tree symbol violation(s) in: {}",
                                       violation_count,
                                       file_path.display());
                            }
                            Err(e) => {
                                eprintln!("✗ Error writing fixed content to {}: {}", file_path.display(), e);
                                overall_success = false;
                            }
                        }
                    }
                } else {
                    // File has non-tree-symbol violations that cannot be auto-fixed
                    let output = format_results(&results, &file_path.display().to_string(), config.verbose);
                    print!("{}", output);
                    eprintln!("\n⚠️  Cannot auto-fix: File contains non-fixable violations.");
                    eprintln!("Common Unicode characters can be auto-fixed (tree symbols, checkmarks, arrows, accents, quotes, etc.).");
                    eprintln!("This file has other Unicode characters or unprintable control characters that cannot be safely converted.");
                    overall_success = false;
                }
            } else {
                // No violations
                if config.verbose || files.len() == 1 {
                    println!("✓ File validation successful: {}", file_path.display());
                }
            }
        } else {
            // Normal validation mode (no fix/dry-run)
            let output = format_results(&results, &file_path.display().to_string(), config.verbose);
            print!("{}", output);

            if should_exit_with_error(&results) {
                overall_success = false;
            }
        }

        files_processed += 1;
    }

    if files.len() > 1 {
        println!("\n📊 Processed {} file(s)", files_processed);
    }

    if !overall_success {
        process::exit(1);
    }
}

/// Resolve file pattern (glob or single file) to list of file paths
fn resolve_files(config: &Config) -> Result<Vec<PathBuf>, String> {
    // Check if filename contains glob patterns
    if config.filename.contains('*') || config.filename.contains('?') || config.filename.contains('[') {
        // It's a glob pattern
        let pattern = config.file_path().display().to_string();
        let mut paths = Vec::new();

        for entry in glob(&pattern).map_err(|e| format!("Invalid glob pattern: {}", e))? {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        paths.push(path);
                    }
                }
                Err(e) => eprintln!("Warning: Error reading glob entry: {}", e),
            }
        }

        Ok(paths)
    } else {
        // It's a single file path
        let path = config.file_path();
        if path.exists() {
            Ok(vec![path])
        } else {
            Err(format!("File not found: {}", path.display()))
        }
    }
}
