use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

#[test]
fn logs_keep_latest_100_lines_with_utc_timestamps() {
    let directory = env::temp_dir().join(format!(
        "gravitle-cli-log-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    let home = Path::new("/Users/test");

    for index in 0..105 {
        write_record(&directory, home, &format!("message {index}")).unwrap();
    }

    let content = fs::read_to_string(directory.join("cli.log")).unwrap();
    let lines: Vec<_> = content.lines().collect();
    assert_eq!(lines.len(), LOG_LIMIT);

    for (index, line) in lines.iter().enumerate() {
        let (timestamp, message) = line.split_once(' ').unwrap();
        chrono::DateTime::parse_from_rfc3339(timestamp).unwrap();
        assert!(timestamp.ends_with('Z'));
        assert_eq!(message, format!("message {}", index + 5));
    }

    write_record(&directory, home, "first\nsecond\n").unwrap();
    let content = fs::read_to_string(directory.join("cli.log")).unwrap();
    let lines: Vec<_> = content.lines().collect();
    assert_eq!(lines.len(), LOG_LIMIT);
    assert!(lines[98].ends_with("Z first"));
    assert!(lines[99].ends_with("Z second"));

    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn paths_use_tilde_without_rewriting_similar_prefixes() {
    assert_eq!(
        normalize_paths(
            "/Users/test/file; /Users/test; /Users/test-other/file",
            Path::new("/Users/test"),
        ),
        "~/file; ~; /Users/test-other/file",
    );
}