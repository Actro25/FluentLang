pub mod cli;
use clap::Parser;
use crate::cli::Cli;

#[tokio::main]
async fn main() {
    //Creating base parsing that clap suggests.
    let cli = Cli::parse();

    //If there is an Error just show it.
    if let Err(err) = cli.process_command().await {
        println!("{}", err);
    }
}