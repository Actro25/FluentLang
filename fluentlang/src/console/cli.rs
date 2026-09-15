use crate::console::attributes::input::{ConfigCommands, MainCommands, SetArguments};
use crate::console::config_parsing::{InputJson, get_config_data, get_path, set_config_data};
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
                        SetArguments::Private { key_value, .. } => {
                            let mut input = InputJson::default();
                            input.private = Some(key_value.clone());
                            set_config_data(&get_path("config.json")?, &input)?;
                        }
                        SetArguments::Public { key_value, .. } => {
                            let mut input = InputJson::default();
                            input.public = Some(key_value.clone());
                            set_config_data(&get_path("config.json")?, &input)?;
                        }
                    },
                    ConfigCommands::Get => show_config_data()?,
                },
            }
        } else {
            println!("No command specified.")
        }

        Ok(())
    }
}

pub fn show_config_data() -> Result<(), String> {
    let output = get_config_data(&get_path("config.json")?)?;

    println!("Public key: {}", output.public);
    println!("Private key: {}", output.private);

    Ok(())
}