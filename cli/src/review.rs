use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const REVIEW_TEMPLATE: &str = include_str!("review_prompt.md");

pub(crate) fn review(specification: &str) -> Result<(), Box<dyn Error>> {
    let path = resolve_specification(specification)?;
    crate::todo::run_specification(&path.to_string_lossy(), REVIEW_TEMPLATE)
}

fn resolve_specification(specification: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = Path::new(specification);
    if path.is_absolute()
        || specification.contains('/')
        || matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("yml" | "yaml")
        )
    {
        return Ok(path.to_path_buf());
    }
    let home = std::env::var_os("HOME").ok_or("HOME is not set")?;
    let directory = PathBuf::from(home)
        .join("github.com/loicbourgois/gravitle/front/moonlighter/review");
    resolve_prefix(&directory, specification)
}

fn resolve_prefix(directory: &Path, prefix: &str) -> Result<PathBuf, Box<dyn Error>> {
    if prefix.is_empty() {
        return Err("Review specification prefix must not be empty".into());
    }
    let mut matches = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_file()
            && matches!(
                path.extension().and_then(|extension| extension.to_str()),
                Some("yml" | "yaml")
            )
            && entry.file_name().to_string_lossy().starts_with(prefix)
        {
            matches.push(path);
        }
    }
    matches.sort();
    match matches.as_slice() {
        [path] => Ok(path.clone()),
        [] => Err(format!("No Moonlighter review specification matches {prefix:?}").into()),
        _ => Err(format!(
            "Ambiguous Moonlighter review specification {prefix:?}: {}",
            matches
                .iter()
                .filter_map(|path| path.file_name())
                .map(|name| name.to_string_lossy())
                .collect::<Vec<_>>()
                .join(", ")
        )
        .into()),
    }
}

#[cfg(test)]
#[path = "review_test.rs"]
mod tests;