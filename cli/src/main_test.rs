use crate::Cli;
use crate::Commands;
use crate::CompletionShell;
use clap::Parser;

#[test]
fn completions_default_to_zsh() {
    let cli = Cli::try_parse_from(["g", "completions"]).unwrap();

    assert!(matches!(
        cli.command,
        Commands::Completions {
            shell: CompletionShell::Zsh
        }
    ));
}

#[test]
fn tests_default_to_all() {
    let cli = Cli::try_parse_from(["g", "test"]).unwrap();

    assert!(matches!(
        cli.command,
        Commands::Test {
            suite: crate::TestSuite::All
        }
    ));
}

#[test]
fn tests_accept_explicit_suites() {
    for (argument, expected) in [
        ("all", crate::TestSuite::All),
        ("python", crate::TestSuite::Python),
        ("rust", crate::TestSuite::Rust),
    ] {
        let cli = Cli::try_parse_from(["g", "test", argument]).unwrap();

        match cli.command {
            Commands::Test { suite } => assert_eq!(suite, expected),
            _ => panic!("expected test command"),
        }
    }
}

#[test]
fn tests_reject_unknown_suites() {
    assert!(Cli::try_parse_from(["g", "test", "unknown"]).is_err());
}

#[test]
fn all_tests_expand_to_both_suites() {
    assert_eq!(
        crate::TestSuite::All.suites(),
        &[crate::TestSuite::Rust, crate::TestSuite::Python],
    );
}

#[test]
fn test_commands_use_absolute_project_paths_and_uv_for_python() {
    use std::{ffi::OsStr, path::Path};

    let directory = Path::new("/tmp/gravitle project/cli");
    let commands = crate::test_commands(directory, crate::TestSuite::All);

    assert_eq!(commands.len(), 2);

    let rust = &commands[0];
    assert_eq!(rust.get_program(), OsStr::new("cargo"));
    assert_eq!(rust.get_current_dir(), Some(directory));
    assert_eq!(
        rust.get_args().collect::<Vec<_>>(),
        vec![
            OsStr::new("test"),
            OsStr::new("--manifest-path"),
            OsStr::new("/tmp/gravitle project/cli/Cargo.toml"),
        ],
    );

    let python = &commands[1];
    assert_eq!(python.get_program(), OsStr::new("uv"));
    assert_eq!(python.get_current_dir(), Some(directory));
    assert_eq!(
        python.get_args().collect::<Vec<_>>(),
        vec![
            OsStr::new("run"),
            OsStr::new("--no-project"),
            OsStr::new("python"),
            OsStr::new("-m"),
            OsStr::new("unittest"),
            OsStr::new("discover"),
            OsStr::new("-s"),
            OsStr::new("/tmp/gravitle project/cli/scripts"),
            OsStr::new("-p"),
            OsStr::new("*_test.py"),
        ],
    );
}

#[test]
fn selecting_one_suite_builds_only_its_command() {
    let directory = std::path::Path::new("/tmp/gravitle/cli");

    for (suite, program) in [
        (crate::TestSuite::Rust, "cargo"),
        (crate::TestSuite::Python, "uv"),
    ] {
        let commands = crate::test_commands(directory, suite);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].get_program(), std::ffi::OsStr::new(program),);
    }
}
#[test]
fn format_command_is_accepted() {
    let cli = Cli::try_parse_from(["g", "format"]).unwrap();
    assert!(matches!(cli.command, Commands::Format));
}

#[test]
fn formatting_uses_the_cli_manifest_and_directory() {
    use std::{ffi::OsStr, path::Path};

    let directory = Path::new("/tmp/gravitle project/cli");
    let command = crate::format_command(directory);

    assert_eq!(command.get_program(), OsStr::new("cargo"));
    assert_eq!(command.get_current_dir(), Some(directory));
    assert_eq!(
        command.get_args().collect::<Vec<_>>(),
        vec![
            OsStr::new("fmt"),
            OsStr::new("--manifest-path"),
            OsStr::new("/tmp/gravitle project/cli/Cargo.toml"),
        ],
    );
}

#[test]
fn command_descriptions_match_their_behavior() {
    use clap::CommandFactory;

    let command = Cli::command();

    for (name, expected) in [
        (
            "completions",
            "Generate shell completions; defaults to Zsh",
        ),
        ("format", "Format the CLI Rust source with cargo fmt"),
        ("front", "Run the project's scripts/front.sh"),
        (
            "test",
            "Run CLI tests; defaults to both Rust and Python",
        ),
        ("up", "Run the project's scripts/up.sh"),
        (
            "watch",
            "Manage background rebuilding and installation of the CLI",
        ),
        ("wip", "Run the CLI work-in-progress todo workflow"),
        (
            "todo",
            "Generate todo improvement proposals from a YAML specification",
        ),
    ] {
        let subcommand = command
            .find_subcommand(name)
            .unwrap_or_else(|| panic!("missing command: {name}"));

        assert_eq!(
            subcommand.get_about().map(ToString::to_string),
            Some(expected.to_owned()),
            "incorrect description for g {name}",
        );
    }
}

#[test]
fn todo_requires_a_specification() {
    assert!(Cli::try_parse_from(["g", "todo"]).is_err());

    let cli = Cli::try_parse_from(["g", "todo", "/tmp/project/tasks.yml"]).unwrap();
    match cli.command {
        Commands::Todo { spec_file } => {
            assert_eq!(spec_file, "/tmp/project/tasks.yml");
        }
        _ => panic!("expected todo command"),
    }
}