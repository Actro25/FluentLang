use crate::table_components::art_component::ArtComponent;
use crate::table_components::chat_component::ChatComponent;
use crate::table_components::config_status_component::ConfigStatusComponent;
use crate::table_components::status_component::StatusComponent;
use crate::table_components::{Paddings, TableComponent};
use crate::tables::Table;
use crate::tables::fluent_lang_table::FluentLangTable;
use fluentlang_core::config_parsing::{AvailableAiProviders, OutputData};
use fluentlang_core::error::AppErrors;
use termimad::MadSkin;
use termimad::crossterm::style::Color;
use termimad::crossterm::terminal;
use terminal_colorsaurus::{QueryOptions, color_palette};

/// The function was created for building an output for showing answer from a provider's API.
/// The function builds a table from components, and after it renders the table.
/// The table shows art component, status component, and chat component.
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
            ArtComponent::new(width, table_paddings.clone()).get_component()?,
        ))
        .add_component(Box::new(
            StatusComponent::new(width, provider.clone(), table_paddings.clone())
                .get_component()?,
        ))
        .add_component(Box::new(
            ChatComponent::new(width, provider, sentence, answer, table_paddings)
                .get_component()?,
        ))
        .render(fg, bg);

    Ok(())
}

/// The function was created for building an output for showing the config data.
/// The function builds a table from components, and after it renders the table.
/// The table shows art component, and config status component.
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
            ArtComponent::new(width, table_paddings.clone()).get_component()?,
        ))
        .add_component(Box::new(
            ConfigStatusComponent::new(width, output_data, table_paddings).get_component()?,
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
