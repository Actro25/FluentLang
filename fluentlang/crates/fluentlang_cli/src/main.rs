use clap::Parser;
use fluentlang_core::Cli;
fn main() {
    //Creating base parsing that clap suggests.
    let cli = Cli::parse();

    //If there is an Error just show it.
    if let Err(err) = cli.process_command() {
        println!("{}", err);
    }
}