use crate::table_components::TableComponent;
use fluentlang_core::config_parsing::OutputData;

pub struct ConfigStatusComponent {
    component: Vec<String>,
    width: usize,
    output_data: OutputData,
}

impl ConfigStatusComponent {
    pub fn new(terminal_width: u16, output_data: OutputData) -> Self {
        Self {
            component: Vec::new(),
            width: (terminal_width as usize).saturating_sub(2),
            output_data,
        }
    }

    fn write_provider(&mut self, header: &str, private: &str) {
        let hidden_key = format!("Private: {}", self.translate_into_hidden(private));

        let left_padding = self.width.saturating_sub(header.chars().count()) / 2;
        self.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left_padding),
            header,
            " ".repeat(self.width.saturating_sub(header.chars().count() + left_padding))
        ));

        let left_padding = self.width.saturating_sub(hidden_key.chars().count()) / 2;
        self.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left_padding),
            hidden_key,
            " ".repeat(self.width.saturating_sub(hidden_key.chars().count() + left_padding))
        ));
    }

    fn translate_into_hidden(&self, key: &str) -> String {
        let percent_of_shown = 10;
        let to_show = key.len() / percent_of_shown;

        let mut result: Vec<char> = key.chars().collect();

        let mut index = to_show;
        while index < result.len() {
            result[index] = '*';
            index += 1;
        }

        result.into_iter().collect()
    }
}

impl TableComponent for ConfigStatusComponent {
    fn get_component(mut self) -> Self
    where
        Self: Sized,
    {
        self.component.push(format!("│{}│", " ".repeat(self.width)));

        let header = "=== Current Configuration ===";
        let left_padding = self.width.saturating_sub(header.chars().count()) / 2;
        self.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left_padding),
            header,
            " ".repeat(self.width.saturating_sub(header.chars().count() + left_padding))
        ));

        let cur_provider = format!(
            "Current Active Provider: {:?}",
            self.output_data.current_provider
        );
        let left_padding = self.width.saturating_sub(cur_provider.chars().count()) / 2;
        self.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left_padding),
            cur_provider,
            " ".repeat(self.width.saturating_sub(cur_provider.chars().count() + left_padding))
        ));

        self.write_provider(
            "--- Groq Cloud ---",
            &self.output_data.groq_cloud.private.clone(),
        );
        self.component.push(format!("│{}│", " ".repeat(self.width)));

        self.write_provider(
            "--- Google AI Studio ---",
            &self.output_data.google_ai_studio.private.clone(),
        );
        self.component.push(format!("│{}│", " ".repeat(self.width)));

        self.write_provider(
            "--- Open Router ---",
            &self.output_data.open_router.private.clone(),
        );
        self.component.push(format!("│{}│", " ".repeat(self.width)));

        self
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