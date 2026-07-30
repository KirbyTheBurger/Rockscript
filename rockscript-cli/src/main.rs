use std::fs;

use clap::Parser as CliParser;

use crate::{repl::run_repl, utils::run};

mod repl;
mod utils;

#[derive(CliParser)]
#[command(about, version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run a program (.rock file)
    Run {
        filename: String,
        #[arg(short, long)]
        debug: bool,
    },

    /// Run the REPL
    Repl,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run {filename, debug} => {
            let source = fs::read_to_string(&filename);
            match source {
                Ok(s) => {
                    run(s, filename, debug);
                },

                Err(e) => println!("{e}"),
            }
        },

        Commands::Repl => {
            if let Err(e) = run_repl() {
                println!("Got error while attempting to edit REPL history: {e}");
            }
        }
    }
}