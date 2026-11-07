use super::*;
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