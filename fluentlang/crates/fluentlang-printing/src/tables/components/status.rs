use termimad::MadSkin;
use crate::tables::components::{NecessaryDataTable, Paddings, TableComponent};
use fluentlang_utils::config_parsing::AvailableAiProviders;
use fluentlang_utils::error::AppErrors;

const PROVIDER_PREFIX: &str = "Active AI Provider: ";
const STATUS_TEXT: &str = "Status: Ready. You can read answer below";

pub struct StatusComponent {
    data: NecessaryDataTable,
    provider: AvailableAiProviders,
}

impl StatusComponent {

    pub fn new(
        terminal_width: u16,
        provider: AvailableAiProviders,
        padding: Paddings,
    ) -> Result<Self, AppErrors> {
        let inner_width = (terminal_width as usize).saturating_sub(2);

        let provider_text_width = format!(
            "{}{:?}",
            PROVIDER_PREFIX,
            provider
        )
            .chars()
            .count();

        let status_text_width = STATUS_TEXT.chars().count();

        let max_content_width = provider_text_width.max(status_text_width);

        let required_width = padding
            .left
            .saturating_add(max_content_width)
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
            provider,
        })
    }
}

impl TableComponent for StatusComponent {
    fn into_component(mut self, _: Option<MadSkin>) -> Result<Self, AppErrors> {
        let empty_line = format!("│{}│", " ".repeat(self.data.terminal_inner_width));

        for _ in 0..self.data.padding.top {
            self.data.component.push(empty_line.clone());
        }

        let info_text = format!("{}{:?}", PROVIDER_PREFIX, self.provider);
        let left_padding = " ".repeat(self.data.padding.left);
        let right_padding = " ".repeat(
            self.data.terminal_inner_width.saturating_sub(info_text.chars().count() + left_padding.len()),
        );
        let ready_line = format!("│{}{}{}│", left_padding, info_text, right_padding);
        self.data.component.push(ready_line);

        let info_text = STATUS_TEXT;
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


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn test_provider() -> AvailableAiProviders {
        AvailableAiProviders::GoogleAiStudio
    }

    fn create_status_component(
        terminal_width: u16,
        padding: Paddings,
    ) -> Result<StatusComponent, AppErrors> {
        StatusComponent::new(
            terminal_width,
            test_provider(),
            padding,
        )?
            .into_component(None)
    }

    fn assert_size_error(result: Result<StatusComponent, AppErrors>) {
        assert!(
            matches!(result, Err(AppErrors::SizeTerminalProblem())),
            "Expected SizeTerminalProblem"
        );
    }

    fn assert_line_widths(lines: &[String], terminal_width: u16) {
        for line in lines {
            assert_eq!(
                line.chars().count(),
                terminal_width as usize,
                "Incorrect line width: {line:?}"
            );
        }
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(30, false)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn status_component_creation(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_status_component(
            terminal_width,
            Paddings::default(),
        );

        if should_succeed {
            let component = result.unwrap();

            assert_eq!(component.data.component.len(), 2
                + component.data.padding.top
                + component.data.padding.bottom);
        } else {
            assert_size_error(result);
        }
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn status_component_works_solo_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_status_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_solo_table();

        let inner_width = terminal_width as usize - 2;

        let top_line_expected =
            format!("╭{}╮", "─".repeat(inner_width));

        let bottom_line_expected =
            format!("╰{}╯", "─".repeat(inner_width));

        assert_eq!(lines.first().unwrap(), &top_line_expected);
        assert_eq!(lines.last().unwrap(), &bottom_line_expected);

        assert_line_widths(&lines, terminal_width);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn status_component_works_start_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_status_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_start_of_table();

        let inner_width = terminal_width as usize - 2;

        let top_line_expected =
            format!("╭{}╮", "─".repeat(inner_width));

        let bottom_line_expected =
            format!("├{}┤", "─".repeat(inner_width));

        assert_eq!(lines.first().unwrap(), &top_line_expected);
        assert_eq!(lines.last().unwrap(), &bottom_line_expected);

        assert_line_widths(&lines, terminal_width);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn status_component_works_middle_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_status_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_middle_of_table();

        let inner_width = terminal_width as usize - 2;

        let bottom_line_expected =
            format!("├{}┤", "─".repeat(inner_width));

        assert_eq!(lines.last().unwrap(), &bottom_line_expected);

        assert_line_widths(&lines, terminal_width);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn status_component_works_end_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_status_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_end_of_table();

        let inner_width = terminal_width as usize - 2;

        let bottom_line_expected =
            format!("╰{}╯", "─".repeat(inner_width));

        assert_eq!(lines.last().unwrap(), &bottom_line_expected);

        assert_line_widths(&lines, terminal_width);
    }

    #[rstest]
    #[case::no_padding(120, 0, 0, true)]
    #[case::small_padding(120, 2, 2, true)]
    #[case::asymmetric_padding(120, 5, 10, true)]
    #[case::large_padding(120, 30, 30, true)]
    #[case::too_large_padding(80, 20, 20, false)]
    #[case::padding_exceeds_width(30, 20, 20, false)]
    fn status_component_horizontal_padding(
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

        let result = create_status_component(
            terminal_width,
            padding,
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_solo_table();

        assert_line_widths(&lines, terminal_width);

        let content_lines = &component.data.component[
            component.data.padding.top
                ..component.data.component.len()
                - component.data.padding.bottom
            ];

        for line in content_lines {
            let content = line
                .strip_prefix('│')
                .unwrap()
                .strip_suffix('│')
                .unwrap();

            assert!(
                content.starts_with(&" ".repeat(left)),
                "Incorrect left padding: {line:?}"
            );

            assert!(
                content.ends_with(&" ".repeat(right)),
                "Incorrect right padding: {line:?}"
            );
        }
    }

    #[rstest]
    #[case::no_padding(0, 0)]
    #[case::top_only(3, 0)]
    #[case::bottom_only(0, 3)]
    #[case::both(2, 4)]
    #[case::large_padding(10, 10)]
    fn status_component_vertical_padding(
        #[case] top: usize,
        #[case] bottom: usize,
    ) {
        let padding = Paddings {
            top,
            bottom,
            ..Paddings::default()
        };

        let component = create_status_component(
            120,
            padding,
        )
            .unwrap();

        let lines = component.to_solo_table();

        let empty_line = format!(
            "│{}│",
            " ".repeat(118)
        );

        let expected_count = 2 + top + 2 + bottom;

        assert_eq!(lines.len(), expected_count);

        for line in &lines[1..1 + top] {
            assert_eq!(line, &empty_line);
        }

        let bottom_start = lines.len() - 1 - bottom;

        for line in &lines[bottom_start..lines.len() - 1] {
            assert_eq!(line, &empty_line);
        }
    }

    #[test]
    fn status_component_renders_correct_text() {
        let component = create_status_component(
            120,
            Paddings::default(),
        )
            .unwrap();

        let rendered = component
            .data
            .component
            .join("\n");

        let expected_provider = format!(
            "{}{:?}",
            PROVIDER_PREFIX,
            test_provider()
        );

        assert!(rendered.contains(&expected_provider));
        assert!(rendered.contains(STATUS_TEXT));
    }
}