use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

struct ReviewDirectory(PathBuf);

impl ReviewDirectory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "gravitle-review-test-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for ReviewDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn unique_prefix_resolves_yaml_and_ignores_generated_outputs() {
    let directory = ReviewDirectory::new();
    let specification = directory.0.join("01_performance.yml");
    fs::write(&specification, "models: []\n").unwrap();
    fs::write(directory.0.join("01_performance.yml.t.p.md"), "prompt").unwrap();
    fs::create_dir(directory.0.join("01_directory.yml")).unwrap();
    assert_eq!(
        resolve_prefix(&directory.0, "01").unwrap(),
        specification,
    );
}

#[test]
fn missing_and_ambiguous_prefixes_fail() {
    let directory = ReviewDirectory::new();
    assert!(resolve_prefix(&directory.0, "01").is_err());
    assert!(resolve_prefix(&directory.0, "").is_err());
    fs::write(directory.0.join("01_cpu.yml"), "").unwrap();
    fs::write(directory.0.join("01_render.yaml"), "").unwrap();
    let error = resolve_prefix(&directory.0, "01").unwrap_err().to_string();
    assert!(error.contains("Ambiguous"));
    assert!(error.contains("01_cpu.yml"));
    assert!(error.contains("01_render.yaml"));
}

#[test]
fn explicit_specification_paths_are_preserved() {
    for specification in [
        "/tmp/reviews/performance.yml",
        "reviews/performance.yml",
        "performance.yaml",
        "~/reviews/performance.yml",
    ] {
        assert_eq!(
            resolve_specification(specification).unwrap(),
            PathBuf::from(specification),
        );
    }
}