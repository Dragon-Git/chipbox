mod commands;

use clap::{CommandFactory, Parser, Subcommand};
use commands::{CompletionCommand, LintCommand, SimCommand, WaveCommand};
use std::borrow::BorrowMut;

#[derive(Parser)]
#[command(version, about, long_about = None, after_help = "Type 'chipbox <SUBCOMMAND> -h' for more information about a subcommand")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Simulation command
    Sim(SimCommand),
    /// Waveform command
    Wave(WaveCommand),
    /// Waveform command
    Lint(LintCommand),
    /// Completion command
    Completion(CompletionCommand),
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Sim(sim)) => sim.run(),
        Some(Commands::Wave(wave)) => wave.run(),
        Some(Commands::Lint(lint)) => lint.run(),
        Some(Commands::Completion(completion)) => completion.run(Cli::command().borrow_mut()),
        None => println!("No command specified, Please use -h for help"),
    }
}
