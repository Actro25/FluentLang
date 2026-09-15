use clap::Parser;

mod console;

fn main() {
    let cli = console::cli::Cli::parse();
    if let Err(err) = cli.process_command() {
        println!("{}",err);
    }
}