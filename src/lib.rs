// Core validation types and traits

use std::fmt;

/// Status of a validation check
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationStatus {
    Pass,
    Fail,
}

/// Error found during validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub line_number: usize,
    pub column: Option<usize>,
    pub message: String,
    pub context: Option<String>,
}

impl ValidationError {
    pub fn new(line_number: usize, message: String) -> Self {
        Self {
            line_number,
            column: None,
            message,
            context: None,
        }
    }

    pub fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}", self.line_number)?;
        if let Some(col) = self.column {
            write!(f, ", Column {}", col)?;
        }
        write!(f, ": {}", self.message)?;
        Ok(())
    }
}

/// Result of running a validator
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub validator_name: String,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn pass(validator_name: String) -> Self {
        Self {
            status: ValidationStatus::Pass,
            validator_name,
            errors: Vec::new(),
        }
    }

    pub fn fail(validator_name: String, errors: Vec<ValidationError>) -> Self {
        Self {
            status: ValidationStatus::Fail,
            validator_name,
            errors,
        }
    }

    pub fn is_pass(&self) -> bool {
        self.status == ValidationStatus::Pass
    }

    pub fn is_fail(&self) -> bool {
        self.status == ValidationStatus::Fail
    }
}

/// Trait for validators
pub trait Validator {
    fn name(&self) -> &str;
    fn validate(&self, content: &str) -> ValidationResult;
}

pub mod cli;
pub mod file_ops;
pub mod reporter;
pub mod validators;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::new(5, "Test error".to_string());
        assert_eq!(error.line_number, 5);
        assert_eq!(error.column, None);
        assert_eq!(error.message, "Test error");
        assert_eq!(error.context, None);
    }

    #[test]
    fn test_validation_error_with_column() {
        let error = ValidationError::new(10, "Error".to_string()).with_column(15);
        assert_eq!(error.line_number, 10);
        assert_eq!(error.column, Some(15));
    }

    #[test]
    fn test_validation_error_with_context() {
        let error =
            ValidationError::new(3, "Error".to_string()).with_context("line content".to_string());
        assert_eq!(error.context, Some("line content".to_string()));
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::new(5, "Test error".to_string()).with_column(10);
        let display = format!("{}", error);
        assert!(display.contains("Line 5"));
        assert!(display.contains("Column 10"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_validation_result_pass() {
        let result = ValidationResult::pass("Test Validator".to_string());
        assert_eq!(result.status, ValidationStatus::Pass);
        assert_eq!(result.validator_name, "Test Validator");
        assert!(result.errors.is_empty());
        assert!(result.is_pass());
        assert!(!result.is_fail());
    }

    #[test]
    fn test_validation_result_fail() {
        let errors = vec![
            ValidationError::new(1, "Error 1".to_string()),
            ValidationError::new(2, "Error 2".to_string()),
        ];
        let result = ValidationResult::fail("Test Validator".to_string(), errors);
        assert_eq!(result.status, ValidationStatus::Fail);
        assert_eq!(result.validator_name, "Test Validator");
        assert_eq!(result.errors.len(), 2);
        assert!(!result.is_pass());
        assert!(result.is_fail());
    }
}
