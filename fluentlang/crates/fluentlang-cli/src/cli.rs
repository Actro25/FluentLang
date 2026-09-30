use clap::{Parser, Subcommand};
use fluentlang_core::config_parsing::{get_config_data, get_path, set_config_data, InputData, Keys, AvailableAiProviders, OutputData};
use fluentlang_core::error::AppErrors;

const CONFIG_NAME: &str = "config.json";

#[derive(Parser, Debug)]
#[command(
    name = "fluentlang",
    version,
    long_about = r#"
This is base struct for cli.
For example:
fluentlang "This is my first sentence!"
    ^                  ^
This is key word       |
           This is a sentence that you want to understand"#
)]
pub enum Cli {
    #[command(
        version,
        about = "An attribute that returns explained sentence",
        long_about = r#"
This is subcommand for key work "fluentlang"
For example:
fluentlang sentence "This is my first sentence!"
               ^        ^
This is subcommand      |
           This is a sentence that you want to understand"#
    )]
    Sentence {
        #[arg(value_name = "SENTENCE")]
        sentence: String,
    },
    #[command(
        version,
        about = "An attribute that helps to set up the config file",
        long_about = r#"
This is subcommand for key work "fluentlang"
For example:
fluentlang config set public "PUBLIC-KEY"
               ^   ^
This is subcommand |
           This is also a subcommand but for "config""#
    )]
    Config{
        #[command(subcommand)]
        command: ConfigCommands,
    }
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    #[command(
        version,
        about = "An attribute that setting up the config file by values",
        long_about = r#"
This is also subcommand struct but for MainCommands struct
For example:
fluentlang config set private "YOUR-PRIVATE-KEY"
                  ^
This is subcommand for config. This will help you to set data by parameters."#
    )]
    Set {
        #[command(subcommand)]
        command: SetSubcommands,
    },

    #[command(
        version,
        about = "An attribute that gets and shows all the config data",
        long_about = r#"
This is also subcommand struct but for MainCommands struct
For example:
fluentlang config get
                  ^
This is subcommand for config. It'll show config data."#
    )]
    Get,
}

#[derive(Subcommand, Debug)]
pub enum SetSubcommands{
    GroqCloud {
        #[command(subcommand)]
        command: SetArguments,
    },
    GoogleAiStudio {
        #[command(subcommand)]
        command: SetArguments,
    },
    OpenRouter {
        #[command(subcommand)]
        command: SetArguments,
    },
    CerebrasInference {
        #[command(subcommand)]
        command: SetArguments,
    },
    CurrentProvider {
        #[command(subcommand)]
        command: SetCurrentProvider,
    }
}

#[derive(Subcommand, Debug)]
pub enum SetCurrentProvider{
    #[command(version)]
    GroqCloud{},

    #[command(version)]
    GoogleAiStudio{},

    #[command(version)]
    OpenRouter{},

    #[command(version)]
    CerebrasInference{}
}

#[derive(Subcommand, Debug)]
pub enum SetArguments {
    #[command(
        version,
        about = "A private key parameter",
        long_about = r#"
This is arguments struct for config data that is also subcommand.
For example:
fluentlang config set private "YOUR-PRIVATE-KEY"
                        ^
This is a config parameters that contains a value."#
    )]
    Private { key_value: String },

    #[command(
        version,
        about = "A public key parameter",
        long_about = r#"
This is arguments struct for config data that is also subcommand.
For example:
fluentlang config set public "YOUR-PRIVATE-KEY"
                        ^
This is a config parameters that contains a value."#
    )]
    Public { key_value: String },
}

