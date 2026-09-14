use crate::console::attributes::input::{ConfigCommands, ConfigSubcommands, Input, MainCommands};
use crate::console::json::{InputJson, Json};
use clap::Parser;

pub struct Cli {
    cli_data: Input,
    json: Json,
}

impl Cli {
    pub fn new() -> Self {
        let cli_data = Input::parse();
        let json = Json::new();
        Self { cli_data, json }
    }

    pub fn process_command(&self) {
        if let Some(_) = self.cli_data.sentence.as_deref() {
            println!("The API currently unavailable.");
        }

        match &self.cli_data.command {
            None => println!("No command specified."),
            Some(main_command) => match main_command {
                MainCommands::Config { command, .. } => match command {
                    ConfigCommands::Set { command, .. } => match command {
                        ConfigSubcommands::Keys(set_value) => {
                            let input = InputJson {
                                keys: Some(set_value.clone()),
                            };
                            match self.json.set_config_data(&input) {
                                Ok(_) => {}
                                Err(err) => println!("There is an error: {}", err),
                            };
                        }
                    },
                    ConfigCommands::Show(show_value) => show_value.show(),
                },
            },
        }
    }
}
