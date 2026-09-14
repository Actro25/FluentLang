use crate::console::attributes::input::{ConfigSubcommands, Input, MainCommands};
use clap::Parser;
pub struct Cli {
    cli_data: Input,
}

impl Cli {
    pub fn new() -> Self {
        let cli_data = Input::parse();
        Self { cli_data }
    }

    pub fn process_command(&self) {
        if let Some(value) = self.cli_data.sentence.as_deref() {
            println!("The API currently unavailable.");
        }

        match &self.cli_data.command {
            None => println!("No command specified."), //There should be adequate output like err or sth else.
            Some(main_command) => match main_command {
                MainCommands::Config { command, .. } => match command {
                    ConfigSubcommands::Set(set_value) => set_value.set(),
                    ConfigSubcommands::Show(show_value) => show_value.show(),
                },
            },
        }
    }
}
