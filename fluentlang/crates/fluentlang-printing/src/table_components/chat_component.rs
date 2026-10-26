use strip_ansi_escapes::strip;
use termimad::FmtText;
use crate::table_components::TableComponent;
use fluentlang_core::config_parsing::AvailableAiProviders;
use fluentlang_core::error::AppErrors;
use crate::app_print::get_skin;

const LEFT_PADDING: usize = 3;
const RIGHT_PADDING: usize = 3;
const BEFORE_USER_PADDING: usize = 1;

pub struct ChatComponent {
    component: Vec<String>,
    width: usize,
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
    ) -> Self {
        Self {
            component: Vec::new(),
            width: (terminal_width as usize).saturating_sub(2),
            provider,
            sentence,
            answer,
        }
    }

    fn push_wrapped_lines(&mut self, prefix: &str, text: &str) -> Result<(), AppErrors> {
        let prefix_len = prefix.chars().count() + BEFORE_USER_PADDING;
        let available = self
            .width
            .saturating_sub(LEFT_PADDING + RIGHT_PADDING + prefix_len)
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

            let used = LEFT_PADDING + visible_len;
            let filler = self.width.saturating_sub(used);

            self.component.push(format!(
                "│{}{}{}│",
                " ".repeat(LEFT_PADDING),
                ready_line,
                " ".repeat(filler)
            ));
        }

        Ok(())
    }
}

impl TableComponent for ChatComponent {
    fn get_component(mut self) -> Result<Self, AppErrors> {
        let empty_line = format!("│{}│", " ".repeat(self.width));

        self.component.push(empty_line.clone());

        self.push_wrapped_lines("User: ", &self.sentence.to_string())?;

        self.component.push(empty_line.clone());

        let provider = format!("{:?}: ", self.provider);
        self.push_wrapped_lines(&provider, self.answer.clone().as_str())?;

        self.component.push(empty_line);

        Ok(self)
    }

    fn to_start_of_table(&self) -> Vec<String> {
        let mut result = self.component.clone();
        result.insert(0, format!("╭{}╮", "─".repeat(self.width)));
        result.push(format!("├{}┤", "─".repeat(self.width)));
        result
    }

    fn to_middle_of_table(&self) -> Vec<String> {
        let mut result = self.component.clone();
        result.push(format!("├{}┤", "─".repeat(self.width)));
        result
    }

    fn to_end_of_table(&self) -> Vec<String> {
        let mut result = self.component.clone();
        result.push(format!("╰{}╯", "─".repeat(self.width)));
        result
    }

    fn to_solo_table(&self) -> Vec<String> {
        let mut result = self.component.clone();
        result.insert(0, format!("╭{}╮", "─".repeat(self.width)));
        result.push(format!("╰{}╯", "─".repeat(self.width)));
        result
    }
}