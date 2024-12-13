mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Play
}

fn main() {
    let cli = Cli::parse();
    env_logger::init();

    match &cli.command {
        Some(Commands::Play) => {
            commands::play::play_doppelkopf();
        }
        None => { }
    }
}

