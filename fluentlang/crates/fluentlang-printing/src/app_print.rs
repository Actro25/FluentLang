use crate::art::ArtDraw;
use crossterm::style::{Color, Stylize};
use crossterm::terminal;
use fluentlang_core::config_parsing::AvailableAiProviders;
use fluentlang_core::error::AppErrors;
use std::fmt::format;
use terminal_colorsaurus::{QueryOptions, color_palette};

pub fn print_status(
    art: impl ArtDraw,
    provider: AvailableAiProviders,
    sentence: String,
    answer: String,
) -> Result<(), AppErrors> {
    let (width, _) = terminal::size()?;
    let inner_width = (width as usize).saturating_sub(2);
    let mut content = art.get_art(width);

    write_art_into_table(&mut content, inner_width);
    write_info_into_table(&mut content, inner_width, &provider);
    write_result_into_table(&mut content, inner_width, &provider, sentence, answer);
    let upper_str = format!("╭{}╮", "─".repeat((width - 2) as usize));
    let bottom_str = format!("╰{}╯", "─".repeat((width - 2) as usize));

    print_table(&upper_str, &bottom_str, content)?;

    Ok(())
}

fn print_table(
    upper_str: &String,
    bottom_str: &String,
    content: Vec<String>,
) -> Result<(), AppErrors> {
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

    println!("{}", upper_str.as_str().with(fg).on(bg));
    for line in &content {
        println!("{}", line.as_str().with(fg).on(bg));
    }
    println!("{}", bottom_str.as_str().with(fg).on(bg));

    Ok(())
}

fn write_info_into_table(
    content: &mut Vec<String>,
    inner_terminal_width: usize,
    provider: &AvailableAiProviders,
) {
    let separator = format!("├{}┤", "─".repeat(inner_terminal_width));
    content.push(separator);
    content.push(format!("│{}│", " ".repeat(inner_terminal_width)));

    let info_text = format!("Active AI Provider: {:?}", provider);
    let left_padding = format!("{}", " ".repeat(3));
    let right_padding = format!(
        "{}",
        " ".repeat(inner_terminal_width - info_text.chars().count() - left_padding.len())
    );
    let ready_line = format!("│{}{}{}│", left_padding, info_text, right_padding);
    content.push(ready_line);

    let info_text = "Status: Ready. You can read answer below";
    let left_padding = format!("{}", " ".repeat(3));
    let right_padding = format!(
        "{}",
        " ".repeat(inner_terminal_width - info_text.chars().count() - left_padding.len())
    );
    let ready_line = format!("│{}{}{}│", left_padding, info_text, right_padding);
    content.push(ready_line);

    content.push(format!("│{}│", " ".repeat(inner_terminal_width)));
}

const LEFT_PADDING: usize = 3;
const RIGHT_PADDING: usize = 3;
const BEFORE_USER_PADDING: usize = 1;

fn rebuild_into_lines(text: &str, available_width: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for paragraph in text.lines() {
        let mut temporary_storage = String::new();
        let mut current_width: usize = 0;

        for word in paragraph.split_whitespace() {
            // POSSIBLE PROBLEM: What if it's the first iteration and word has a len more than an available width?
            if current_width + word.chars().count() > available_width {
                result.push(temporary_storage.clone());
                temporary_storage = String::new();
                current_width = 0;
            }

            temporary_storage.push_str(format!("{} ", word).as_str());
            current_width += word.chars().count() + 1;
        }

        if current_width != 0 {
            result.push(temporary_storage.clone());
        }

        result.push(" ".repeat(available_width));
    }

    result
}

fn push_wrapped_lines(
    content: &mut Vec<String>,
    inner_terminal_width: usize,
    prefix: &str,
    text: &str,
) {
    let prefix_len = prefix.chars().count() + BEFORE_USER_PADDING;
    let available = inner_terminal_width
        .saturating_sub(LEFT_PADDING + RIGHT_PADDING + prefix_len)
        .max(1);

    let first_prefix = format!("{}{}", prefix, " ".repeat(BEFORE_USER_PADDING));
    let next_prefix = " ".repeat(prefix_len);

    for (index, line) in rebuild_into_lines(text, available).into_iter().enumerate() {
        let lead = if index == 0 {
            &first_prefix
        } else {
            &next_prefix
        };
        let ready_line = format!("{}{}", lead, line);

        let used = LEFT_PADDING + ready_line.chars().count();
        let filler = inner_terminal_width.saturating_sub(used);

        content.push(format!(
            "│{}{}{}│",
            " ".repeat(LEFT_PADDING),
            ready_line,
            " ".repeat(filler)
        ));
    }
}

fn write_result_into_table(
    content: &mut Vec<String>,
    inner_terminal_width: usize,
    provider: &AvailableAiProviders,
    sentence: String,
    answer: String,
) {
    let empty_line = format!("│{}│", " ".repeat(inner_terminal_width));

    content.push(format!("├{}┤", "─".repeat(inner_terminal_width)));
    content.push(empty_line.clone());

    push_wrapped_lines(
        content,
        inner_terminal_width,
        "User: ",
        &format!("{:?}", sentence),
    );

    content.push(empty_line.clone());

    let provider = format!("{:?}: ", provider);
    push_wrapped_lines(content, inner_terminal_width, &provider, &answer);

    content.push(empty_line);
}

fn write_art_into_table(current_art: &mut Vec<String>, inner_terminal_width: usize) {
    let art_width = current_art.iter().map(|l| l.chars().count()).max().unwrap();

    let total_pad = inner_terminal_width.saturating_sub(art_width);
    let left = total_pad / 2;

    for line in current_art.iter_mut() {
        let line_width = line.chars().count();
        let right = inner_terminal_width.saturating_sub(left + line_width);

        *line = format!("│{}{}{}│", " ".repeat(left), line, " ".repeat(right));
    }
}
