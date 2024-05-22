mod commands;
mod state;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fmt;

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
        frequency: Frequency,
    },
    Do {
        id: u32,
    },
    Delete {
        id: u32,
    },
    Show {
        id: Option<u32>,
    },
    List {},
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize, Debug)]
enum Mode {
    Learn,
    Unlearn,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize, Debug)]
enum Frequency {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Habit {
    id: u32,
    name: String,
    learn: Mode,
    threshold: u32,
    frequency: Frequency,
}

impl fmt::Display for Habit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} description: {:?} type: {:?} threshold: {:?} frequency: {:?})",
            self.id, self.name, self.learn, self.threshold, self.frequency
        )
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create {
            name,
            learn,
            threshold,
            frequency,
        } => commands::create::run(name.to_string(), *learn, *threshold, *frequency).expect("Oops"),
        Commands::List {} => {
            let result = commands::list::run().expect("Oops");
            println!("{:?}", result);
        }
        _ => {
            println!("OOPS UNIMPLEMENTED")
        }
    }
}
