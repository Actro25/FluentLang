use crate::tables::components::{NecessaryDataTable, Paddings, TableComponent};
use fluentlang_utils::error::AppErrors;
use std::sync::LazyLock;

static FULL_ART: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        r"______  _                  _        _".to_string(),
        r"| ____|| |_   _  ___ _ __ | |_     | |    __ _ _ __   __ _".to_string(),
        r"| |_   | | | | |/ _ \ '_ \| __|____| |   / _` | '_ \ / _` |".to_string(),
        r"|  _|  | | |_| |  __/ | | | ||_____| |__| (_| | | | | (_| |".to_string(),
        r"|_|    |_|\__,_|\___|_| |_|\__|    |_____\__,_|_| |_|\__, |".to_string(),
        r"                                                     |___/".to_string(),
    ]
});
static SMALL_ART: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        r"    ______ _".to_string(),
        r"   | ____|| | ".to_string(),
        r"  | |_   | |".to_string(),
        r" |  _|  | |___".to_string(),
        r"|_|    |_____|".to_string(),
    ]
});

pub struct ArtComponent {
    data: NecessaryDataTable,
}

impl ArtComponent {
    pub fn new(terminal_width: u16, padding: Paddings) -> Self {
        Self {
            data: NecessaryDataTable {
                component: vec![],
                terminal_inner_width: (terminal_width as usize).saturating_sub(2),
                padding,
            },
        }
    }

    fn get_art(&self) -> Vec<String> {
        let full_art_max_width = FULL_ART.iter().map(|l| l.chars().count()).max().unwrap();

        if (self
            .data
            .terminal_inner_width
            .saturating_sub(self.data.padding.right + self.data.padding.left)
            / full_art_max_width) as f64
            > 1.5
        {
            FULL_ART.clone()
        } else {
            SMALL_ART.clone()
        }
    }
}

impl TableComponent for ArtComponent {
    fn into_component(mut self) -> Result<Self, AppErrors> {
        let current_art = self.get_art();

        let art_width = current_art.iter().map(|l| l.chars().count()).max().unwrap();

        let available_width = self
            .data
            .terminal_inner_width
            .saturating_sub(self.data.padding.left + self.data.padding.right);

        let total_pad = available_width.saturating_sub(art_width);
        let left = self.data.padding.left + (total_pad / 2);

        let mut result = Vec::<String>::new();

        for _ in 0..self.data.padding.top {
            result.push(format!("│{}│", " ".repeat(self.data.terminal_inner_width)));
        }

        for line in &current_art {
            let line_width = line.chars().count();
            let right = self
                .data
                .terminal_inner_width
                .saturating_sub(left + line_width);

            result.push(format!(
                "│{}{}{}│",
                " ".repeat(left),
                line,
                " ".repeat(right)
            ));
        }

        for _ in 0..self.data.padding.bottom {
            result.push(format!("│{}│", " ".repeat(self.data.terminal_inner_width)));
        }

        self.data.component = result;

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
