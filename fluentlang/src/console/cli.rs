use std::convert::Into;
use crate::console::attributes::input::{ConfigCommands, ConfigSubcommands, MainCommands};
use crate::console::config_parsing::{set_config_data, InputJson, get_path, get_config_data};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    pub sentence: Option<String>,

    #[command(subcommand)]
    pub command: Option<MainCommands>,
}

impl Cli {

    pub fn process_command(&self) -> Result<(), String> {
        if let Some(_) = self.sentence.as_ref() {
            println!("The API currently unavailable.");
        }

        if let Some(main_command) = &self.command {
            match main_command {
                MainCommands::Config { command, .. } => match command {
                    ConfigCommands::Set { command, .. } => match command {
                        ConfigSubcommands::Keys(set_value) => {
                            let input = InputJson {
                                keys: Some(set_value.clone()),
                            };
                            set_config_data(&get_path("config.json")?, &input)?;
                        }
                    },
                    ConfigCommands::Show => show_config_data()?,
                },
            }
        }
        else { println!("No command specified.") }

        Ok(())
    }
}

pub fn show_config_data() -> Result<(), String>{
    let output = get_config_data(&get_path("config.json")?)?;

    println!("Public key: {}", output.keys.public);
    println!("Private key: {}", output.keys.private);

    Ok(())
}