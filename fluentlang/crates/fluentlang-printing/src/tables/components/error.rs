
use crate::tables::components::{
    NecessaryDataTable,
    Paddings,
    TableComponent,
};
use fluentlang_utils::error::AppErrors;
use std::sync::LazyLock;
use termimad::{FmtText, MadSkin};

const ICON_SPACING: usize = 3;

static ERROR_ART: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| {
        vec![
            "╭───────╮",
            "│   !   │",
            "│   !   │",
            "╰───────╯",
        ]
    });

pub struct ErrorComponent {
    data: NecessaryDataTable,
    error: String,
}

impl ErrorComponent {
    pub fn new(
        terminal_width: u16,
        error: String,
        padding: Paddings,
    ) -> Self {
        Self {
            data: NecessaryDataTable {
                component: vec![],
                terminal_inner_width: (terminal_width as usize)
                    .saturating_sub(2),
                padding,
            },
            error,
        }
    }

    fn push_line(&mut self, content: &str) {
        let width = self.data.terminal_inner_width;
        let left = self.data.padding.left.min(width);

        let visible = content.chars().count();
        let right = width.saturating_sub(left + visible);

        self.data.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left),
            content,
            " ".repeat(right),
        ));
    }

    fn push_empty_line(&mut self) {
        self.data.component.push(format!(
            "│{}│",
            " ".repeat(self.data.terminal_inner_width)
        ));
    }

    fn render_error(&mut self) {
        let available_width = self
            .data
            .terminal_inner_width
            .saturating_sub(
                self.data.padding.left + self.data.padding.right
            );

        let icon_width = ERROR_ART
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        let use_full_icon =
            available_width >= icon_width + ICON_SPACING + 10;

        let icon: Vec<&str> = if use_full_icon {
            ERROR_ART.iter().copied().collect()
        } else {
            vec!["!"]
        };

        let actual_icon_width = icon
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        let spacing = if use_full_icon {
            ICON_SPACING
        } else {
            1
        };

        let text_width = available_width
            .saturating_sub(actual_icon_width + spacing)
            .max(1);

        // FmtText використовується для перенесення тексту.
        let skin = MadSkin::no_style();

        let message = format!("ERROR\n{}", self.error);

        let rendered = format!(
            "{}",
            FmtText::from(&skin, &message, Some(text_width))
        );

        let text_lines: Vec<&str> = rendered.lines().collect();

        let total_lines = icon.len().max(text_lines.len());

        for index in 0..total_lines {
            let art_line = icon.get(index).copied().unwrap_or("");
            let text_line = text_lines.get(index).copied().unwrap_or("");

            let art_padding = actual_icon_width
                .saturating_sub(art_line.chars().count());

            let line = format!(
                "{}{}{}{}",
                art_line,
                " ".repeat(art_padding),
                " ".repeat(spacing),
                text_line,
            );

            self.push_line(&line);
        }
    }

    fn horizontal_border(&self, left: &str, right: &str) -> String {
        format!(
            "{}{}{}",
            left,
            "─".repeat(self.data.terminal_inner_width),
            right
        )
    }
}

impl TableComponent for ErrorComponent {
    fn into_component(mut self) -> Result<Self, AppErrors> {
        for _ in 0..self.data.padding.top {
            self.push_empty_line();
        }

        self.render_error();

        for _ in 0..self.data.padding.bottom {
            self.push_empty_line();
        }

        Ok(self)
    }

    fn to_start_of_table(&self) -> Vec<String> {
        let mut result = self.data.component.clone();

        result.insert(0, self.horizontal_border("╭", "╮"));
        result.push(self.horizontal_border("├", "┤"));

        result
    }

    fn to_middle_of_table(&self) -> Vec<String> {
        let mut result = self.data.component.clone();
        result.push(self.horizontal_border("├", "┤"));
        result
    }

    fn to_end_of_table(&self) -> Vec<String> {
        let mut result = self.data.component.clone();
        result.push(self.horizontal_border("╰", "╯"));
        result
    }

    fn to_solo_table(&self) -> Vec<String> {
        let mut result = self.data.component.clone();

        result.insert(0, self.horizontal_border("╭", "╮"));
        result.push(self.horizontal_border("╰", "╯"));

        result
    }
}