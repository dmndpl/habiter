mod commands;
mod state;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { 
        name: String,

        #[arg(value_enum)]
        learn: Mode,

        threshold: u32,

        #[arg(value_enum)]
        frequency: Frequency
    },
    Do { id: u32},
    Delete { id: u32},
    List { id: Option<u32>},
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
enum Mode {
    Learn,
    Unlearn,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
enum Frequency {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Serialize, Deserialize)]
struct Habit {
    id: u32,
    name: String,
    learn: Mode,
    threshold: u32,
    frequency: Frequency
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name, learn, threshold, frequency } => {
            commands::create::run(name.to_string(), *learn, *threshold, *frequency).expect("Oops")
        }
        _ => {
            println!("OOPS UNIMPLEMENTED")
        }
    }
}
