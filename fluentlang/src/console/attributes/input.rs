use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum MainCommands {
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Set {
        #[command(subcommand)]
        command: SetArguments,
    },
    Get,
}

#[derive(Subcommand, Debug)]
pub enum SetArguments {
    Private { key_value: String },
    Public { key_value: String },
}
