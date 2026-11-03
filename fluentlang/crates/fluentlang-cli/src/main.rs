pub mod cli;
use clap::Parser;
use fluentlang_printing::app_print::print_error;
use fluentlang_printing::tables::components::Paddings;
use crate::cli::Cli;

#[tokio::main]
async fn main() {
    //Creating base parsing that clap suggests.
    let cli = Cli::parse();

    //If there is an Error just show it.
    if let Err(err) = cli.process_command().await
        && let Err(err) = print_error(err.to_string(), Paddings::default())
    {
        println!("{}", err);
    }
}