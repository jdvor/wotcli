mod common;
mod tankopedia;

use clap::{Args, Parser, Subcommand};

/// Command line tool for World of Tanks players.
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Jan Dvorak<jandvorak.public@gmail.com>")]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Init(Init),
    Tankopedia(Tankopedia),
    Upload(Upload)
}

/// Downloads tankopedia data and stores them locally.
#[derive(Args)]
struct Init {
    /// Path where to save tankopedia data.
    target_path: Option<String>
}

/// Downloads tankopedia data and stores them locally.
#[derive(Args)]
struct Tankopedia {
    /// Path where to save tankopedia data.
    #[clap(short, long)]
    target_path: Option<String>
}

/// Get a number of quotes to start your day
#[derive(Args)]
struct Upload {
    /// The directory with battle replays. If none is provided,
    /// there will be an attempt to search for it in usual location.
    replay_dir: Option<String>
}

pub trait Executable {
    fn execute(&self) -> String;
}

impl Executable for Tankopedia {
    fn execute(&self) -> String {
        tankopedia::exec()
    }
}

impl Executable for Upload {
    fn execute(&self) -> String {
        String::from("Upload")
    }
}

fn main() {
    let opts: Cli = Cli::parse();

    match opts.action {
        Action::Tankopedia(c) => {
            println!("{}", c.execute());
        }
        Action::Upload(c) => {
            println!("{}", c.execute());
        }
    }
}
