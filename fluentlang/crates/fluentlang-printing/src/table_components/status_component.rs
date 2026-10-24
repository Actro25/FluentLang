use fluentlang_core::config_parsing::AvailableAiProviders;
use crate::table_components::TableComponent;

pub struct StatusComponent {
    component: Vec<String>,
    width: usize,
    provider: AvailableAiProviders
}

impl StatusComponent {
    pub fn new(terminal_width: u16, provider: AvailableAiProviders) -> Self {
        Self {
            component: Vec::new(),
            width: (terminal_width as usize).saturating_sub(2),
            provider
        }
    }
}

impl TableComponent for StatusComponent {
    fn get_component(mut self) -> Self {
        let info_text = format!("Active AI Provider: {:?}", self.provider);
        let left_padding = " ".repeat(3);
        let right_padding = " ".repeat(self.width - info_text.chars().count() - left_padding.len());
        let ready_line = format!("│{}{}{}│", left_padding, info_text, right_padding);
        self.component.push(ready_line);

        let info_text = "Status: Ready. You can read answer below";
        let left_padding = " ".repeat(3);
        let right_padding = " ".repeat(self.width - info_text.chars().count() - left_padding.len());
        let ready_line = format!("│{}{}{}│", left_padding, info_text, right_padding);
        self.component.push(ready_line);

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
