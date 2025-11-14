use clap::Parser;
use markdown_checker::cli::{Cli, Config};
use markdown_checker::file_ops::read_file_content;
use markdown_checker::reporter::{format_results, should_exit_with_error};
use markdown_checker::validators::validate_all;
use std::process;

fn main() {
    let cli = Cli::parse();
    let config = Config::from_cli(cli);

    let file_path = config.file_path();

    // Read file content
    let content = match read_file_content(&file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path.display(), e);
            process::exit(2);
        }
    };

    // Run all validators
    let results = validate_all(&content);

    // Format and display results
    let output = format_results(&results, &file_path.display().to_string(), config.verbose);
    print!("{}", output);

    // Exit with appropriate code
    if should_exit_with_error(&results) {
        process::exit(1);
    }
}
