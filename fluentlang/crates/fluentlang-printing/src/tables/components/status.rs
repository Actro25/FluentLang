use crate::tables::components::{NecessaryDataTable, Paddings, TableComponent};
use fluentlang_utils::config_parsing::AvailableAiProviders;
use fluentlang_utils::error::AppErrors;

pub struct StatusComponent {
    data: NecessaryDataTable,
    provider: AvailableAiProviders,
}

impl StatusComponent {
    pub fn new(terminal_width: u16, provider: AvailableAiProviders, padding: Paddings) -> Self {
        Self {
            data: NecessaryDataTable {
                component: vec![],
                terminal_inner_width: (terminal_width as usize).saturating_sub(2),
                padding,
            },
            provider,
        }
    }
}

impl TableComponent for StatusComponent {
    fn into_component(mut self) -> Result<Self, AppErrors> {
        let empty_line = format!("│{}│", " ".repeat(self.data.terminal_inner_width));

        for _ in 0..self.data.padding.top {
            self.data.component.push(empty_line.clone());
        }

        let info_text = format!("Active AI Provider: {:?}", self.provider);
        let left_padding = " ".repeat(self.data.padding.left);
        let right_padding = " ".repeat(
            self.data.terminal_inner_width.saturating_sub(info_text.chars().count() + left_padding.len()),
        );
        let ready_line = format!("│{}{}{}│", left_padding, info_text, right_padding);
        self.data.component.push(ready_line);

        let info_text = "Status: Ready. You can read answer below";
        let left_padding = " ".repeat(self.data.padding.left);
        let right_padding = " ".repeat(
            self.data.terminal_inner_width.saturating_sub(info_text.chars().count() + left_padding.len()),
        );
        let ready_line = format!("│{}{}{}│", left_padding, info_text, right_padding);
        self.data.component.push(ready_line);

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