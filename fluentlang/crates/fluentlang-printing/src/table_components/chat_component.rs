use crate::app_print::get_skin;
use crate::table_components::{NecessaryDataTable, Paddings, TableComponent};
use fluentlang_tools::config_parsing::AvailableAiProviders;
use fluentlang_tools::error::AppErrors;
use strip_ansi_escapes::strip;
use termimad::FmtText;

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
        padding: Paddings
    ) -> Self {
        Self {
            data: NecessaryDataTable {
                component: vec![],
                terminal_inner_width: (terminal_width as usize).saturating_sub(2),
                padding,
            },
            provider,
            sentence,
            answer,
        }
    }

    fn push_wrapped_lines(&mut self, prefix: &str, text: &str) -> Result<(), AppErrors> {
        let prefix_len = prefix.chars().count() + BEFORE_USER_PADDING;
        let available = self
            .data
            .terminal_inner_width
            .saturating_sub(self.data.padding.left + self.data.padding.right + prefix_len)
            .max(1);

        let first_prefix = format!("{}{}", prefix, " ".repeat(BEFORE_USER_PADDING));
        let next_prefix = " ".repeat(prefix_len);

        let skin = get_skin()?;

        let rendered_text = format!("{}", FmtText::from(&skin, text, Some(available)));

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
    fn get_component(mut self) -> Result<Self, AppErrors> {
        let empty_line = format!("│{}│", " ".repeat(self.data.terminal_inner_width));

        for _ in 0..self.data.padding.top {
            self.data.component.push(empty_line.clone());
        }

        self.push_wrapped_lines("User: ", &self.sentence.to_string())?;

        self.data.component.push(empty_line.clone());

        let provider = format!("{:?}: ", self.provider);
        self.push_wrapped_lines(&provider, self.answer.clone().as_str())?;

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