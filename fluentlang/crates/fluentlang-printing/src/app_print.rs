use crate::table_components::TableComponent;
use crate::table_components::art_component::ArtComponent;
use crate::table_components::chat_component::ChatComponent;
use crate::table_components::status_component::StatusComponent;
use crossterm::style::{Color, Stylize};
use crossterm::terminal;
use fluentlang_core::config_parsing::AvailableAiProviders;
use fluentlang_core::error::AppErrors;
use terminal_colorsaurus::{QueryOptions, color_palette};

pub fn print_status(
    provider: AvailableAiProviders,
    sentence: String,
    answer: String,
) -> Result<(), AppErrors> {
    let (width, _) = terminal::size()?;

    let components: Vec<Box<dyn TableComponent>> = vec![
        Box::new(ArtComponent::new(width).get_component()),
        Box::new(StatusComponent::new(width, provider.clone()).get_component()),
        Box::new(ChatComponent::new(width, provider, sentence, answer).get_component()),
    ];
    render_table(components)?;

    Ok(())
}

fn render_table(components: Vec<Box<dyn TableComponent>>) -> Result<(), AppErrors> {
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

    for (index, component) in components.iter().enumerate() {
        let res = if index == 0 {
            component.to_start_of_table()
        } else if index == components.len() - 1 {
            component.to_end_of_table()
        } else {
            component.to_middle_of_table()
        };

        for line in res {
            println!("{}", line.as_str().with(fg).on(bg));
        }
    }

    Ok(())
}
