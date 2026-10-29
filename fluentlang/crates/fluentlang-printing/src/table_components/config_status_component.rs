use crate::table_components::{NecessaryDataTable, Paddings, TableComponent};
use fluentlang_core::config_parsing::OutputData;
use fluentlang_core::error::AppErrors;

pub struct ConfigStatusComponent {
    data: NecessaryDataTable,
    output_data: OutputData,
}

impl ConfigStatusComponent {
    pub fn new(terminal_width: u16, output_data: OutputData, padding: Paddings) -> Self {
        Self {
            data: NecessaryDataTable {
                component: vec![],
                terminal_inner_width: (terminal_width as usize).saturating_sub(2),
                padding,
            },
            output_data,
        }
    }

    fn write_provider(&mut self, header: &str, private: &str) {
        let hidden_key = format!("Private: {}", self.translate_into_hidden(private));

        let left_padding = self
            .data
            .terminal_inner_width
            .saturating_sub(header.chars().count())
            / 2;
        self.data.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left_padding),
            header,
            " ".repeat(
                self.data
                    .terminal_inner_width
                    .saturating_sub(header.chars().count() + left_padding)
            )
        ));

        let left_padding = self
            .data
            .terminal_inner_width
            .saturating_sub(hidden_key.chars().count())
            / 2;
        self.data.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left_padding),
            hidden_key,
            " ".repeat(
                self.data
                    .terminal_inner_width
                    .saturating_sub(hidden_key.chars().count() + left_padding)
            )
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
    fn get_component(mut self) -> Result<Self, AppErrors>
    where
        Self: Sized,
    {
        let empty_line = format!("│{}│", " ".repeat(self.data.terminal_inner_width));

        for _ in 0..self.data.padding.top {
            self.data.component.push(empty_line.clone());
        }

        let header = "=== Current Configuration ===";
        let left_padding = self
            .data
            .terminal_inner_width
            .saturating_sub(header.chars().count())
            / 2;
        self.data.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left_padding),
            header,
            " ".repeat(
                self.data
                    .terminal_inner_width
                    .saturating_sub(header.chars().count() + left_padding)
            )
        ));

        let cur_provider = format!(
            "Current Active Provider: {:?}",
            self.output_data.current_provider
        );
        let left_padding = self
            .data
            .terminal_inner_width
            .saturating_sub(cur_provider.chars().count())
            / 2;
        self.data.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left_padding),
            cur_provider,
            " ".repeat(
                self.data
                    .terminal_inner_width
                    .saturating_sub(cur_provider.chars().count() + left_padding)
            )
        ));

        self.write_provider(
            "--- Groq Cloud ---",
            &self.output_data.groq_cloud.private.clone(),
        );
        self.data
            .component
            .push(format!("│{}│", " ".repeat(self.data.terminal_inner_width)));

        self.write_provider(
            "--- Google AI Studio ---",
            &self.output_data.google_ai_studio.private.clone(),
        );
        self.data
            .component
            .push(format!("│{}│", " ".repeat(self.data.terminal_inner_width)));

        self.write_provider(
            "--- Open Router ---",
            &self.output_data.open_router.private.clone(),
        );
        
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