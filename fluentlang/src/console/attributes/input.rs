use crate::console::attributes::args::set::SetArg;
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
        command: ConfigSubcommands,
    },
    Show,
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubcommands {
    Keys(#[command(flatten)] SetArg),
}
