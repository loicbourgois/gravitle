use std::io::BufRead;
use std::sync::Mutex;
use std::{
    collections::VecDeque,
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

use chrono::Utc;
use fs2::FileExt;

const LOG_LIMIT: usize = 100;

pub(crate) fn record(message: &str) {
    let Some(home) = env::var_os("HOME") else {
        return;
    };
    let home = Path::new(&home);
    let directory = home.join("github.com/loicbourgois/gravitle/cli/logs");

    // Logging is best-effort and must not change command behavior.
    let _ = write_record(&directory, home, message);
}

fn normalize_paths(message: &str, home: &Path) -> String {
    let home = home.to_string_lossy();
    if home.is_empty() {
        return message.to_owned();
    }

    let mut output = String::new();
    let mut remaining = message;

    while let Some(index) = remaining.find(home.as_ref()) {
        output.push_str(&remaining[..index]);
        remaining = &remaining[index + home.len()..];

        let boundary = remaining.chars().next().is_none_or(|character| {
            character == '/'
                || character.is_whitespace()
                || matches!(character, '\'' | '"' | ')' | ']' | '}' | ':' | ',' | ';')
        });

        output.push_str(if boundary { "~" } else { home.as_ref() });
    }

    output.push_str(remaining);
    output
}

fn write_record(directory: &Path, home: &Path, message: &str) -> io::Result<()> {
    fs::create_dir_all(directory)?;

    // Keep the lock separate from the file rewritten below.
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(directory.join("cli.lock"))?;
    lock.lock_exclusive()?;

    let path = directory.join("cli.log");
    let existing = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(error),
    };
    let mut lines: VecDeque<String> = existing
        .lines()
        .map(|line| normalize_paths(line, home))
        .collect();

    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    for line in normalize_paths(message, home).lines() {
        lines.push_back(format!("{timestamp} {line}"));
    }
    while lines.len() > LOG_LIMIT {
        lines.pop_front();
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    for line in lines {
        writeln!(file, "{line}")?;
    }
    file.flush()
}

static TEST_OUTPUT_LOCK: Mutex<()> = Mutex::new(());

pub(crate) fn forward_test_output(reader: impl BufRead) -> io::Result<()> {
    for line in reader.split(b'\n') {
        let line = line?;
        let message = String::from_utf8_lossy(&line);
        let _guard = TEST_OUTPUT_LOCK
            .lock()
            .map_err(|_| io::Error::other("test output lock poisoned"))?;

        let home = env::var_os("HOME").ok_or_else(|| io::Error::other("HOME is not set"))?;
        let home = Path::new(&home);
        let directory = home.join("github.com/loicbourgois/gravitle/cli/logs");
        let mut terminal = io::stdout().lock();

        write_test_output(&directory, home, &message, &mut terminal)?;
    }

    Ok(())
}

fn write_test_output(
    directory: &Path,
    home: &Path,
    message: &str,
    terminal: &mut impl Write,
) -> io::Result<()> {
    fs::create_dir_all(directory)?;

    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(directory.join("terminal.lock"))?;
    lock.lock_exclusive()?;

    let path = directory.join("terminal.log");
    let existing = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(error),
    };
    let mut lines: VecDeque<String> = existing.lines().map(str::to_owned).collect();

    // Preserve blank output lines as timestamped records too.
    let message = message.strip_suffix('\r').unwrap_or(message);
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let output = format!("{timestamp} {}\n", normalize_paths(message, home));

    lines.push_back(output.trim_end_matches('\n').to_owned());
    while lines.len() > LOG_LIMIT {
        lines.pop_front();
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    for line in lines {
        writeln!(file, "{line}")?;
    }
    file.flush()?;

    terminal.write_all(output.as_bytes())?;
    terminal.flush()
}

#[cfg(test)]
#[path = "logging_test.rs"]
mod tests;
