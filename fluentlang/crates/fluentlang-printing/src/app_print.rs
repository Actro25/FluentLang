use crate::tables::Table;
use crate::tables::components::art::ArtComponent;
use crate::tables::components::chat::ChatComponent;
use crate::tables::components::config_status::ConfigStatusComponent;
use crate::tables::components::status::StatusComponent;
use crate::tables::components::{Paddings, TableComponent};
use crate::tables::fluent_lang::FluentLangTable;
use fluentlang_utils::config_parsing::{AvailableAiProviders, OutputData};
use fluentlang_utils::error::AppErrors;
use termimad::MadSkin;
use termimad::crossterm::style::Color;
use termimad::crossterm::terminal;
use terminal_colorsaurus::{QueryOptions, color_palette};

/// The function was created for building an output for showing answer from a provider's API.
/// The function builds a table from components, and after it renders the table.
/// The table shows art component, status component, and chat component.
///
/// # Arguments
///
/// * `provider` - current chosen provider.
/// * `sentence` - a sentence that was sent to the provider's API.
/// * `answer` - an answer from the provider's API.
pub fn print_status(
    provider: AvailableAiProviders,
    sentence: String,
    answer: String,
    table_paddings: Paddings,
) -> Result<(), AppErrors> {
    let (width, _) = terminal::size()?;
    let palette = color_palette(QueryOptions::default())?;
    let fg = Color::Rgb {
        r: 113,
        g: 163,
        b: 255,
    };
    let bg = Color::Rgb {
        r: palette.background.r as u8,
        g: palette.background.g as u8,
        b: palette.background.b as u8,
    };

    FluentLangTable::default()
        .add_component(Box::new(
            ArtComponent::new(width, table_paddings.clone()).into_component()?,
        ))
        .add_component(Box::new(
            StatusComponent::new(width, provider.clone(), table_paddings.clone())
                .into_component()?,
        ))
        .add_component(Box::new(
            ChatComponent::new(width, provider, sentence, answer, table_paddings)
                .into_component()?,
        ))
        .render(fg, bg);

    Ok(())
}

/// The function was created for building an output for showing the config data.
/// The function builds a table from components, and after it renders the table.
/// The table shows art component, and config status component.
///
/// # Arguments
///
/// * `output_data` - the config data.
pub fn print_config_data(
    output_data: OutputData,
    table_paddings: Paddings,
) -> Result<(), AppErrors> {
    let (width, _) = terminal::size()?;

    let palette = color_palette(QueryOptions::default())?;
    let fg = Color::Rgb {
        r: 113,
        g: 163,
        b: 255,
    };
    let bg = Color::Rgb {
        r: palette.background.r as u8,
        g: palette.background.g as u8,
        b: palette.background.b as u8,
    };

    FluentLangTable::default()
        .add_component(Box::new(
            ArtComponent::new(width, table_paddings.clone()).into_component()?,
        ))
        .add_component(Box::new(
            ConfigStatusComponent::new(width, output_data, table_paddings).into_component()?,
        ))
        .render(fg, bg);

    Ok(())
}

pub fn get_skin() -> Result<MadSkin, AppErrors> {
    let palette = color_palette(QueryOptions::default())?;
    let fg = Color::Rgb {
        r: 113,
        g: 163,
        b: 255,
    };
    let bg = Color::Rgb {
        r: palette.background.r as u8,
        g: palette.background.g as u8,
        b: palette.background.b as u8,
    };
    let mut skin = MadSkin::default();
    skin.set_bg(bg);
    skin.set_fg(fg);

    Ok(skin)
}
