pub mod cli;
pub mod config_parsing;
use crate::cli::Cli;
use clap::Parser;

fn main() {
    //Creating base parsing that clap suggests.
    let cli = Cli::parse();

    //If there is an Error just show it.
    if let Err(err) = cli.process_command() {
        println!("{}", err);
    }
}