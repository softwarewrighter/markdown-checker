//! Integration tests for markdown-checker.

use markdown_checker::file_ops::read_file_content;
use markdown_checker::validators::validate_all;
use std::path::PathBuf;

/// Test that the project's own README.md passes all validation checks
/// This ensures we practice what we preach!
#[test]
fn test_project_readme_passes_validation() {
    let readme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("README.md");

    // Read the README content
    let content = read_file_content(&readme_path).expect("Failed to read project README.md");

    // Run all validators
    let results = validate_all(&content);

    // Check that all validators pass
    for result in &results {
        if result.is_fail() {
            panic!(
                "Project README.md failed {} validator with {} error(s):\n{:?}",
                result.validator_name,
                result.errors.len(),
                result.errors
            );
        }
    }

    // Verify all results are passing
    assert!(
        results.iter().all(|r| r.is_pass()),
        "Project README.md should pass all validation checks"
    );
}

/// Test that valid.md fixture passes validation
#[test]
fn test_valid_fixture_passes() {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid.md");

    let content = read_file_content(&fixture_path).expect("Failed to read valid.md fixture");

    let results = validate_all(&content);

    assert!(
        results.iter().all(|r| r.is_pass()),
        "valid.md fixture should pass all checks"
    );
}

/// Test that tree_chars.md fixture fails validation
#[test]
fn test_tree_chars_fixture_fails() {
    let fixture_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/tree_chars.md");

    let content = read_file_content(&fixture_path).expect("Failed to read tree_chars.md fixture");

    let results = validate_all(&content);

    // Should have failures
    assert!(
        results.iter().any(|r| r.is_fail()),
        "tree_chars.md fixture should fail validation"
    );

    // Specifically, Tree Symbols validator should fail
    let tree_result = results
        .iter()
        .find(|r| r.validator_name == "Tree Symbols")
        .expect("Tree Symbols validator should run");

    assert!(
        tree_result.is_fail(),
        "Tree Symbols validator should fail for tree_chars.md"
    );
}

/// Test that non_ascii.md fixture fails validation
#[test]
fn test_non_ascii_fixture_fails() {
    let fixture_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/non_ascii.md");

    let content = read_file_content(&fixture_path).expect("Failed to read non_ascii.md fixture");

    let results = validate_all(&content);

    // Should have failures
    assert!(
        results.iter().any(|r| r.is_fail()),
        "non_ascii.md fixture should fail validation"
    );

    // ASCII Subset validator should fail
    let ascii_result = results
        .iter()
        .find(|r| r.validator_name == "ASCII Subset")
        .expect("ASCII Subset validator should run");

    assert!(
        ascii_result.is_fail(),
        "ASCII Subset validator should fail for non_ascii.md"
    );
}
