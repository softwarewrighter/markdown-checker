# Example Tool Output

This file shows example output from the markdown-checker tool.
It contains Unicode characters (checkmarks, X marks) that are intentionally invalid.

## Example 1: Successful Validation

```
$ markdown-checker
✓ File validation successful: ./README.md
```

## Example 2: Detecting Tree Symbols

Given a file with tree characters:

```markdown
Project structure:
├── src/
│   └── main.rs
└── tests/
```

Running the checker:

```
$ markdown-checker
✗ File validation failed: ./README.md

ASCII Subset: ✗ Fail (3 errors)
  Line 2, Column 1: Non-ASCII character: '├' (U+251C)
  Line 3, Column 1: Non-ASCII character: '│' (U+2502)
  Line 3, Column 5: Non-ASCII character: '└' (U+2514)

Tree Symbols: ✗ Fail (3 errors)
  Line 2, Column 1: Tree symbol '├' (U+251C) detected. Use '+' or '|' instead
  Line 3, Column 1: Tree symbol '│' (U+2502) detected. Use '|' instead
  Line 3, Column 5: Tree symbol '└' (U+2514) detected. Use '+' or '`' instead
```

## Example 3: Verbose Mode

```
$ markdown-checker -v
Checking file: ./README.md
File size: 1,234 bytes

Running validators...
[1/3] ASCII Subset... ✓ Pass
[2/3] Printable Characters... ✓ Pass
[3/3] Tree Symbols... ✓ Pass

✓ File validation successful: ./README.md
```
