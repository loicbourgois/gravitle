#![deny(warnings)]
extern crate clap;
extern crate dirs;
extern crate notify;
use clap::{App, SubCommand};
use notify::{watcher, RecursiveMode, Watcher};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
fn main() {
    let matches = App::new("gravitle")
        .usage("gravitle [COMMAND]")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("build"))
        .subcommand(SubCommand::with_name("lint"))
        .subcommand(SubCommand::with_name("format"))
        .subcommand(
            SubCommand::with_name("start")
                .setting(clap::AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("front"))
                .subcommand(SubCommand::with_name("server"))
                .subcommand(SubCommand::with_name("server_2"))
                .subcommand(SubCommand::with_name("server_async"))
                .subcommand(SubCommand::with_name("client")),
        )
        .subcommand(
            SubCommand::with_name("test")
                .setting(clap::AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("server")),
        )
        .subcommand(SubCommand::with_name("watch"))
        .subcommand(SubCommand::with_name("poc"))
        .get_matches();
    if let Some(_matches) = matches.subcommand_matches("build") {
        build();
    } else if matches.subcommand_matches("lint").is_some() {
        lint();
    } else if let Some(_matches) = matches.subcommand_matches("format") {
        format();
    } else if let Some(_matches) = matches.subcommand_matches("poc") {
        poc();
    } else if let Some(matches_2) = matches.subcommand_matches("start") {
        if let Some(_matches) = matches_2.subcommand_matches("front") {
            start_front();
        } else if let Some(_matches) = matches_2.subcommand_matches("server") {
            start_server();
        } else if let Some(_matches) = matches_2.subcommand_matches("server_2") {
            start_server_2();
        } else if let Some(_matches) = matches_2.subcommand_matches("server_async") {
            start_server_async();
        } else if let Some(_matches) = matches_2.subcommand_matches("client") {
            start_client();
        }
    } else if let Some(matches_2) = matches.subcommand_matches("test") {
        if let Some(_matches) = matches_2.subcommand_matches("server") {
            test_server();
        }
    } else if let Some(_matches) = matches.subcommand_matches("watch") {
        watch();
    }
}
fn poc() {
    runshellcmd(
        "Poc",
        Command::new("python").arg(format!("{}/poc.py", base_dir())),
    );
}
fn runshellcmd(title: &str, command: &mut Command) -> bool {
    println!("[ start ] {}", title);
    if let Ok(mut child) = command.spawn() {
        match child.wait().expect("error").code() {
            Some(code) => {
                if code == 0 {
                    println!("[  end  ] {} done", title);
                    return true;
                } else {
                    println!("[ error ] {} failed", title);
                }
            }
            None => {
                println!("[ error ] {} failed", title);
            }
        }
    } else {
        println!("[ error ] {} didn't start", title);
    }
    false
}
fn succes() -> bool {
    println!("[success]");
    true
}
fn build() -> bool {
    return runshellcmd(
        "Building cli",
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(format!("{}/cli/", base_dir())),
    ) && runshellcmd(
        "Building server",
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(format!("{}/server/", base_dir())),
    ) && test_server()
        && runshellcmd(
            "Building wasm",
            Command::new("wasm-pack")
                .arg("build")
                .arg("--release")
                .env("RUSTFLAGS", "--cfg=web_sys_unstable_apis")
                .current_dir(format!("{}/wasm/", base_dir())),
        )
        && runshellcmd(
            "Fixing",
            Command::new("npm")
                .arg("audit")
                .arg("fix")
                .current_dir(format!("{}/front/", base_dir())),
        )
        && runshellcmd(
            "Installing",
            Command::new("npm")
                .arg("install")
                .current_dir(format!("{}/front/", base_dir())),
        )
        && succes();
}

fn lint() -> bool {
    runshellcmd(
        "Linting cli",
        Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("--deny")
            .arg("warnings")
            .current_dir(format!("{}/cli/", base_dir())),
    )
    && runshellcmd(
        "Linting server",
        Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("--deny")
            .arg("warnings")
            .current_dir(format!("{}/server/", base_dir())),
    ) && succes()
}

fn format() {
    for project in ["cli", "client", "server", "wasm"] {
        runshellcmd(
            &format!("Formating {}", project),
            Command::new("cargo")
                .arg("fmt")
                .arg("--manifest-path")
                .arg(format!("{}/{}/Cargo.toml", base_dir(), project)),
        );
    }
}
fn start_front() {
    let _ = build()
        && runshellcmd(
            "Starting client",
            Command::new("npm")
                .arg("run")
                .arg("start")
                .current_dir(format!("{}/front", base_dir())),
        );
}
fn start_server() -> bool {
    return runshellcmd(
        "Starting server",
        Command::new("cargo")
            .arg("run")
            .arg("--release")
            .current_dir(format!("{}/server", base_dir())),
    );
}
fn start_server_2() -> bool {
    return runshellcmd(
        "Starting server_2",
        Command::new("cargo")
            .arg("run")
            .arg("--release")
            .current_dir(format!("{}/server_2", base_dir())),
    );
}
fn start_server_async() -> bool {
    return runshellcmd(
        "Starting server_async",
        Command::new("cargo")
            .arg("run")
            .arg("--release")
            .env("RUST_LOG", "info")
            .current_dir(format!("{}/server_async", base_dir())),
    );
}
fn start_client() -> bool {
    return runshellcmd(
        "Starting client",
        Command::new("cargo")
            .arg("run")
            .arg("--release")
            .current_dir(format!("{}/client", base_dir())),
    );
}
fn test_server() -> bool {
    return runshellcmd(
        "Testing server",
        Command::new("cargo")
            .arg("test")
            .arg("--release")
            .arg("--")
            .arg("--nocapture")
            .current_dir(format!("{}/server/", base_dir())),
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
    watcher
        .watch(format!("{}/cli/src", base_dir()), RecursiveMode::Recursive)
        .unwrap();
    watcher
        .watch(
            format!("{}/cli/Cargo.toml", base_dir()),
            RecursiveMode::Recursive,
        )
        .unwrap();
    watcher
        .watch(
            format!("{}/server/src", base_dir()),
            RecursiveMode::Recursive,
        )
        .unwrap();
    watcher
        .watch(
            format!("{}/server/Cargo.toml", base_dir()),
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
