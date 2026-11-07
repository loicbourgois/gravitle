use crate::openrouter::{call_openrouter, openrouter_api_key};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

const TODO_TEMPLATE: &str = include_str!("todo_prompt.md");

fn validate_models(spec: &TodoSpec) -> Result<(), Box<dyn std::error::Error>> {
    if spec.models.is_empty() {
        return Err("Todo specification must contain at least one model".into());
    }

    let mut names = std::collections::HashSet::new();
    for model in &spec.models {
        let name = short_model_name(model);
        if model.trim().is_empty() || name.is_empty() {
            return Err("Todo specification contains an invalid model name".into());
        }
        if !names.insert(name) {
            return Err(format!("Models produce colliding response filenames: {model}").into());
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct TodoSpec {
    models: Vec<String>,
    files: Vec<String>,
    rules: Vec<String>,
}

fn display_path(path: &Path) -> String {
    if let Ok(home) = env::var("HOME") {
        let home_path = Path::new(&home);

        if path == home_path {
            return "~".to_string();
        }

        if let Ok(rest) = path.strip_prefix(home_path) {
            return format!("~/{}", rest.display());
        }
    }

    path.display().to_string()
}

fn prompt_output_path(spec_path: &Path) -> PathBuf {
    let file_name = spec_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("todo.yml");
    // todo.prompt
    spec_path.with_file_name(format!("{}.t.p.md", file_name))
}

fn response_output_path(spec_path: &Path, model: &str) -> PathBuf {
    let file_name = spec_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("todo.yml");

    spec_path.with_file_name(format!(
        // todo.response
        "{}.t.r.{}.md",
        file_name,
        short_model_name(model)
    ))
}

fn short_model_name(model: &str) -> String {
    let model_name = model.rsplit('/').next().unwrap_or(model);

    model_name
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || *character == '.')
        .collect()
}


pub fn todo(spec_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_specification(spec_file, TODO_TEMPLATE)
}

pub(crate) fn run_specification(
    spec_file: &str,
    template: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec_path = expand_home(spec_file);
    let spec_dir = spec_path
        .parent()
        .ok_or_else(|| format!("Could not determine parent directory for {spec_file}"))?;
    let spec_contents = fs::read_to_string(&spec_path).map_err(|error| {
        format!(
            "Could not open specification {}: {error}",
            display_path(&spec_path)
        )
    })?;
    let spec: TodoSpec = serde_yaml::from_str(&spec_contents)?;
    validate_models(&spec)?;
    let files = load_todo_files(spec_dir, &spec.files)?;
    let prompt = build_prompt(template, &spec.rules, &files);
    let prompt_path = prompt_output_path(&spec_path);
    fs::write(&prompt_path, &prompt)?;
    crate::logging::command_message(&format!("Prompt: {}", display_path(&prompt_path)))?;
    let api_key = openrouter_api_key()?;
    let total = spec.models.len();
    let mut completed = 0usize;
    let (tx, rx) = mpsc::channel();
    let api_key_arc = Arc::new(api_key);
    let prompt_arc = Arc::new(prompt);
    for (idx, model) in spec.models.iter().enumerate() {
        let tx = tx.clone();
        let api_key = api_key_arc.clone();
        let prompt = prompt_arc.clone();
        let model = model.clone();
        thread::spawn(move || {
            let started = std::time::Instant::now();
            let result = call_openrouter(api_key.as_str(), &model, prompt.as_str())
                .map_err(|error| error.to_string());
            let elapsed = started.elapsed();
            let _ = tx.send((idx, model, result, elapsed));
        });
    }
    drop(tx); // Close the channel when all senders are done
    let mut failures = Vec::new();
    let mut saved_responses = 0usize;
    crate::logging::command_message("Responses:")?;
    for (_idx, model, result, elapsed) in rx.iter() {
        completed += 1;
        match result {
            Ok(response) => {
                let response_path = response_output_path(&spec_path, &model);

                if let Err(error) = fs::write(&response_path, response) {
                    let error =
                        format!("Could not write {}: {error}", display_path(&response_path),);
                    let log = format!(
                        "  {}:\n    error:    {}\n    progress: {}/{}",
                        model, error, completed, total,
                    );
                    crate::logging::command_message(&log)?;
                    failures.push(format!("{model}: {error}"));
                } else {
                    saved_responses += 1;

                    let log = format!(
                        "  {}:\n    progress: {}/{}\n    duration: {:?}\n    file:     {}",
                        model,
                        completed,
                        total,
                        elapsed,
                        display_path(&response_path),
                    );
                    crate::logging::command_message(&log)?;
                }
            }
            Err(error) => {
                let log = format!(
                    "  {}:\n    error:    {}\n    progress: {}/{}",
                    model, error, completed, total
                );
                crate::logging::command_message(&log)?;
                failures.push(format!("{}: {}", model, error));
            }
        }
    }

    finish_todo(saved_responses, &failures)
}

fn finish_todo(
    saved_responses: usize,
    failures: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if saved_responses > 0 {
        return Ok(());
    }

    if failures.is_empty() {
        return Err("No model todo responses were saved".into());
    }

    Err(format!(
        "No model todo responses were saved; {} model todo(s) failed:\n{}",
        failures.len(),
        failures.join("\n"),
    )
    .into())
}

pub fn load_todo_files(
    spec_dir: &Path,
    files: &[String],
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut loaded = Vec::new();
    let mut errors = Vec::new();

    for file in files {
        let path = resolve_file_path(spec_dir, file)?;
        match fs::read_to_string(&path) {
            Ok(contents) => loaded.push((file.clone(), contents)),
            Err(error) => errors.push(format!("Could not open {}: {error}", display_path(&path))),
        }
    }

    if errors.is_empty() {
        Ok(loaded)
    } else {
        Err(errors.join("\n").into())
    }
}

fn resolve_file_path(spec_dir: &Path, file: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let raw_path = file.strip_prefix("file://").unwrap_or(file);
    let path = expand_home(raw_path);

    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(spec_dir.join(path))
    }
}

fn code_language(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("yml" | "yaml") => "yaml",
        Some("json") => "json",
        Some("toml") => "toml",
        Some("sh" | "bash") => "bash",
        Some("md" | "markdown") => "markdown",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("html") => "html",
        Some("css") => "css",
        _ => "text",
    }
}

