use crate::{ValidationError, ValidationResult, Validator};

pub struct TreeSymbolValidator;

impl TreeSymbolValidator {
    const TREE_SYMBOLS: &'static [char] = &[
        '├', '└', '│', '─', '┌', '┐', '┘', '┤', '┴', '┬', '┼', '╭', '╮', '╯', '╰', '╱', '╲', '╳',
    ];

    fn is_tree_symbol(ch: char) -> bool {
        Self::TREE_SYMBOLS.contains(&ch) || (ch as u32 >= 0x2500 && ch as u32 <= 0x257F)
        // Box Drawing block
    }

    fn suggest_alternative(ch: char) -> &'static str {
        match ch {
            '├' | '┤' => "Use '+' or '|' instead",
            '└' | '┘' | '┌' | '┐' => "Use '+' or '`' instead",
            '│' => "Use '|' instead",
            '─' => "Use '-' instead",
            _ => "Use standard ASCII characters like |, +, -, `",
        }
    }
}

impl Validator for TreeSymbolValidator {
    fn name(&self) -> &str {
        "Tree Symbols"
    }

    fn validate(&self, content: &str) -> ValidationResult {
        let mut errors = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if Self::is_tree_symbol(ch) {
                    errors.push(
                        ValidationError::new(
                            line_num + 1,
                            format!(
                                "Tree symbol '{}' (U+{:04X}) detected. {}",
                                ch,
                                ch as u32,
                                Self::suggest_alternative(ch)
                            ),
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
    fn test_no_tree_symbols_passes() {
        let content = "Project structure:\n+ src/\n  + main.rs\n";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.is_pass());
    }

    #[test]
    fn test_detects_common_tree_chars() {
        let content = "├── src/\n│   └── main.rs\n";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.is_fail());
        // ├, ─, ─, │, └, ─, ─ = 7 tree symbols
        assert_eq!(result.errors.len(), 7);
    }

    #[test]
    fn test_provides_suggestions() {
        let content = "├── file";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].message.contains("instead"));
    }

    #[test]
    fn test_box_drawing_range() {
        // Test characters in U+2500 - U+257F range
        let content = "─━│┃┄┅┆┇";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.is_fail());
        assert!(result.errors.len() >= 5);
    }

    #[test]
    fn test_line_numbers_correct() {
        let content = "OK\n├── bad\nOK";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.is_fail());
        assert_eq!(result.errors[0].line_number, 2);
    }

    #[test]
    fn test_vertical_bar_suggestion() {
        let content = "│";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.is_fail());
        assert!(result.errors[0].message.contains("Use '|' instead"));
    }

    #[test]
    fn test_horizontal_bar_suggestion() {
        let content = "─";
        let validator = TreeSymbolValidator;
        let result = validator.validate(content);
        assert!(result.is_fail());
        assert!(result.errors[0].message.contains("Use '-' instead"));
    }
}
