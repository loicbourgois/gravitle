use crate::base_dir;
use crate::runshellcmd;
use std::process::Command;

pub fn release_front() -> bool {
    runshellcmd(
        "Building front (production)",
        Command::new("npm")
            .arg("run")
            .arg("build")
            .current_dir(format!("{}/front/", base_dir())),
    )
}

pub fn release_server() {}
