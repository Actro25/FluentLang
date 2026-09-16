use clap::Parser;

mod console;

fn main() {
    //Creating base parsing that clap suggests.
    let cli = console::cli::Cli::parse();

    //If there is an Error just show it.
    if let Err(err) = cli.process_command() {
        println!("{}", err);
    }
}