use clap::{Parser, Subcommand};
use std::process::ExitCode;

/// ormus: the person-facing command. Rust port in progress.
#[derive(Parser, Debug)]
#[command(name = "ormus", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Check the local setup
    Doctor,
    /// Print shell completions
    Completions { shell: String },
    /// Connect a channel or service
    Connect {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Interactive setup
    Setup,
    /// Initialise a project folder
    Init {
        project_folder: String,
        #[arg(long)]
        profile: Option<String>,
        #[arg(long)]
        state: Option<String>,
    },
    /// Manage channel pairing
    Pairing {
        #[command(subcommand)]
        action: PairingAction,
    },
}

#[derive(Subcommand, Debug)]
enum PairingAction {
    /// Approve a pairing code for a channel
    Approve {
        channel: String,
        code: String,
        #[arg(long)]
        profile: Option<String>,
        #[arg(long)]
        state: Option<String>,
    },
}

fn name(cmd: &Option<Command>) -> &'static str {
    match cmd {
        None => "tui",
        Some(Command::Doctor) => "doctor",
        Some(Command::Completions { .. }) => "completions",
        Some(Command::Connect { .. }) => "connect",
        Some(Command::Setup) => "setup",
        Some(Command::Init { .. }) => "init",
        Some(Command::Pairing { .. }) => "pairing approve",
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    eprintln!("{}", ormus_core::not_yet_ported(name(&cli.command)));
    ExitCode::from(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_pairing() {
        let c = Cli::parse_from(["ormus", "pairing", "approve", "slack", "1234"]);
        assert_eq!(name(&c.command), "pairing approve");
    }
}
