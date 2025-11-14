pub mod ascii;
pub mod tree_symbols;
pub mod unprintable;

pub use ascii::AsciiValidator;
pub use tree_symbols::TreeSymbolValidator;
pub use unprintable::UnprintableValidator;

use crate::{ValidationResult, Validator};

/// Run all validators on the content
pub fn validate_all(content: &str) -> Vec<ValidationResult> {
    vec![
        AsciiValidator.validate(content),
        UnprintableValidator.validate(content),
        TreeSymbolValidator.validate(content),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_all_returns_all_validators() {
        let content = "test";
        let results = validate_all(content);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_validate_all_with_violations() {
        let content = "├── test";
        let results = validate_all(content);
        let failed: Vec<_> = results.iter().filter(|r| r.is_fail()).collect();
        assert!(!failed.is_empty());
    }
}
