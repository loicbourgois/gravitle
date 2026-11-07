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

#[test]
fn test_output_matches_terminal_and_retains_latest_100_lines() {
    let directory = env::temp_dir().join(format!(
        "gravitle-cli-terminal-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    let home = Path::new("/Users/test");
    let mut terminal = Vec::new();

    for index in 0..105 {
        write_test_output(
            &directory,
            home,
            &format!("/Users/test/project test {index}"),
            &mut terminal,
        )
        .unwrap();
    }

    let terminal = String::from_utf8(terminal).unwrap();
    let terminal_lines: Vec<_> = terminal.lines().collect();
    let content = fs::read_to_string(directory.join("terminal.log")).unwrap();
    let file_lines: Vec<_> = content.lines().collect();

    assert_eq!(file_lines.len(), LOG_LIMIT);
    assert_eq!(file_lines.as_slice(), &terminal_lines[5..]);

    for (index, line) in file_lines.iter().enumerate() {
        let (timestamp, message) = line.split_once(' ').unwrap();
        chrono::DateTime::parse_from_rfc3339(timestamp).unwrap();
        assert!(timestamp.ends_with('Z'));
        assert_eq!(message, format!("~/project test {}", index + 5));
    }

    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn test_output_preserves_blank_lines_and_normalizes_crlf() {
    let directory = env::temp_dir().join(format!(
        "gravitle-cli-terminal-lines-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    let mut terminal = Vec::new();

    for message in ["first\r", "", "last"] {
        write_test_output(&directory, Path::new("/Users/test"), message, &mut terminal).unwrap();
    }

    let content = fs::read_to_string(directory.join("terminal.log")).unwrap();
    assert_eq!(content.as_bytes(), terminal.as_slice());

    let messages: Vec<_> = content
        .lines()
        .map(|line| line.split_once(' ').unwrap().1)
        .collect();
    assert_eq!(messages, ["first", "", "last"]);

    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn retained_terminal_log_paths_are_normalized() {
    let directory = env::temp_dir().join(format!(
        "gravitle-cli-terminal-retained-test-{}-{}",
        std::process::id(),
        NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&directory).unwrap();
    fs::write(
        directory.join("terminal.log"),
        concat!(
            "2026-11-07T13:39:19Z Installed /Users/test/.local/bin/g\n",
            "2026-11-07T13:39:20Z \n",
            "2026-11-07T13:39:21Z Unrelated /Users/test-other/file\n",
        ),
    )
    .unwrap();

    let mut terminal = Vec::new();
    write_test_output(
        &directory,
        Path::new("/Users/test"),
        "Watching /Users/test/project",
        &mut terminal,
    )
    .unwrap();

    let content = fs::read_to_string(directory.join("terminal.log")).unwrap();
    let lines: Vec<_> = content.lines().collect();

    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0], "2026-11-07T13:39:19Z Installed ~/.local/bin/g");
    assert_eq!(lines[1], "2026-11-07T13:39:20Z ");
    assert_eq!(
        lines[2],
        "2026-11-07T13:39:21Z Unrelated /Users/test-other/file"
    );
    assert!(lines[3].ends_with("Z Watching ~/project"));

    // Retained history is not replayed; newly emitted output matches the file.
    assert_eq!(
        String::from_utf8(terminal).unwrap(),
        format!("{}\n", lines[3]),
    );

    fs::remove_dir_all(directory).unwrap();
}
