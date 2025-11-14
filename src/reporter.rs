use crate::{ValidationResult, ValidationStatus};

pub fn format_results(results: &[ValidationResult], file_path: &str, verbose: bool) -> String {
    let mut output = String::new();

    if verbose {
        output.push_str(&format!("Checking file: {}\n\n", file_path));
        output.push_str("Running validators...\n");
        for (i, result) in results.iter().enumerate() {
            let status_symbol = if result.is_pass() { "✓" } else { "✗" };
            output.push_str(&format!(
                "[{}/{}] {}... {} {}\n",
                i + 1,
                results.len(),
                result.validator_name,
                status_symbol,
                if result.is_pass() { "Pass" } else { "Fail" }
            ));
        }
        output.push('\n');
    }

    let all_pass = results.iter().all(|r| r.is_pass());

    if all_pass {
        output.push_str(&format!("✓ File validation successful: {}\n", file_path));
    } else {
        output.push_str(&format!("✗ File validation failed: {}\n\n", file_path));

        for result in results {
            let status_symbol = if result.is_pass() { "✓" } else { "✗" };
            let status_text = if result.is_pass() { "Pass" } else { "Fail" };

            output.push_str(&format!(
                "{}: {} {}",
                result.validator_name, status_symbol, status_text
            ));

            if !result.errors.is_empty() {
                output.push_str(&format!(" ({} errors)", result.errors.len()));
            }
            output.push('\n');

            for error in &result.errors {
                output.push_str(&format!("  {}\n", error));
            }
        }
    }

    output
}

pub fn should_exit_with_error(results: &[ValidationResult]) -> bool {
    results.iter().any(|r| r.status == ValidationStatus::Fail)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ValidationError;

    #[test]
    fn test_format_success() {
        let results = vec![ValidationResult::pass("Test Validator".to_string())];
        let output = format_results(&results, "test.md", false);
        assert!(output.contains("✓"));
        assert!(output.contains("successful"));
        assert!(output.contains("test.md"));
    }

    #[test]
    fn test_format_failure() {
        let errors = vec![ValidationError::new(1, "Error 1".to_string())];
        let results = vec![ValidationResult::fail("Test Validator".to_string(), errors)];
        let output = format_results(&results, "test.md", false);
        assert!(output.contains("✗"));
        assert!(output.contains("failed"));
        assert!(output.contains("Error 1"));
    }

    #[test]
    fn test_verbose_output() {
        let results = vec![
            ValidationResult::pass("Validator 1".to_string()),
            ValidationResult::pass("Validator 2".to_string()),
        ];
        let output = format_results(&results, "test.md", true);
        assert!(output.contains("Checking file:"));
        assert!(output.contains("[1/2]"));
        assert!(output.contains("[2/2]"));
        assert!(output.contains("Running validators"));
    }

    #[test]
    fn test_should_exit_with_error_on_failure() {
        let results = vec![
            ValidationResult::pass("Good".to_string()),
            ValidationResult::fail("Bad".to_string(), vec![]),
        ];
        assert!(should_exit_with_error(&results));
    }

    #[test]
    fn test_should_not_exit_on_success() {
        let results = vec![
            ValidationResult::pass("Good 1".to_string()),
            ValidationResult::pass("Good 2".to_string()),
        ];
        assert!(!should_exit_with_error(&results));
    }
}
