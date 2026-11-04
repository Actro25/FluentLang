use crate::tables::components::{NecessaryDataTable, Paddings, TableComponent};
use fluentlang_utils::error::AppErrors;
use std::sync::LazyLock;
use termimad::MadSkin;

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
    pub fn new(terminal_width: u16, padding: Paddings) -> Result<Self, AppErrors> {
        let inner_width = (terminal_width as usize).saturating_sub(2);

        let small_art_width = SMALL_ART
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        let required_width = small_art_width
            .saturating_add(padding.left)
            .saturating_add(padding.right);

        if inner_width < required_width {
            return Err(AppErrors::SizeTerminalProblem());
        }

        Ok(Self {
            data: NecessaryDataTable {
                component: vec![],
                terminal_inner_width: inner_width,
                padding,
            },
        })
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
    fn into_component(mut self, _mad_skin: Option<MadSkin>) -> Result<Self, AppErrors> {
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


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn create_art_component(
        terminal_width: u16,
        paddings: Paddings
    ) -> Result<ArtComponent, AppErrors> {
        ArtComponent::new(terminal_width, paddings)?
            .into_component(None)
    }

    fn assert_size_error(result: Result<ArtComponent, AppErrors>) {
        assert!(
            matches!(result, Err(AppErrors::SizeTerminalProblem())),
            "Expected SizeTerminalProblem"
        );
    }

    #[rstest]
    #[case::small_art(30, true)]
    #[case::big_art(120, true)]
    #[case::too_small(10, false)]
    #[case::zero_width(0, false)]
    fn art_component_creation(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_art_component(terminal_width, Paddings::default());

        if should_succeed {
            let art = result.unwrap();
            let lines = art.to_solo_table();

            assert!(!lines.is_empty());
        } else {
            assert_size_error(result);
        }
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(60, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn art_component_works_solo_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_art_component(terminal_width, Paddings::default());

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let art = result.unwrap();
        let lines = art.to_solo_table();

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
    #[case::small(60, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn art_component_works_end_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_art_component(terminal_width, Paddings::default());

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let art = result.unwrap();
        let lines = art.to_end_of_table();

        let inner_width = (terminal_width as usize).saturating_sub(2);

        let bottom_line_expected =
            format!("╰{}╯", "─".repeat(inner_width));

        assert_eq!(bottom_line_expected, lines[lines.len() - 1]);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(60, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn art_component_works_middle_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_art_component(terminal_width, Paddings::default());

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let art = result.unwrap();
        let lines = art.to_middle_of_table();

        let inner_width = (terminal_width as usize).saturating_sub(2);

        let bottom_line_expected =
            format!("├{}┤", "─".repeat(inner_width));

        assert_eq!(bottom_line_expected, lines[lines.len() - 1]);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(60, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn art_component_works_start_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_art_component(terminal_width, Paddings::default());

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let art = result.unwrap();
        let lines = art.to_start_of_table();

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
    fn art_component_horizontal_padding(
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

        let result = create_art_component(terminal_width, padding);

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let art = result.unwrap();
        let lines = art.to_solo_table();

        for line in &lines {
            assert_eq!(
                line.chars().count(),
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
    fn art_component_vertical_padding(
        #[case] top: usize,
        #[case] bottom: usize,
    ) {
        let padding = Paddings {
            top,
            bottom,
            ..Paddings::default()
        };

        let art = create_art_component(120, padding).unwrap();

        let lines = art.to_solo_table();
        let inner_width = 118;

        let empty_line = format!("│{}│", " ".repeat(inner_width));

        let art_height = art.get_art().len();
        let expected_count = 2 + top + art_height + bottom;

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