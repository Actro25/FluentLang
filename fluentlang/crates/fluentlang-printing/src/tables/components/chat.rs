use std::any::type_name_of_val;
use crate::tables::components::{NecessaryDataTable, Paddings, TableComponent};
use fluentlang_utils::config_parsing::AvailableAiProviders;
use fluentlang_utils::error::AppErrors;
use strip_ansi_escapes::strip;
use termimad::{FmtText, MadSkin};
use fluentlang_utils::error::AppErrors::TheComponentExpectsSkinButSkinNotFound;

const BEFORE_USER_PADDING: usize = 1;

pub struct ChatComponent {
    data: NecessaryDataTable,
    provider: AvailableAiProviders,
    sentence: String,
    answer: String,
}

impl ChatComponent {

    pub fn new(
        terminal_width: u16,
        provider: AvailableAiProviders,
        sentence: String,
        answer: String,
        padding: Paddings,
    ) -> Result<Self, AppErrors> {
        let inner_width = (terminal_width as usize).saturating_sub(2);

        let user_prefix_width = "User: ".chars().count()
            + BEFORE_USER_PADDING;

        let provider_prefix_width = format!("{provider:?}: ")
            .chars()
            .count()
            + BEFORE_USER_PADDING;

        let max_prefix_width = user_prefix_width.max(provider_prefix_width);

        let required_width = padding
            .left
            .saturating_add(padding.right)
            .saturating_add(max_prefix_width)
            .saturating_add(1);

        if inner_width < required_width {
            return Err(AppErrors::SizeTerminalProblem());
        }

        Ok(Self {
            data: NecessaryDataTable {
                component: vec![],
                terminal_inner_width: inner_width,
                padding,
            },
            provider,
            sentence,
            answer,
        })
    }


    fn push_wrapped_lines(&mut self, prefix: &str, text: &str, skin: &MadSkin) -> Result<(), AppErrors> {
        let prefix_len = prefix.chars().count() + BEFORE_USER_PADDING;
        let available = self
            .data
            .terminal_inner_width
            .saturating_sub(self.data.padding.left + self.data.padding.right + prefix_len)
            .max(1);

        let first_prefix = format!("{}{}", prefix, " ".repeat(BEFORE_USER_PADDING));
        let next_prefix = " ".repeat(prefix_len);

        let rendered_text = format!("{}", FmtText::from(skin, text, Some(available)));

        for (index, line) in rendered_text.lines().enumerate() {
            let lead = if index == 0 {
                &first_prefix
            } else {
                &next_prefix
            };

            let ready_line = format!("{}{}", lead, line);

            let stripped_bytes = strip(&ready_line);
            let clean_line = String::from_utf8_lossy(&stripped_bytes);
            let visible_len = clean_line.chars().count();

            let used = self.data.padding.left + visible_len;
            let filler = self.data.terminal_inner_width.saturating_sub(used);

            self.data.component.push(format!(
                "│{}{}{}│",
                " ".repeat(self.data.padding.left),
                ready_line,
                " ".repeat(filler)
            ));
        }

        Ok(())
    }
}

impl TableComponent for ChatComponent {
    fn into_component(mut self, mad_skin: Option<MadSkin>) -> Result<Self, AppErrors> {
        if mad_skin.is_none() {
            return Err(TheComponentExpectsSkinButSkinNotFound(type_name_of_val(&self).to_string()));
        }
        let skin = mad_skin.unwrap();

        let empty_line = format!("│{}│", " ".repeat(self.data.terminal_inner_width));

        for _ in 0..self.data.padding.top {
            self.data.component.push(empty_line.clone());
        }

        self.push_wrapped_lines("User: ", &self.sentence.to_string(), &skin)?;

        self.data.component.push(empty_line.clone());

        let provider = format!("{:?}: ", self.provider);
        self.push_wrapped_lines(&provider, self.answer.clone().as_str(), &skin)?;

        for _ in 0..self.data.padding.bottom {
            self.data.component.push(empty_line.clone());
        }

        Ok(self)
    }

    fn to_start_of_table(&self) -> Vec<String> {
        let mut result = self.data.component.clone();
        result.insert(
            0,
            format!("╭{}╮", "─".repeat(self.data.terminal_inner_width)),
        );
        result.push(format!("├{}┤", "─".repeat(self.data.terminal_inner_width)));
        result
    }

    fn to_middle_of_table(&self) -> Vec<String> {
        let mut result = self.data.component.clone();
        result.push(format!("├{}┤", "─".repeat(self.data.terminal_inner_width)));
        result
    }

    fn to_end_of_table(&self) -> Vec<String> {
        let mut result = self.data.component.clone();
        result.push(format!("╰{}╯", "─".repeat(self.data.terminal_inner_width)));
        result
    }

