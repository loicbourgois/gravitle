mod logging;

#[cfg(test)]
#[path = "main_test.rs"]
mod tests;


use std::{
    env,
    path::PathBuf,
    process::{Command, ExitCode},
};

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell, generate};

#[derive(Parser)]
#[command(name = "g")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Up,
    Front,
    /// Run CLI tests; defaults to both Rust and Python.
    Test {
        #[arg(value_enum, default_value = "all")]
        suite: TestSuite,
    },
    /// Run the CLI work-in-progress todo workflow.
    Wip,
    /// Manage background rebuilding and installation of the CLI.
    Watch {
        #[arg(value_enum, default_value = "start")]
        action: WatchAction,
    },
    Completions {
        #[arg(value_enum, default_value = "zsh")]
        shell: CompletionShell,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum TestSuite {
    All,
    Python,
    Rust,
}

impl TestSuite {
    fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Python => "python",
            Self::Rust => "rust",
        }
    }

    fn suites(self) -> &'static [Self] {
        match self {
            Self::All => &[Self::Rust, Self::Python],
            Self::Python => &[Self::Python],
            Self::Rust => &[Self::Rust],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
enum WatchAction {
    Start,
    Status,
    Stop,
}

impl WatchAction {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Status => "status",
            Self::Stop => "stop",
        }
    }
}

#[derive(Clone, ValueEnum)]
enum CompletionShell {
    Bash,
    Zsh,
    Fish,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let command_name = match &cli.command {
        Commands::Up => "up",
        Commands::Front => "front",
        Commands::Test { .. } => "test",
        Commands::Wip => "wip",
        Commands::Watch { action } => action.as_str(),
        Commands::Completions { .. } => "completions",
    };
    let invocation = match &cli.command {
        Commands::Watch { .. } => format!("g watch {command_name}"),
        Commands::Test { suite } => format!("g test {}", suite.as_str()),
        _ => format!("g {command_name}"),
    };

    logging::record(&format!("Started {invocation}"));

    let result = match cli.command {
        Commands::Completions { shell } => {
            let shell = match shell {
                CompletionShell::Bash => Shell::Bash,
                CompletionShell::Zsh => Shell::Zsh,
                CompletionShell::Fish => Shell::Fish,
            };
            generate(shell, &mut Cli::command(), "g", &mut std::io::stdout());
            ExitCode::SUCCESS
        }
        Commands::Up => run_script("bash", "scripts/up.sh", &[]),
        Commands::Front => run_script("bash", "scripts/front.sh", &[]),
        Commands::Test { suite } => run_tests(suite),
        Commands::Wip => run_project_command("zsh", &["-i"], "cli/wip.sh", &[]),
        Commands::Watch { action } => {
            run_script("zsh", "cli/watch.sh", &[action.as_str()])
        }
    };

    logging::record(&format!(
        "Finished {invocation}: {}",
        if result == ExitCode::SUCCESS {
            "success"
        } else {
            "failure"
        }
    ));

    result
}

fn run_script(shell: &str, relative_path: &str, args: &[&str]) -> ExitCode {
    run_project_command(shell, &[], relative_path, args)
}

fn run_project_command(
    program: &str,
    prefix_args: &[&str],
    relative_path: &str,
    args: &[&str],
) -> ExitCode {
    let Some(home) = env::var_os("HOME") else {
        eprintln!("HOME is not set");
        return ExitCode::FAILURE;
    };

    let path = PathBuf::from(home)
        .join("github.com/loicbourgois/gravitle")
        .join(relative_path);

    match Command::new(program)
        .args(prefix_args)
        .arg(&path)
        .args(args)
        .status()
    {
        Ok(status) => {
            logging::record(&format!(
                "{program} for {} exited with {status}",
                path.display()
            ));
            ExitCode::from(status.code().unwrap_or(1).clamp(0, 255) as u8)
        }
        Err(error) => {
            let message =
                format!("failed to run {program} for {}: {error}", path.display());
            logging::record(&message);
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn test_commands(cli_dir: &std::path::Path, suite: TestSuite) -> Vec<Command> {
    suite
        .suites()
        .iter()
        .map(|suite| {
            let mut command = match suite {
                TestSuite::Rust => {
                    let mut command = Command::new("cargo");
                    command
                        .arg("test")
                        .arg("--manifest-path")
                        .arg(cli_dir.join("Cargo.toml"));
                    command
                }
                TestSuite::Python => {
                    let mut command = Command::new("uv");
                    command
                        .args([
                            "run",
                            "--no-project",
                            "python",
                            "-m",
                            "unittest",
                            "discover",
                            "-s",
                        ])
                        .arg(cli_dir)
                        .args(["-p", "*_test.py"]);
                    command
                }
                TestSuite::All => unreachable!("all expands into individual suites"),
            };
            command.current_dir(cli_dir);
            command
        })
        .collect()
}

fn run_tests(suite: TestSuite) -> ExitCode {
    let Some(home) = env::var_os("HOME") else {
        let message = "Cannot run tests: HOME is not set";
        logging::record(message);
        eprintln!("{message}");
        return ExitCode::FAILURE;
    };

    let cli_dir = PathBuf::from(home).join("github.com/loicbourgois/gravitle/cli");
    let mut succeeded = true;

    for mut command in test_commands(&cli_dir, suite) {
        let program = command.get_program().to_string_lossy().into_owned();

        match command.status() {
            Ok(status) => {
                logging::record(&format!("{program} tests exited with {status}"));
                if !status.success() {
                    succeeded = false;
                }
            }
            Err(error) => {
                let message = format!("Failed to start {program} tests: {error}");
                logging::record(&message);
                eprintln!("{message}");
                succeeded = false;
            }
        }
    }

    if succeeded {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}