use termimad::crossterm::style::{Color, Stylize};
use termimad::crossterm::terminal;
use termimad::MadSkin;
use crate::table_components::TableComponent;
use crate::table_components::art_component::ArtComponent;
use crate::table_components::chat_component::ChatComponent;
use crate::table_components::config_status_component::ConfigStatusComponent;
use crate::table_components::status_component::StatusComponent;
use fluentlang_core::config_parsing::{AvailableAiProviders, OutputData};
use fluentlang_core::error::AppErrors;
use terminal_colorsaurus::{QueryOptions, color_palette};

pub struct FluentLangTable {}

impl FluentLangTable {
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
    ) -> Result<(), AppErrors> {
        let (width, _) = terminal::size()?;

        let components: Vec<Box<dyn TableComponent>> = vec![
            Box::new(ArtComponent::new(width).get_component()?),
            Box::new(StatusComponent::new(width, provider.clone()).get_component()?),
            Box::new(ChatComponent::new(width, provider, sentence, answer).get_component()?),
        ];

        show_table(components)?;

        Ok(())
    }

    /// The function was created for building an output for showing the config data.
    /// The function builds a table from components, and after it renders the table.
    /// The table shows art component, and config status component.
    ///
    /// * `output_data` - the config data.
    pub fn print_config_data(output_data: OutputData) -> Result<(), AppErrors> {
        let (width, _) = terminal::size()?;

        let components: Vec<Box<dyn TableComponent>> = vec![
            Box::new(ArtComponent::new(width).get_component()?),
            Box::new(ConfigStatusComponent::new(width, output_data).get_component()?),
        ];

        show_table(components)?;

        Ok(())
    }
}

/// The function was created for linking components together.
/// The function builds component into [`Vec<String>`] to render a table.
///
/// * `components` - a list of components for building a table.
fn show_table(components: Vec<Box<dyn TableComponent>>) -> Result<(), AppErrors> {
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

    match components.len() {
        1 => {
            let res = components[0].to_solo_table();
            render_table(res, fg, bg);
        }
        2 => {
            for (index, component) in components.iter().enumerate() {
                let res = if index == 0 {
                    component.to_start_of_table()
                } else {
                    component.to_end_of_table()
                };

                render_table(res, fg, bg);
            }
        }
        _ => {
            for (index, component) in components.iter().enumerate() {
                let res = if index == 0 {
                    component.to_start_of_table()
                } else if index == components.len() - 1 {
                    component.to_end_of_table()
                } else {
                    component.to_middle_of_table()
                };

                render_table(res, fg, bg);
            }
        }
    }

    Ok(())
}

/// The function was created for printing `res` using `fg` and `bg`.
///
/// * `res` - a ready list for rendering table.
/// * `fg` - a color of the symbols.
/// * `bg` - a color of the background in the terminal.
fn render_table(res: Vec<String>, fg: Color, bg: Color) {
    for line in res {
        println!("{}", line.as_str().with(fg).on(bg));
    }
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