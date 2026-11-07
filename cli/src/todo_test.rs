use super::*;

#[test]
fn models_must_be_nonempty_and_have_distinct_output_names() {
    for models in [
        vec![],
        vec![" ".to_owned()],
        vec!["provider-a/model".to_owned(), "provider-b/model".to_owned()],
    ] {
        let spec = TodoSpec {
            models,
            files: vec![],
            rules: vec![],
        };
        assert!(validate_models(&spec).is_err());
    }
}

#[test]
fn relative_file_urls_resolve_beside_the_specification() {
    assert_eq!(
        resolve_file_path(Path::new("/tmp/project"), "file://src/main.rs").unwrap(),
        Path::new("/tmp/project/src/main.rs"),
    );
}

#[test]
fn all_missing_files_are_reported() {
    let error = load_todo_files(
        Path::new("/dev/null"),
        &["first.rs".to_owned(), "second.rs".to_owned()],
    )
    .unwrap_err()
    .to_string();

    assert!(error.contains("first.rs"));
    assert!(error.contains("second.rs"));
}

#[test]
fn inserted_placeholders_remain_literal() {
    let prompt = build_prompt(
        "{files}\n{rules}",
        &["Keep {files} literal".to_owned()],
        &[("example.md".to_owned(), "{rules}".to_owned())],
    );

    assert!(prompt.contains("\n{rules}\n"));
    assert!(prompt.ends_with("Keep {files} literal"));
}

#[test]
fn fences_are_longer_than_embedded_backticks() {
    assert_eq!(code_fence("```rust\nexample\n```"), "````");
}