fn code_fence(contents: &str) -> String {
    let longest_run = contents
        .lines()
        .map(|line| {
            line.chars()
                .fold((0, 0), |(longest, current), character| {
                    if character == '`' {
                        (longest.max(current + 1), current + 1)
                    } else {
                        (longest, 0)
                    }
                })
                .0
        })
        .max()
        .unwrap_or(0);

    "`".repeat((longest_run + 1).max(3))
}

fn build_prompt(template: &str, rules: &[String], files: &[(String, String)]) -> String {
    let rules_text = rules
        .iter()
        .map(|rule| format!("- {}", rule.trim()))
        .collect::<Vec<_>>()
        .join("\n");

    let files_text = files
        .iter()
        .map(|(path, contents)| {
            let fence = code_fence(contents);
            let language = code_language(path);

            format!(
                "## {}\n\n{}{}\n{}{}{}\n",
                path,
                fence,
                language,
                contents,
                if contents.ends_with('\n') { "" } else { "\n" },
                fence,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut prompt = String::new();
    let mut remaining = template;

    while let Some(index) = remaining.find('{') {
        prompt.push_str(&remaining[..index]);
        remaining = &remaining[index..];

        if let Some(rest) = remaining.strip_prefix("{rules}") {
            prompt.push_str(&rules_text);
            remaining = rest;
        } else if let Some(rest) = remaining.strip_prefix("{files}") {
            prompt.push_str(&files_text);
            remaining = rest;
        } else {
            prompt.push('{');
            remaining = &remaining[1..];
        }
    }

    prompt.push_str(remaining);
    prompt
}

fn expand_home(path: &str) -> PathBuf {
    if path == "~"
        && let Ok(home) = env::var("HOME")
    {
        return PathBuf::from(home);
    }
    if let Some(rest) = path.strip_prefix("~/")
        && let Ok(home) = env::var("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    PathBuf::from(path)
}

#[cfg(test)]
#[path = "todo_test.rs"]
mod tests;
