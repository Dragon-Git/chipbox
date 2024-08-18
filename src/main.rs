mod commands;

use clap::{Parser, Subcommand};
use commands::{SimCommand, WaveCommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
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
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Sim(sim)) => sim.run(),
        Some(Commands::Wave(wave)) => wave.run(),
        None => println!("No command specified, Please use -h for help"),
    }
}
