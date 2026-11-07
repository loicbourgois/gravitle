#[cfg(test)]
#[path = "main_test.rs"]
mod tests;

mod logging;
mod openrouter;
mod todo;
mod review;

use std::{
    env,
    path::PathBuf,
    process::{Command, ExitCode},
};
use std::{
    io::{self, BufReader},
    process::{ExitStatus, Stdio},
    thread,
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
    /// Generate shell completions; defaults to Zsh.
    Completions {
        #[arg(value_enum, default_value = "zsh")]
        shell: CompletionShell,
    },
    /// Format the CLI Rust source with cargo fmt.
    Format,
    /// Run the project's scripts/front.sh.
    Front,
    /// Run Moonlighter workflows.
    Moon {
        #[command(subcommand)]
        command: MoonCommands,
    },
    /// Review files from a YAML specification or a Moonlighter review prefix.
    Review {
        /// YAML path or unique Moonlighter review prefix, such as 01.
        specification: String,
    },
    /// Run CLI tests; defaults to both Rust and Python.
    Test {
        #[arg(value_enum, default_value = "all")]
        suite: TestSuite,
    },
    /// Generate todo improvement proposals from a YAML specification.
    Todo {
        /// YAML specification; relative file references resolve beside this file.
        spec_file: String,
    },
    /// Run the project's scripts/up.sh.
    Up,
    /// Manage background rebuilding and installation of the CLI.
    Watch {
        #[arg(value_enum, default_value = "start")]
        action: WatchAction,
    },
    /// Run the CLI work-in-progress todo workflow.
    Wip,
}

#[derive(Subcommand)]
enum MoonCommands {
    /// Generate improvement proposals from the Moonlighter specification.
    Wip,
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
        Commands::Format => "format",
        Commands::Test { .. } => "test",
        Commands::Moon { .. } => "moon",
        Commands::Review { .. } => "review",
        Commands::Wip => "wip",
        Commands::Watch { action } => action.as_str(),
        Commands::Todo { .. } => "todo",
        Commands::Completions { .. } => "completions",
    };
    let invocation = match &cli.command {
        Commands::Watch { .. } => format!("g watch {command_name}"),
        Commands::Test { suite } => format!("g test {}", suite.as_str()),
        Commands::Moon {
            command: MoonCommands::Wip,
        } => "g moon wip".to_owned(),
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
        Commands::Todo { spec_file } => run_todo(&spec_file),
        Commands::Review { specification } => run_review(&specification),
        Commands::Moon {
            command: MoonCommands::Wip,
        } => {
            let Some(home) = env::var_os("HOME") else {
                let message = "Cannot run Moonlighter wip: HOME is not set";
                logging::record(message);
                if logging::command_message(message).is_err() {
                    eprintln!("{message}");
                }
                return ExitCode::FAILURE;
            };

            let spec_path = moon_wip_spec_path(std::path::Path::new(&home));
            run_todo(&spec_path.to_string_lossy())
        }
        Commands::Up => run_script("bash", "scripts/up.sh", &[]),
        Commands::Front => run_script("bash", "scripts/front.sh", &[]),
        Commands::Format => run_format(),
        Commands::Test { suite } => run_tests(suite),
        Commands::Wip => run_script("bash", "cli/scripts/wip.sh", &[]),
        Commands::Watch { action } => run_script("zsh", "cli/scripts/watch.sh", &[action.as_str()]),
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

fn run_review(specification: &str) -> ExitCode {
    match review::review(specification) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let message = format!("Review failed: {error}");
            logging::record(&message);
            if logging::command_message(&message).is_err() {
                eprintln!("{message}");
            }
            ExitCode::FAILURE
        }
    }
}

fn moon_wip_spec_path(home: &std::path::Path) -> PathBuf {
    home.join("github.com/loicbourgois/gravitle/front/moonlighter/wip.yml")
}

