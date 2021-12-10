#![deny(warnings)]
extern crate clap;
use clap::{App, SubCommand};
use std::process::Command;
extern crate dirs;
extern crate notify;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
fn main() {
    let matches = App::new("gravitle")
        .usage("gravitle [COMMAND]")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("build"))
        .subcommand(SubCommand::with_name("format"))
        .subcommand(SubCommand::with_name("start"))
        .subcommand(SubCommand::with_name("watch"))
        .get_matches();
    if let Some(_matches) = matches.subcommand_matches("build") {
        build();
    } else if let Some(_matches) = matches.subcommand_matches("format") {
        format();
    } else if let Some(_matches) = matches.subcommand_matches("start") {
        start();
    } else if let Some(_matches) = matches.subcommand_matches("watch") {
        watch();
    }
}
fn runshellcmd(title: &str, command: &mut Command) -> bool {
    println!("[start] {}", title);
    if let Ok(mut child) = command.spawn() {
        if child.wait().expect("error").code().unwrap() == 0 {
            println!("[ end ] {} done", title);
            return true;
        } else {
            println!("[error] {} failed", title);
        }
    } else {
        println!("[error] {} didn't start", title);
    }
    return false;
}
fn build() -> bool {
    return runshellcmd(
        "Building",
        Command::new("wasm-pack")
            .arg("build")
            .env("RUSTFLAGS", "--cfg=web_sys_unstable_apis")
            .current_dir(format!("{}/wasm/", base_dir())),
    ) && runshellcmd(
        "Fixing",
        Command::new("npm")
            .arg("audit")
            .arg("fix")
            .current_dir(format!("{}/front/", base_dir())),
    ) && runshellcmd(
        "Installing",
        Command::new("npm")
            .arg("install")
            .current_dir(format!("{}/front/", base_dir())),
    );
}
fn format() {
    runshellcmd(
        "Formating",
        Command::new("cargo")
            .arg("fmt")
            .arg("--manifest-path")
            .arg(format!("{}/cli/Cargo.toml", base_dir())),
    );
    runshellcmd(
        "Formating",
        Command::new("cargo")
            .arg("fmt")
            .arg("--manifest-path")
            .arg(format!("{}/wasm/Cargo.toml", base_dir())),
    );
}
fn start() {
    let _ = build() && runshellcmd(
        "Starting",
        Command::new("npm")
            .arg("run")
            .arg("start")
            .current_dir(format!("{}/front", base_dir())),
    );
}
fn watch() {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher
        .watch(format!("{}/wasm/src", base_dir()), RecursiveMode::Recursive)
        .unwrap();
    watcher
        .watch(
            format!("{}/wasm/Cargo.toml", base_dir()),
            RecursiveMode::Recursive,
        )
        .unwrap();
    build();
    loop {
        match rx.recv() {
            Ok(_) => {
                build();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
fn base_dir() -> String {
    return format!(
        "{}/github.com/loicbourgois/gravitle",
        dirs::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    );
}
