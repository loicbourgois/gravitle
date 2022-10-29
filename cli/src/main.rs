mod release;
use release::*;
extern crate clap;
extern crate dirs;
extern crate notify;
use clap::ArgMatches;
use clap::{App, SubCommand};
use notify::{watcher, RecursiveMode, Watcher};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::Instant;

#[derive(Deserialize)]
struct Configuration {
    host: String,
}

fn main() {
    let mut configuration_str = fs::read_to_string(format!("{}/configuration.yml", base_dir()))
        .expect("Something went wrong reading the file");
    let env_filename = format!("{}/../gravitle_local/gravitle.env", base_dir());
    let env_file = File::open(&env_filename).unwrap_or_else(|_| {
        panic!("Could not find {}",&env_filename)
    });
    let env_reader = BufReader::new(env_file);
    for (_index, line) in env_reader.lines().enumerate() {
        let line = line.unwrap();
        let split = line.split('=').collect::<Vec<&str>>();
        let key = format!("${{{}}}", split[0]);
        let value = split[1];
        match configuration_str.find(&key) {
            Some(_) => {
                configuration_str = configuration_str.replace(&key, value);
            }
            None => println!("Could not find key in configuration {}", &key),
        }
    }
    let configuration: Configuration = serde_yaml::from_str(&configuration_str).unwrap();
    let matches = App::new("gravitle")
        .usage("gravitle [COMMAND]")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("build"))
        .subcommand(SubCommand::with_name("lint"))
        .subcommand(SubCommand::with_name("format"))
        .subcommand(
            SubCommand::with_name("release")
                .setting(clap::AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("front"))
                .subcommand(SubCommand::with_name("server")),
        )
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
            SubCommand::with_name("host")
                .setting(clap::AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("setup"))
                .subcommand(SubCommand::with_name("sync"))
                .subcommand(SubCommand::with_name("run"))
                .subcommand(SubCommand::with_name("log"))
                .subcommand(SubCommand::with_name("kill"))
                .subcommand(SubCommand::with_name("ssh"))
                .subcommand(SubCommand::with_name("status")),
        )
        .subcommand(
            SubCommand::with_name("test")
                .setting(clap::AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("server")),
        )
        .subcommand(SubCommand::with_name("watch"))
        .subcommand(SubCommand::with_name("poc"))
        .get_matches();
    handle(matches, &configuration);
}

fn handle(matches: ArgMatches, configuration: &Configuration) {
    if matches.subcommand_matches("build").is_some() {
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
    } else if let Some(matches) = matches.subcommand_matches("release") {
        if matches.subcommand_matches("front").is_some() {
            release_front();
        } else if matches.subcommand_matches("server").is_some() {
            release_server();
        }
    } else if let Some(_matches) = matches.subcommand_matches("watch") {
        watch();
    } else if let Some(matches) = matches.subcommand_matches("host") {
        if matches.subcommand_matches("setup").is_some() {
            host_setup(&configuration.host);
        } else if matches.subcommand_matches("sync").is_some() {
            host_sync(&configuration.host);
        } else if matches.subcommand_matches("run").is_some() {
            host_run(&configuration.host);
        } else if matches.subcommand_matches("log").is_some() {
            host_log(&configuration.host);
        } else if matches.subcommand_matches("kill").is_some() {
            host_kill(&configuration.host);
        } else if matches.subcommand_matches("status").is_some() {
            host_status(&configuration.host);
        } else if matches.subcommand_matches("ssh").is_some() {
            host_ssh(&configuration.host);
        }
    }
}

fn host_kill(host: &str) -> bool {
    runshellcmd_default_title(
        Command::new("ssh")
            .arg(format!("gravitle@{}", host))
            .arg("screen -X -S server quit"),
    )
}

fn host_status(host: &str) -> bool {
    runshellcmd_default_title(
        Command::new("ssh")
            .arg(format!("gravitle@{}", host))
            .arg("screen -list"),
    )
}

fn host_sync(host: &str) -> bool {
    runshellcmd_default_title(
        Command::new("rsync")
            .arg("--recursive")
            .arg("--verbose")
            .arg("--exclude")
            .arg("target")
            .arg(format!("{}/server/", base_dir()))
            .arg(format!("gravitle@{}:/home/gravitle/github.com/loicbourgois/gravitle/server/", host)),
    ) && runshellcmd_default_title(
        Command::new("rsync")
            .arg("--recursive")
            .arg("--verbose")
            .arg("--exclude")
            .arg("target")
            .arg(format!("{}/core/", base_dir()))
            .arg(format!("gravitle@{}:/home/gravitle/github.com/loicbourgois/gravitle/core/", host)),
    )
    && runshellcmd_default_title(
        Command::new("ssh")
            .arg(format!("gravitle@{}", host))
            .arg("/home/gravitle/.cargo/bin/cargo build --release --manifest-path /home/gravitle/github.com/loicbourgois/gravitle/server/Cargo.toml"),
    )
}

fn host_ssh(host: &str) -> bool {
    runshellcmd_default_title(Command::new("ssh").arg(format!("gravitle@{}", host)))
}