fn run_todo(spec_file: &str) -> ExitCode {
    match todo::todo(spec_file) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let message = format!("Todo failed: {error}");
            logging::record(&message);
            if logging::command_message(&message).is_err() {
                eprintln!("{message}");
            }
            ExitCode::FAILURE
        }
    }
}

fn run_script(shell: &str, relative_path: &str, args: &[&str]) -> ExitCode {
    run_project_command(shell, &[], relative_path, args)
}

fn format_command(cli_dir: &std::path::Path) -> Command {
    let mut command = Command::new("cargo");
    command
        .arg("fmt")
        .arg("--manifest-path")
        .arg(cli_dir.join("Cargo.toml"))
        .current_dir(cli_dir);
    command
}

fn run_format() -> ExitCode {
    let Some(home) = env::var_os("HOME") else {
        eprintln!("Cannot format CLI: HOME is not set");
        return ExitCode::FAILURE;
    };

    let cli_dir = PathBuf::from(home).join("github.com/loicbourgois/gravitle/cli");

    match run_logged_command(&mut format_command(&cli_dir)) {
        Ok(status) => {
            logging::record(&format!("cargo fmt exited with {status}"));
            ExitCode::from(status.code().unwrap_or(1).clamp(0, 255) as u8)
        }
        Err(error) => {
            let message = format!("Failed to run or log cargo fmt: {error}");
            logging::record(&message);
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
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
    let project_dir = PathBuf::from(home).join("github.com/loicbourgois/gravitle");
    let path = project_dir.join(relative_path);

    let mut command = project_command(
        &project_dir,
        program,
        prefix_args,
        relative_path,
        args,
    );

    let log_name = if relative_path == "scripts/front.sh" {
        "front.log"
    } else {
        "terminal.log"
    };

    match run_logged_command_to(&mut command, log_name) {
        Ok(status) => {
            logging::record(&format!(
                "{program} for {} exited with {status}",
                path.display()
            ));
            ExitCode::from(status.code().unwrap_or(1).clamp(0, 255) as u8)
        }
        Err(error) => {
            let message = format!(
                "Failed to run or log {program} for {}: {error}",
                path.display()
            );
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
            "-u",
            "-m",
            "unittest",
            "discover",
            "-s",
        ])
        .arg(cli_dir.join("scripts"))
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

        match run_logged_command(&mut command) {
            Ok(status) => {
                logging::record(&format!("{program} tests exited with {status}"));
                if !status.success() {
                    succeeded = false;
                }
            }
            Err(error) => {
                let message = format!("Failed to run or log {program} tests: {error}");
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

fn run_logged_command(command: &mut Command) -> io::Result<ExitStatus> {
    run_logged_command_to(command, "terminal.log")
}

fn project_command(
    project_dir: &std::path::Path,
    program: &str,
    prefix_args: &[&str],
    relative_path: &str,
    args: &[&str],
) -> Command {
    let mut command = Command::new(program);
    command
        .args(prefix_args)
        .arg(project_dir.join(relative_path))
        .args(args)
        .current_dir(project_dir);
    command
}

fn run_logged_command_to(command: &mut Command, log_name: &str) -> io::Result<ExitStatus> {
    let mut child = command
        .env(logging::OUTPUT_DELEGATED_ENV, "1")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("stdout was piped");
    let stderr = child.stderr.take().expect("stderr was piped");

    thread::scope(|scope| {
        let stdout_worker =
            scope.spawn(|| logging::forward_command_output_to(BufReader::new(stdout), log_name));
        let stderr_worker =
            scope.spawn(|| logging::forward_command_output_to(BufReader::new(stderr), log_name));

        let status = child.wait();

        let stdout_result = stdout_worker
            .join()
            .unwrap_or_else(|_| Err(io::Error::other("stdout reader panicked")));
        let stderr_result = stderr_worker
            .join()
            .unwrap_or_else(|_| Err(io::Error::other("stderr reader panicked")));

        let status = status?;
        stdout_result?;
        stderr_result?;
        Ok(status)
    })
}
