/// Auto-fix module for correcting markdown violations
///
/// Currently supports:
/// - Tree symbols: Replaces Unicode box-drawing characters with ASCII equivalents

/// Fix tree symbols in the content by replacing them with ASCII equivalents
pub fn fix_tree_symbols(content: &str) -> String {
    let mut fixed = String::with_capacity(content.len());

    for ch in content.chars() {
        let replacement = match ch {
            // Box-drawing characters to ASCII
            '├' | '┤' | '┼' | '┬' | '┴' | '╋' => '+',
            '└' | '┘' | '┌' | '┐' | '╰' | '╯' | '╭' | '╮' => '+',
            '│' | '┃' | '║' => '|',
            '─' | '━' | '═' => '-',
            _ if is_box_drawing(ch) => '+', // fallback for other box-drawing chars
            _ => ch,
        };
        fixed.push(replacement);
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
}