impl Cli {
    pub async fn process_command(self) -> Result<(), AppErrors> {
        //If the firs argument isn't a sentence then return CurrentlyUnavailable.
        match self {
            Cli::Sentence { sentence, .. } => {
                let mut config_data = match get_config_data(&get_path(CONFIG_NAME)?) {
                    Ok(data) => data,
                    Err(AppErrors::ConfigFileDoesntExist) => OutputData::default(),
                    Err(err) => return Err(err)
                };

                match config_data.current_provider {
                    AvailableAiProviders::GroqCloud => {
                        fluentlang_api::api::GroqAPI::send_request(sentence, config_data.groq_cloud.private).await?;
                    },
                    AvailableAiProviders::GoogleAiStudio => return Err(AppErrors::CurrentlyUnavailable),
                    AvailableAiProviders::OpenRouter => return Err(AppErrors::CurrentlyUnavailable),
                    AvailableAiProviders::CerebrasInference => return Err(AppErrors::CurrentlyUnavailable),
                    AvailableAiProviders::Unknow => return Err(AppErrors::UnknowProvider),
                    AvailableAiProviders::ProviderIsNotChosen => return Err(AppErrors::ProviderIsNotChosen),
                };

            }
            Cli::Config { command, .. } => match command {
                ConfigCommands::Set { command, .. } => match command {
                    SetSubcommands::GroqCloud { command, .. } => match command {
                        SetArguments::Private { key_value, .. } => {
                            //Creating input data for the JSON setting
                            //Input data that I want to save in config file
                            let input = InputData::GroqCloud(Keys::Private(key_value));
                            //Call set function with path where we want to save config data.
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        }
                        SetArguments::Public { key_value, .. } => {
                            let input = InputData::GroqCloud(Keys::Public(key_value));
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        }
                    },
                    SetSubcommands::GoogleAiStudio { command, .. } => match command {
                        SetArguments::Private { key_value, .. } => {
                            let input = InputData::GoogleAiStudio(Keys::Private(key_value));
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        }
                        SetArguments::Public { key_value, .. } => {
                            let input = InputData::GoogleAiStudio(Keys::Public(key_value));
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        }
                    },
                    SetSubcommands::OpenRouter { command, .. } => match command {
                        SetArguments::Private { key_value, .. } => {
                            let input = InputData::OpenRouter(Keys::Private(key_value));
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        }
                        SetArguments::Public { key_value, .. } => {
                            let input = InputData::OpenRouter(Keys::Public(key_value));
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        }
                    },
                    SetSubcommands::CerebrasInference { command, .. } => match command {
                        SetArguments::Private { key_value, .. } => {
                            let input = InputData::CerebrasInference(Keys::Private(key_value));
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        }
                        SetArguments::Public { key_value, .. } => {
                            let input = InputData::CerebrasInference(Keys::Public(key_value));
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        }
                    },
                    SetSubcommands::CurrentProvider { command, .. } => match command {
                        SetCurrentProvider::GroqCloud { .. } => {
                            let input = InputData::CurrentProvider(AvailableAiProviders::GroqCloud);
                            //Call set function with path where we want to save config data.
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        },
                        SetCurrentProvider::GoogleAiStudio { .. } => {
                            let input = InputData::CurrentProvider(AvailableAiProviders::GoogleAiStudio);
                            //Call set function with path where we want to save config data.
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        },
                        SetCurrentProvider::OpenRouter { .. } => {
                            let input = InputData::CurrentProvider(AvailableAiProviders::OpenRouter);
                            //Call set function with path where we want to save config data.
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        },
                        SetCurrentProvider::CerebrasInference { .. } => {
                            let input = InputData::CurrentProvider(AvailableAiProviders::CerebrasInference);
                            //Call set function with path where we want to save config data.
                            set_config_data(&get_path(CONFIG_NAME)?, input)?;
                        },
                    },
                },
                ConfigCommands::Get => show_config_data()?,
            },
        }

        Ok(())
    }
}

pub fn show_config_data() -> Result<(), AppErrors> {
    //Getting config data to show
    let output = get_config_data(&get_path(CONFIG_NAME)?)?;

    println!("=== Current Configuration ===");
    println!("Current Active Provider: {:?}\n", output.current_provider);

    println!("--- Groq Cloud ---");
    println!("  Public:  {}", output.groq_cloud.public);
    println!("  Private: {}", output.groq_cloud.private);

    println!("\n--- Google AI Studio ---");
    println!("  Public:  {}", output.google_ai_studio.public);
    println!("  Private: {}", output.google_ai_studio.private);

    println!("\n--- Open Router ---");
    println!("  Public:  {}", output.open_router.public);
    println!("  Private: {}", output.open_router.private);

    println!("\n--- Cerebras Inference ---");
    println!("  Public:  {}", output.cerebras_inference.public);
    println!("  Private: {}", output.cerebras_inference.private);

    Ok(())
}