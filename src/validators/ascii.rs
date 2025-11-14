use crate::{ValidationError, ValidationResult, Validator};

pub struct AsciiValidator;

impl Validator for AsciiValidator {
    fn name(&self) -> &str {
        "ASCII Subset"
    }

    fn validate(&self, content: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if (ch as u32) > 127 {
                    errors.push(
                        ValidationError::new(
                            line_num + 1,
                            format!("Non-ASCII character: '{}' (U+{:04X})", ch, ch as u32),
                        )
                        .with_column(col + 1),
                    );
                }
            }
        }

        if errors.is_empty() {
            ValidationResult::pass(self.name().to_string())
        } else {
            ValidationResult::fail(self.name().to_string(), errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_ascii_passes() {
        let validator = AsciiValidator;
        let result = validator.validate("Hello, World!\nThis is a test.");
        assert!(result.is_pass());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_unicode_emoji_fails() {
        let validator = AsciiValidator;
        let result = validator.validate("Hello 👋");
        assert!(result.is_fail());
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].line_number, 1);
        assert!(result.errors[0].message.contains("👋"));
    }

    #[test]
    fn test_accented_chars_fail() {
        let validator = AsciiValidator;
        let result = validator.validate("café");
        assert!(result.is_fail());
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].line_number, 1);
        assert!(result.errors[0].message.contains("é"));
    }

    #[test]
    fn test_multiple_violations() {
        let validator = AsciiValidator;
        let result = validator.validate("Line 1: café\nLine 2: naïve\nLine 3: 日本語");
        assert!(result.is_fail());
        assert!(result.errors.len() >= 3);
    }

    #[test]
    fn test_line_numbers_correct() {
        let validator = AsciiValidator;
        let result = validator.validate("OK\nBad: ñ\nOK");
        assert!(result.is_fail());
        assert_eq!(result.errors[0].line_number, 2);
    }

    #[test]
    fn test_column_numbers_reported() {
        let validator = AsciiValidator;
        let result = validator.validate("café");
        assert!(result.is_fail());
        assert!(result.errors[0].column.is_some());
    }
}
