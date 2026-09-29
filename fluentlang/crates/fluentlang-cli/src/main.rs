pub mod cli;
pub mod config_parsing;
use crate::cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    // //Creating base parsing that clap suggests.
    // let cli = Cli::parse();
    //
    // //If there is an Error just show it.
    // if let Err(err) = cli.process_command() {
    //     println!("{}", err);
    // }
    fluentlang_api::api::GroqAPI::send_request("What are you looking at?".into()).await;
}