use clap::{Parser, Subcommand, ValueEnum};
use fluentlang_api::provider_response::get_response;
use fluentlang_api::providers::google_ai_studio::GoogleAiStudioAPI;
use fluentlang_api::providers::groq_api::GroqAPI;
use fluentlang_api::providers::open_router::OpenRouterAPI;
use fluentlang_printing::app_print::{print_config_data, print_status};
use fluentlang_printing::tables::components::Paddings;
use fluentlang_utils::config_parsing::{
    AvailableAiProviders, InputData, Key, OutputData, get_config_data, get_path, set_config_data,
};
use fluentlang_utils::error::AppErrors;

const CONFIG_NAME: &str = "config.json";

#[derive(Parser, Debug)]
#[command(
    name = "fluentlang",
    version,
    about = "CLI tool to analyze and explain sentences using AI providers",
    long_about = r#"A command-line application that connects to AI providers (Groq, Google AI Studio, OpenRouter, Cerebras) to generate detailed explanations of sentences.

Examples:
  fluentlang sentence "Break down this sentence for me."
  fluentlang config default groq-cloud
  fluentlang config set groq.private_key "gsk_..."
"#
)]
pub enum Cli {
    #[command(
        version,
        about = "Explain and break down a given sentence",
        long_about = r#"Sends a prompt to the currently configured AI provider to receive a detailed breakdown and explanation of the input text.

Example:
  fluentlang sentence "The quick brown fox jumps over the lazy dog."
"#
    )]
    Sentence {
        #[arg(value_name = "SENTENCE")]
        sentence: String,
    },

    #[command(
        version,
        about = "Manage configuration settings and API credentials",
        long_about = r#"Configure default AI providers, manage private/public API keys, and view stored application settings.

Examples:
  fluentlang config
  fluentlang config default groq-cloud
  fluentlang config set google.api_key "AIzaSy..."
"#
    )]
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    #[command(
        version,
        about = "Set an API key or config property",
        long_about = r#"Sets a configuration entry such as an API key for a specific AI provider.

Examples:
  fluentlang config set groq-cloud.private_key "gsk_..."
  fluentlang config set open-router.public_key "pk_..."
"#
    )]
    Set { key: ConfigKey, value: String },

    #[command(
        version,
        about = "Set the default active AI provider",
        long_about = r#"Switches the primary AI provider used for analyzing sentences.

Example:
  fluentlang config default groq-cloud
"#
    )]
    Default { provider: Providers },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ConfigKey {
    #[value(name = "groq-cloud.private_key")]
    GroqPrivateKey,

    #[value(name = "google-ai.private_key")]
    GooglePrivateKey,

    #[value(name = "open-router.private_key")]
    OpenRouterPrivateKey,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Providers {
    #[value(name = "groq-cloud", alias = "groq")]
    GroqCloud,

    #[value(name = "google-ai")]
    GoogleAiStudio,

    #[value(name = "open-router")]
    OpenRouter,
}

impl Cli {
    pub async fn process_command(self) -> Result<(), AppErrors> {
        match self {
            Cli::Sentence { sentence, .. } => sentence_command(sentence).await?,
            Cli::Config { command, .. } => match command {
                None => {
                    let config_data = match get_config_data(&get_path(CONFIG_NAME)?) {
                        Ok(data) => data,
                        Err(AppErrors::ConfigFileDoesntExist) => OutputData::default(),
                        Err(err) => return Err(err),
                    };

                    let paddings = Paddings::default();

                    print_config_data(config_data, paddings)?;
                }
                Some(command) => config_command(command)?,
            },
        }

        Ok(())
    }
}

/// The function sends request to the current provider's API.
///
/// # Arguments
///
/// * `sentence` - a sentence that the user wants to know all about.
async fn sentence_command(sentence: String) -> Result<(), AppErrors> {
    //Firstly, we have to get what the current provider is.
    let config_data = match get_config_data(&get_path(CONFIG_NAME)?) {
        Ok(data) => data,
        Err(AppErrors::ConfigFileDoesntExist) => OutputData::default(),
        Err(err) => return Err(err),
    };

    //When we have config data we send request to the corresponding provider.
    let answer = match config_data.current_provider {
        AvailableAiProviders::GroqCloud => {
            get_response(
                GroqAPI::default(),
                sentence.as_str(),
                get_private_key(&config_data).as_str(),
            )
            .await?
        }
        AvailableAiProviders::GoogleAiStudio => {
            get_response(
                GoogleAiStudioAPI::default(),
                sentence.as_str(),
                get_private_key(&config_data).as_str(),
            )
            .await?
        }
        AvailableAiProviders::OpenRouter => {
            get_response(
                OpenRouterAPI::default(),
                sentence.as_str(),
                get_private_key(&config_data).as_str(),
            )
            .await?
        }
        AvailableAiProviders::Unknow => return Err(AppErrors::UnknowProvider),
        AvailableAiProviders::ProviderIsNotChosen => return Err(AppErrors::ProviderIsNotChosen),
    };
    
    let paddings = Paddings::default();

    print_status(config_data.current_provider, sentence, answer, paddings)?;

    Ok(())
}

fn get_private_key(config_data: &OutputData) -> String {
    match &config_data.current_provider {
        AvailableAiProviders::GroqCloud => config_data.groq_cloud.private.clone(),
        AvailableAiProviders::GoogleAiStudio => config_data.google_ai_studio.private.clone(),
        AvailableAiProviders::OpenRouter => config_data.open_router.private.clone(),
        _ => "".to_string(),
    }
}

/// The function writes input data into the config file.
///
/// # Arguments
///
/// * `command` - an enum of the config subcommand.
fn config_command(command: ConfigCommands) -> Result<(), AppErrors> {
    match command {
        ConfigCommands::Set { key, value } => {
            let input = match key {
                ConfigKey::GroqPrivateKey => InputData::GroqCloud(Key::Private(value)),
                ConfigKey::GooglePrivateKey => InputData::GoogleAiStudio(Key::Private(value)),
                ConfigKey::OpenRouterPrivateKey => InputData::OpenRouter(Key::Private(value)),
            };

            set_config_data(&get_path(CONFIG_NAME)?, input)?;
        }
        ConfigCommands::Default { provider } => {
            let input = match provider {
                Providers::GroqCloud => InputData::CurrentProvider(AvailableAiProviders::GroqCloud),
                Providers::GoogleAiStudio => {
                    InputData::CurrentProvider(AvailableAiProviders::GoogleAiStudio)
                }
                Providers::OpenRouter => {
                    InputData::CurrentProvider(AvailableAiProviders::OpenRouter)
                }
            };

            set_config_data(&get_path(CONFIG_NAME)?, input)?;
        }
    };

    Ok(())
}