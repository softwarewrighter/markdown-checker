use crate::{ValidationError, ValidationResult, Validator};

pub struct UnprintableValidator;

impl UnprintableValidator {
    fn is_allowed_whitespace(ch: char) -> bool {
        matches!(ch, ' ' | '\t' | '\n' | '\r')
    }

    fn is_printable(ch: char) -> bool {
        let code = ch as u32;
        // Printable ASCII: 32-126
        (32..=126).contains(&code) || Self::is_allowed_whitespace(ch)
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
                    errors.push(
                        ValidationError::new(
                            line_num + 1,
                            format!("Unprintable character: U+{:04X}", ch as u32),
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
    fn test_allowed_whitespace() {
        let content = "Line 1\nLine 2\tTabbed\r\n";
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert!(result.is_pass());
    }

    #[test]
    fn test_null_byte_detected() {
        let content = "Hello\0World";
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert!(result.is_fail());
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("U+0000"));
    }

    #[test]
    fn test_control_chars() {
        let content = "Hello\x07World"; // Bell character
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert!(result.is_fail());
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("U+0007"));
    }

    #[test]
    fn test_printable_ascii_passes() {
        let content =
            "abcdefghijklmnopqrstuvwxyz\nABCDEFGHIJKLMNOPQRSTUVWXYZ\n0123456789\n!@#$%^&*()";
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert!(result.is_pass());
    }

    #[test]
    fn test_line_numbers_correct() {
        let content = "OK\nBad\x00\nOK";
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert!(result.is_fail());
        assert_eq!(result.errors[0].line_number, 2);
    }

    #[test]
    fn test_tab_is_allowed() {
        let content = "Before\tAfter";
        let validator = UnprintableValidator;
        let result = validator.validate(content);
        assert!(result.is_pass());
    }
}
