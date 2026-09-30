use clap::{Parser, Subcommand};
use fluentlang_core::config_parsing::{
    AvailableAiProviders, InputData, Keys, OutputData, get_config_data, get_path, set_config_data,
};
use fluentlang_api::api::GroqAPI;
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
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
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
pub enum SetSubcommands {
    #[command(
        version,
        about = "This is an Api provider.",
        long_about = r#"
This is an Api provider that was made by Groq team. If this provider demand only private key and doesn't provide public
you have to just enter private key without public.
    "#
    )]
    GroqCloud {
        #[command(subcommand)]
        command: SetArguments,
    },

    #[command(
        version,
        about = "This is an Api provider.",
        long_about = r#"
This is an Api provider that was made by Google team. If this provider demand only private key and doesn't provide public
you have to just enter private key without public.
    "#
    )]
    GoogleAiStudio {
        #[command(subcommand)]
        command: SetArguments,
    },

    #[command(
        version,
        about = "This is an Api provider.",
        long_about = r#"
This is an Api provider that was made by OpenRouter team. If this provider demand only private key and doesn't provide public
you have to just enter private key without public.
    "#
    )]
    OpenRouter {
        #[command(subcommand)]
        command: SetArguments,
    },

    #[command(
        version,
        about = "This is an Api provider.",
        long_about = r#"
This is an Api provider that was made by Cerebras team. If this provider demand only private key and doesn't provide public
you have to just enter private key without public.
    "#
    )]
    CerebrasInference {
        #[command(subcommand)]
        command: SetArguments,
    },

    #[command(
        version,
        about = "Set the current active provider.",
        long_about = r#"
This command sets the current active AI provider that will be used for sending requests.
    "#
    )]
    CurrentProvider {
        #[command(subcommand)]
        command: SetCurrentProvider,
    },
}

#[derive(Subcommand, Debug)]
pub enum SetCurrentProvider {
    #[command(
        version,
        about = "Just a provider.",
        long_about = "This is an Api provider that was made by Groq team."
    )]
    GroqCloud,

    #[command(
        version,
        about = "Just a provider.",
        long_about = "This is an Api provider that was made by Google team."
    )]
    GoogleAiStudio,

    #[command(
        version,
        about = "Just a provider.",
        long_about = "This is an Api provider that was made by OpenRouter team."
    )]
    OpenRouter,

    #[command(
        version,
        about = "Just a provider.",
        long_about = "This is an Api provider that was made by Cerebras team."
    )]
    CerebrasInference,
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

impl SetArguments {
    fn into_keys(self) -> Keys {
        match self {
            SetArguments::Private { key_value, .. } => Keys::Private(key_value),
            SetArguments::Public { key_value, .. } => Keys::Public(key_value),
        }
    }
}

impl Cli {
    pub async fn process_command(self) -> Result<(), AppErrors> {
        //If the firs argument isn't a sentence then return CurrentlyUnavailable.
        match self {
            Cli::Sentence { sentence, .. } => {
                //Firstly, we have to get what the current provider is.
                let config_data = match get_config_data(&get_path(CONFIG_NAME)?) {
                    Ok(data) => data,
                    Err(AppErrors::ConfigFileDoesntExist) => OutputData::default(),
                    Err(err) => return Err(err),
                };

                //When we have config data we send request to the corresponding provider.
                match config_data.current_provider {
                    AvailableAiProviders::GroqCloud => {
                        GroqAPI::send_request(
                            sentence,
                            config_data.groq_cloud.private,
                        )
                        .await?;
                    }
                    AvailableAiProviders::GoogleAiStudio => {
                        return Err(AppErrors::CurrentlyUnavailable);
                    }
                    AvailableAiProviders::OpenRouter => {
                        return Err(AppErrors::CurrentlyUnavailable);
                    }
                    AvailableAiProviders::CerebrasInference => {
                        return Err(AppErrors::CurrentlyUnavailable);
                    }
                    AvailableAiProviders::Unknow => return Err(AppErrors::UnknowProvider),
                    AvailableAiProviders::ProviderIsNotChosen => {
                        return Err(AppErrors::ProviderIsNotChosen);
                    }
                };
            }
            Cli::Config { command, .. } => match command {
                ConfigCommands::Set { command, .. } => {
                    //Parsing data into input
                    let input = match command {
                        SetSubcommands::GroqCloud { command, .. } => {
                            InputData::GroqCloud(command.into_keys())
                        }
                        SetSubcommands::GoogleAiStudio { command, .. } => {
                            InputData::GoogleAiStudio(command.into_keys())
                        }
                        SetSubcommands::OpenRouter { command, .. } => {
                            InputData::OpenRouter(command.into_keys())
                        }
                        SetSubcommands::CerebrasInference { command, .. } => {
                            InputData::CerebrasInference(command.into_keys())
                        }
                        SetSubcommands::CurrentProvider { command, .. } => {
                            let provider = match command {
                                SetCurrentProvider::GroqCloud => AvailableAiProviders::GroqCloud,
                                SetCurrentProvider::GoogleAiStudio => {
                                    AvailableAiProviders::GoogleAiStudio
                                }
                                SetCurrentProvider::OpenRouter => AvailableAiProviders::OpenRouter,
                                SetCurrentProvider::CerebrasInference => {
                                    AvailableAiProviders::CerebrasInference
                                }
                            };
                            InputData::CurrentProvider(provider)
                        }
                    };

                    set_config_data(&get_path(CONFIG_NAME)?, input)?;
                }
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