fn host_setup(host: &str) -> bool {
    // eval "$(ssh-agent -s)"
    // ssh-add --apple-use-keychain $HOME/.ssh/gravitle
    // runshellcmd_default_title(
    //     Command::new("ssh-keygen").arg("-t").arg("ed25519")
    //         .arg("-C").arg(filename).arg("-f").arg(file_path),
    // );
    let start = Instant::now();
    let filename = "gravitle";
    let file_path = &format!("{}/.ssh/{}", home_dir(), filename);
    let filename_root = "loic@mac-perso@hetzner";
    let ssh_key_root = &format!("{}/.ssh/{}", home_dir(), filename_root);
    let sshd_config_local_path = &format!("{}/../gravitle_local/configs/sshd_config", base_dir());
    runshellcmd_default_title(
        Command::new("ssh-add")
            .arg("--apple-use-keychain")
            .arg(&ssh_key_root),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(ssh_key_root)
            .arg(format!("root@{}", host))
            .arg("pwd"),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg(format!("root@{}", host))
            .arg("adduser gravitle || true"),
    ) && runshellcmd_default_title(
        Command::new("ssh-copy-id")
            .arg("-f")
            .arg("-i")
            .arg(&file_path)
            .arg(format!("gravitle@{}", host)),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg("pwd"),
    ) && runshellcmd_default_title(
        Command::new("scp")
            .arg("-i")
            .arg(ssh_key_root)
            .arg(sshd_config_local_path)
            .arg(format!("root@{}:/etc/ssh/sshd_config", host)),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(ssh_key_root)
            .arg(format!("root@{}", host))
            .arg("systemctl restart ssh"),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg("pwd"),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg("/home/gravitle/.cargo/bin/cargo --version"),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg("mkdir -p /home/gravitle/github.com/loicbourgois/gravitle || true"),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(ssh_key_root)
            .arg(format!("root@{}", host))
            .arg("apt-get update"),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(ssh_key_root)
            .arg(format!("root@{}", host))
            .arg("apt-get install gcc gcc-multilib screen iptables nmap -y"),
    ) && host_sync(host)
        && success(&start)
}

fn host_run(host: &str) -> bool {
    let filename = "gravitle";
    let file_path = &format!("{}/.ssh/{}", home_dir(), filename);
    let screen_run = "screen -L -Logfile /home/gravitle/server.log -d -m -S server /home/gravitle/.cargo/bin/cargo run --release --manifest-path /home/gravitle/github.com/loicbourgois/gravitle/server/Cargo.toml";
    runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg("mkdir -p /home/gravitle/github.com/loicbourgois/gravitle_local/dna"),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg(screen_run),
    ) && runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg("screen -S server -X colon 'logfile flush 0^M'"),
    )
}

fn host_log(host: &str) -> bool {
    let file_path = &format!("{}/.ssh/gravitle", home_dir());
    runshellcmd_default_title(
        Command::new("ssh")
            .arg("-i")
            .arg(file_path)
            .arg(format!("gravitle@{}", host))
            .arg("tail -f server.log"),
    )
}

fn poc() {
    runshellcmd(
        "Poc",
        Command::new("python").arg(format!("{}/poc.py", base_dir())),
    );
}

fn runshellcmd_default_title(command: &mut Command) -> bool {
    let title = format!("{:?}", command);
    runshellcmd(&title, command)
}

fn runshellcmd(title: &str, command: &mut Command) -> bool {
    let start = Instant::now();
    println!("[ start ] {}", title);
    if let Ok(mut child) = command.spawn() {
        match child.wait().expect("error").code() {
            Some(code) => {
                if code == 0 {
                    println!("[  end  ] {} [{:?}]", title, start.elapsed());
                    return true;
                } else {
                    println!(
                        "[ error ] {} [{:?}] [code={}]",
                        title,
                        start.elapsed(),
                        code
                    );
                }
            }
            None => {
                println!("[ error ] {} [{:?}] [no code]", title, start.elapsed());
            }
        }
    } else {
        println!("[ error ] {} [{:?}] [no start]", title, start.elapsed());
    }
    false
}
fn success(start: &Instant) -> bool {
    println!("[success] [{:?}]", start.elapsed());
    true
}
fn build() -> bool {
    let start = Instant::now();
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
        // && runshellcmd(
        //     "Building wasm",
        //     Command::new("wasm-pack")
        //         .arg("build")
        //         .arg("--release")
        //         .env("RUSTFLAGS", "--cfg=web_sys_unstable_apis")
        //         .current_dir(format!("{}/wasm/", base_dir())),
        // )
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
        && success(&start);
}

fn lint() -> bool {
    let start = Instant::now();
    runshellcmd(
        "Linting cli",
        Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("--deny")
            .arg("warnings")
            .current_dir(format!("{}/cli/", base_dir())),
    ) && runshellcmd(
        "Linting server",
        Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("--deny")
            .arg("warnings")
            .current_dir(format!("{}/server/", base_dir())),
    ) && success(&start)
}

fn format() -> bool {
    let start = Instant::now();
    for project in ["cli", "client", "server", "wasm"] {
        if !runshellcmd(
            &format!("Formating {}", project),
            Command::new("cargo")
                .arg("fmt")
                .arg("--manifest-path")
                .arg(format!("{}/{}/Cargo.toml", base_dir(), project)),
        ) {
            return false;
        }
    }
    success(&start)
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

fn home_dir() -> String {
    dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

fn base_dir() -> String {
    return format!("{}/github.com/loicbourgois/gravitle", home_dir());
}
