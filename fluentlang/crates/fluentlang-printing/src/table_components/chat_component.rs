use fluentlang_core::config_parsing::AvailableAiProviders;
use crate::table_components::TableComponent;

const LEFT_PADDING: usize = 3;
const RIGHT_PADDING: usize = 3;
const BEFORE_USER_PADDING: usize = 1;

pub struct ChatComponent {
    component: Vec<String>,
    width: usize,
    provider: AvailableAiProviders,
    sentence: String,
    answer: String
}

impl ChatComponent {
    pub fn new(terminal_width: u16, provider: AvailableAiProviders, sentence: String, answer:String) -> Self {
        Self {
            component: Vec::new(),
            width: (terminal_width as usize).saturating_sub(2),
            provider,
            sentence,
            answer
        }
    }

    fn push_wrapped_lines(
        &mut self,
        prefix: &str,
        text: &str,
    ) {
        let prefix_len = prefix.chars().count() + BEFORE_USER_PADDING;
        let available = self.width
            .saturating_sub(LEFT_PADDING + RIGHT_PADDING + prefix_len)
            .max(1);

        let first_prefix = format!("{}{}", prefix, " ".repeat(BEFORE_USER_PADDING));
        let next_prefix = " ".repeat(prefix_len);

        for (index, line) in self.rebuild_into_lines(text, available).into_iter().enumerate() {
            let lead = if index == 0 {
                &first_prefix
            } else {
                &next_prefix
            };
            let ready_line = format!("{}{}", lead, line);

            let used = LEFT_PADDING + ready_line.chars().count();
            let filler = self.width.saturating_sub(used);

            self.component.push(format!(
                "│{}{}{}│",
                " ".repeat(LEFT_PADDING),
                ready_line,
                " ".repeat(filler)
            ));
        }
    }

    fn rebuild_into_lines(&self,text: &str, available_width: usize) -> Vec<String> {
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
}

impl TableComponent for ChatComponent {
    fn get_component(mut self) -> Self {
        let empty_line = format!("│{}│", " ".repeat(self.width));

        self.component.push(empty_line.clone());

        self.push_wrapped_lines(
            "User: ",
            &format!("{:?}", self.sentence),
        );

        self.component.push(empty_line.clone());

        let provider = format!("{:?}: ", self.provider);
        self.push_wrapped_lines(&provider, self.answer.clone().as_str());

        self.component.push(empty_line);

        self
    }

    fn to_start_of_table(&self) -> Vec<String> {
        let mut result = self.component.clone();
        result.insert(0, format!("╭{}╮", "─".repeat(self.width)));
        result
    }

    fn to_middle_of_table(&self) -> Vec<String> {
        let mut result = self.component.clone();
        result.insert(0, format!("├{}┤", "─".repeat(self.width)));
        result.push(format!("├{}┤", "─".repeat(self.width)));
        result
    }

    fn to_end_of_table(&self) -> Vec<String> {
        let mut result = self.component.clone();
        result.push(format!("╰{}╯", "─".repeat(self.width)));
        result
    }
}