    fn to_solo_table(&self) -> Vec<String> {
        let mut result = self.data.component.clone();
        result.insert(
            0,
            format!("╭{}╮", "─".repeat(self.data.terminal_inner_width)),
        );
        result.push(format!("╰{}╯", "─".repeat(self.data.terminal_inner_width)));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn test_provider() -> AvailableAiProviders {
        AvailableAiProviders::GoogleAiStudio
    }

    fn create_chat_component(
        terminal_width: u16,
        padding: Paddings,
    ) -> Result<ChatComponent, AppErrors> {
        ChatComponent::new(
            terminal_width,
            test_provider(),
            "Hello, how are you?".to_string(),
            "I'm fine, thank you!".to_string(),
            padding,
        )?
            .into_component(Option::from(MadSkin::default()))
    }

    fn assert_size_error(result: Result<ChatComponent, AppErrors>) {
        assert!(
            matches!(result, Err(AppErrors::SizeTerminalProblem())),
            "Expected SizeTerminalProblem"
        );
    }

    #[rstest]
    #[case::small(30, true)]
    #[case::large(120, true)]
    #[case::too_small(10, false)]
    #[case::zero_width(0, false)]
    fn chat_component_creation(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_chat_component(
            terminal_width,
            Paddings::default(),
        );

        if should_succeed {
            let chat = result.unwrap();
            let lines = chat.to_solo_table();

            assert!(!lines.is_empty());
        } else {
            assert_size_error(result);
        }
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(30, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn chat_component_works_solo_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_chat_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let chat = result.unwrap();
        let lines = chat.to_solo_table();

        let inner_width = (terminal_width as usize).saturating_sub(2);

        let top_line_expected =
            format!("╭{}╮", "─".repeat(inner_width));

        let bottom_line_expected =
            format!("╰{}╯", "─".repeat(inner_width));

        assert_eq!(top_line_expected, lines[0]);
        assert_eq!(bottom_line_expected, lines[lines.len() - 1]);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(30, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn chat_component_works_end_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_chat_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let chat = result.unwrap();
        let lines = chat.to_end_of_table();

        let inner_width = (terminal_width as usize).saturating_sub(2);

        let bottom_line_expected =
            format!("╰{}╯", "─".repeat(inner_width));

        assert_eq!(bottom_line_expected, lines[lines.len() - 1]);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(30, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn chat_component_works_middle_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_chat_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let chat = result.unwrap();
        let lines = chat.to_middle_of_table();

        let inner_width = (terminal_width as usize).saturating_sub(2);

        let bottom_line_expected =
            format!("├{}┤", "─".repeat(inner_width));

        assert_eq!(bottom_line_expected, lines[lines.len() - 1]);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(30, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn chat_component_works_start_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_chat_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let chat = result.unwrap();
        let lines = chat.to_start_of_table();

        let inner_width = (terminal_width as usize).saturating_sub(2);

        let top_line_expected =
            format!("╭{}╮", "─".repeat(inner_width));

        let bottom_line_expected =
            format!("├{}┤", "─".repeat(inner_width));

        assert_eq!(top_line_expected, lines[0]);
        assert_eq!(bottom_line_expected, lines[lines.len() - 1]);
    }

    #[rstest]
    #[case::no_padding(120, 0, 0, true)]
    #[case::small_padding(120, 2, 2, true)]
    #[case::asymmetric_padding(120, 5, 10, true)]
    #[case::large_padding(120, 40, 40, true)]
    #[case::too_large_padding(30, 10, 10, false)]
    #[case::padding_exceeds_width(30, 20, 20, false)]
    fn chat_component_horizontal_padding(
        #[case] terminal_width: u16,
        #[case] left: usize,
        #[case] right: usize,
        #[case] should_succeed: bool,
    ) {
        let padding = Paddings {
            left,
            right,
            ..Paddings::default()
        };

        let result = create_chat_component(terminal_width, padding);

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let chat = result.unwrap();
        let lines = chat.to_solo_table();

        for line in &lines {
            let stripped = strip(line);
            let clean_line = String::from_utf8_lossy(&stripped);

            assert_eq!(
                clean_line.chars().count(),
                terminal_width as usize,
                "Incorrect line width: left={left}, right={right}"
            );
        }

        assert!(!lines.is_empty());
    }

    #[rstest]
    #[case::no_padding(0, 0)]
    #[case::top_only(3, 0)]
    #[case::bottom_only(0, 3)]
    #[case::both(2, 4)]
    #[case::large_padding(10, 10)]
    fn chat_component_vertical_padding(
        #[case] top: usize,
        #[case] bottom: usize,
    ) {
        let padding = Paddings {
            top,
            bottom,
            ..Paddings::default()
        };

        let chat = create_chat_component(120, padding).unwrap();

        let lines = chat.to_solo_table();
        let inner_width = 118;

        let empty_line = format!("│{}│", " ".repeat(inner_width));

        let content_height = lines.len() - 2 - top - bottom;
        let expected_count = 2 + top + content_height + bottom;

        assert_eq!(lines.len(), expected_count);

        for line in &lines[1..1 + top] {
            assert_eq!(line, &empty_line);
        }

        let bottom_start = lines.len() - 1 - bottom;

        for line in &lines[bottom_start..lines.len() - 1] {
            assert_eq!(line, &empty_line);
        }
    }
}
