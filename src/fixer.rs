//! Auto-fix module for correcting markdown violations.
//!
//! Currently supports:
//! - Tree symbols: Replaces Unicode box-drawing characters with ASCII equivalents
//! - Common Unicode characters: Checkmarks, arrows, accented letters, etc.

/// Fixes tree symbols and common Unicode characters by replacing them with ASCII equivalents.
pub fn fix_tree_symbols(content: &str) -> String {
    let mut fixed = String::with_capacity(content.len());

    for ch in content.chars() {
        match ch {
            // Box-drawing characters to ASCII (single char)
            '├' | '┤' | '┼' | '┬' | '┴' | '╋' => fixed.push('+'),
            '└' | '┘' | '┌' | '┐' | '╰' | '╯' | '╭' | '╮' => fixed.push('+'),
            '│' | '┃' | '║' => fixed.push('|'),
            '─' | '━' | '═' => fixed.push('-'),

            // Common Unicode symbols to ASCII (multi-char)
            '✓' | '✔' | '✅' | '❌' | '✗' | '✘' => fixed.push_str("[x]"),
            '→' | '⇒' | '⟶' | '➔' | '➜' | '➡' => fixed.push_str("->"),
            '←' | '⇐' | '⟵' | '➘' => fixed.push_str("<-"),
            '⬆' | '⇧' | '↑' => fixed.push('^'),
            '⬇' | '⇩' | '↓' => fixed.push('v'),
            '•' | '·' | '●' => fixed.push('*'),
            '…' => fixed.push_str("..."),
            '©' => fixed.push_str("(c)"),
            '®' => fixed.push_str("(R)"),
            '™' => fixed.push_str("(TM)"),
            '§' => fixed.push('S'),
            '°' => fixed.push_str(" degrees"),
            '±' => fixed.push_str("+/-"),
            '×' => fixed.push('x'),
            '÷' => fixed.push('/'),
            '≤' => fixed.push_str("<="),
            '≥' => fixed.push_str(">="),
            '≠' => fixed.push_str("!="),
            '≈' => fixed.push_str("~="),
            '∞' => fixed.push_str("infinity"),
            '🚧' => fixed.push_str("[WIP]"),
            '⚠' | '⚡' => fixed.push_str("[!]"),
            'α' => fixed.push_str("alpha"),
            'β' => fixed.push_str("beta"),
            'γ' => fixed.push_str("gamma"),
            'δ' => fixed.push_str("delta"),
            'π' => fixed.push_str("pi"),
            'Σ' => fixed.push_str("Sigma"),
            'µ' => fixed.push_str("micro"),

            // Accented letters (common European)
            'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' | 'ā' => fixed.push('a'),
            'é' | 'è' | 'ê' | 'ë' | 'ē' => fixed.push('e'),
            'í' | 'ì' | 'î' | 'ï' | 'ī' => fixed.push('i'),
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ō' => fixed.push('o'),
            'ú' | 'ù' | 'û' | 'ü' | 'ū' => fixed.push('u'),
            'ý' | 'ÿ' => fixed.push('y'),
            'ñ' => fixed.push('n'),
            'ç' => fixed.push('c'),
            'Á' | 'À' | 'Â' | 'Ä' | 'Ã' | 'Å' | 'Ā' => fixed.push('A'),
            'É' | 'È' | 'Ê' | 'Ë' | 'Ē' => fixed.push('E'),
            'Í' | 'Ì' | 'Î' | 'Ï' | 'Ī' => fixed.push('I'),
            'Ó' | 'Ò' | 'Ô' | 'Ö' | 'Õ' | 'Ō' => fixed.push('O'),
            'Ú' | 'Ù' | 'Û' | 'Ü' | 'Ū' => fixed.push('U'),
            'Ý' | 'Ÿ' => fixed.push('Y'),
            'Ñ' => fixed.push('N'),
            'Ç' => fixed.push('C'),

            // Quotation marks
            '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{201F}' => fixed.push('"'), // Smart double quotes (", ", „, ‟)
            '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' => fixed.push('\''), // Smart single quotes (', ', ‚, ‛)
            '«' | '»' => fixed.push('"'),

            // Dashes
            '–' => fixed.push('-'),
            '—' => fixed.push_str("--"),

            // Fallback for other box-drawing chars
            _ if is_box_drawing(ch) => fixed.push('+'),

            // Keep everything else as-is
            _ => fixed.push(ch),
        }
    }

    fixed
}

/// Check if a character is in the box-drawing Unicode block
fn is_box_drawing(ch: char) -> bool {
    let code = ch as u32;
    (0x2500..=0x257F).contains(&code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_basic_tree_symbols() {
        let input = "├── src/\n│   └── main.rs\n";
        let expected = "+-- src/\n|   +-- main.rs\n";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_preserves_ascii() {
        let input = "This is normal text\nWith normal characters";
        assert_eq!(fix_tree_symbols(input), input);
    }

    #[test]
    fn test_fix_horizontal_bars() {
        let input = "──────";
        let expected = "------";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_vertical_bars() {
        let input = "│\n│\n│";
        let expected = "|\n|\n|";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_mixed_content() {
        let input = "# Project\n├── docs/\n│   ├── README.md\n│   └── guide.md\n└── src/";
        let expected = "# Project\n+-- docs/\n|   +-- README.md\n|   +-- guide.md\n+-- src/";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_complex_box_drawing() {
        let input = "┌─┬─┐\n├─┼─┤\n└─┴─┘";
        // All corners and junctions → +, horizontal lines → -
        let expected = "+-+-+\n+-+-+\n+-+-+";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_is_box_drawing() {
        assert!(is_box_drawing('├'));
        assert!(is_box_drawing('─'));
        assert!(is_box_drawing('│'));
        assert!(is_box_drawing('└'));
        assert!(!is_box_drawing('a'));
        assert!(!is_box_drawing('1'));
        assert!(!is_box_drawing(' '));
    }

    #[test]
    fn test_fix_checkmarks() {
        let input = "✓ Task done\n✗ Task failed\n✅ Complete\n❌ Error";
        let expected = "[x] Task done\n[x] Task failed\n[x] Complete\n[x] Error";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_arrows() {
        let input = "a → b\nclick here ➜\nx ← y";
        let expected = "a -> b\nclick here ->\nx <- y";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_accented_letters() {
        let input = "Café naïve résumé";
        let expected = "Cafe naive resume";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_special_symbols() {
        let input = "© 2024\n™ Brand\n…continued\n°F";
        let expected = "(c) 2024\n(TM) Brand\n...continued\n degreesF";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_math_symbols() {
        let input = "x ≥ 5\ny ≤ 10\na ≠ b\nx ÷ y";
        let expected = "x >= 5\ny <= 10\na != b\nx / y";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_smart_quotes() {
        // Using Unicode escape sequences for smart quotes
        let input = "\u{201C}Hello\u{201D} \u{2018}world\u{2019}"; // "Hello" 'world'
        let expected = "\"Hello\" 'world'";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_dashes() {
        let input = "em—dash\nen–dash";
        let expected = "em--dash\nen-dash";
        assert_eq!(fix_tree_symbols(input), expected);
    }

    #[test]
    fn test_fix_combined_unicode() {
        let input = "✓ naïve → café\n├── résumé.md\n© 2024";
        let expected = "[x] naive -> cafe\n+-- resume.md\n(c) 2024";
        assert_eq!(fix_tree_symbols(input), expected);
    }
}
