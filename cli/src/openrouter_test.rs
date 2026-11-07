use super::*;
use crate::todo::load_todo_files;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FILE: AtomicU64 = AtomicU64::new(0);

#[test]
fn api_key_is_raw_text_and_missing_or_empty_files_fail() {
    let path = std::env::temp_dir().join(format!(
        "gravitle-key-test-{}-{}",
        std::process::id(),
        NEXT_FILE.fetch_add(1, Ordering::Relaxed),
    ));

    assert!(read_api_key(&path).is_err());

    std::fs::write(&path, "  test-secret\n").unwrap();
    assert_eq!(read_api_key(&path).unwrap(), "test-secret");

    std::fs::write(&path, " \n").unwrap();
    assert!(read_api_key(&path).is_err());

    std::fs::remove_file(path).unwrap();
}

#[test]
fn response_content_preserves_markdown_exactly() {
    let content = "\n# Updates\n\n```rust\nfn example() {}\n```\n";
    let body = serde_json::json!({
        "choices": [{
            "message": {
                "content": content
            }
        }]
    })
    .to_string();

    assert_eq!(parse_response_content(&body).unwrap(), content);
}

#[test]
fn malformed_response_json_is_rejected() {
    assert!(parse_response_content("not json").is_err());
}

#[test]
fn missing_or_non_string_response_content_is_rejected() {
    for body in [
        "{}",
        r#"{"choices":[]}"#,
        r#"{"choices":[{}]}"#,
        r#"{"choices":[{"message":{}}]}"#,
        r#"{"choices":[{"message":{"content":null}}]}"#,
        r#"{"choices":[{"message":{"content":42}}]}"#,
    ] {
        let error = parse_response_content(body).unwrap_err().to_string();

        assert!(
            error.contains("missing message content"),
            "unexpected error for {body}: {error}",
        );
    }
}

#[test]
fn empty_response_content_is_rejected() {
    for content in ["", " ", "\n\t\r\n"] {
        let body = serde_json::json!({
            "choices": [{
                "message": {
                    "content": content
                }
            }]
        })
        .to_string();

        let error = parse_response_content(&body).unwrap_err().to_string();

        assert!(error.contains("empty message content"));
    }
}

#[test]
fn readable_inputs_do_not_hide_missing_files() {
    // A source file included at compile time is guaranteed to be readable.
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/todo.rs");
    let first_missing = source.join("missing-first.rs");
    let second_missing = source.join("missing-second.rs");

    let files = vec![
        source.to_string_lossy().into_owned(),
        first_missing.to_string_lossy().into_owned(),
        second_missing.to_string_lossy().into_owned(),
    ];

    let error = load_todo_files(Path::new(env!("CARGO_MANIFEST_DIR")), &files)
        .unwrap_err()
        .to_string();

    assert_eq!(error.lines().count(), 2);
    assert!(error.contains("missing-first.rs"));
    assert!(error.contains("missing-second.rs"));
}
