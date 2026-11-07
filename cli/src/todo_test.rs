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

#[test]
fn one_saved_response_is_enough_for_success() {
    let failures = vec![
        "provider/failed-model: request failed".to_owned(),
        "provider/unsaved-model: could not write response".to_owned(),
    ];

    assert!(finish_todo(1, &failures).is_ok());
}

#[test]
fn all_saved_responses_succeed() {
    assert!(finish_todo(2, &[]).is_ok());
}

#[test]
fn no_saved_responses_reports_all_failures() {
    let failures = vec![
        "provider/first: request failed".to_owned(),
        "provider/second: could not write response".to_owned(),
    ];

    let error = finish_todo(0, &failures).unwrap_err().to_string();

    assert!(error.contains("2 model todo(s) failed"));
    for failure in failures {
        assert!(error.contains(&failure));
    }
}

#[test]
fn fn_no_results_cannot_be_reported_as_success() {
    assert!(finish_todo(0, &[]).is_err());
}

struct InputDirectory(PathBuf);

impl InputDirectory {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};

        static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

        let path = env::temp_dir().join(format!(
            "gravitle-todo-input-test-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for InputDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn readable_inputs_do_not_hide_missing_files() {
    let directory = InputDirectory::new();
    fs::write(directory.0.join("readable.rs"), "fn example() {}\n").unwrap();

    let files = vec![
        "readable.rs".to_owned(),
        "missing-first.rs".to_owned(),
        "missing-second.rs".to_owned(),
    ];

    let error = load_todo_files(&directory.0, &files)
        .unwrap_err()
        .to_string();

    assert_eq!(error.lines().count(), 2);
    assert!(error.contains("missing-first.rs"));
    assert!(error.contains("missing-second.rs"));
    assert!(!error.contains("readable.rs"));
}

#[test]
fn readable_inputs_preserve_references_and_contents() {
    let directory = InputDirectory::new();
    let contents = "\nfn example() {}\n\n";
    fs::write(directory.0.join("readable.rs"), contents).unwrap();

    let reference = "file://readable.rs".to_owned();
    let loaded = load_todo_files(&directory.0, &[reference.clone()]).unwrap();

    assert_eq!(loaded, vec![(reference, contents.to_owned())]);
}
