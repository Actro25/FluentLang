mod console;

fn main() {
    let cli = console::cli::Cli::new();
    cli.process_command();
}
