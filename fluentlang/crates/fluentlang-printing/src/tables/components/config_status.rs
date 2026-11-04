use crate::tables::components::{NecessaryDataTable, Paddings, TableComponent};
use fluentlang_utils::config_parsing::OutputData;
use fluentlang_utils::error::AppErrors;
use termimad::MadSkin;

const HEADERS: [&str; 4] = [
    "=== Current Configuration ===",
    "--- Groq Cloud ---",
    "--- Google AI Studio ---",
    "--- Open Router ---",
];

const CURRENT_PROVIDER_PREFIX: &str = "Current Active Provider: ";
const PRIVATE_PREFIX: &str = "Private: ";

const PERCENT_OF_SHOWN: usize = 10;

pub struct ConfigStatusComponent {
    data: NecessaryDataTable,
    output_data: OutputData,
}

impl ConfigStatusComponent {
    pub fn new(
        terminal_width: u16,
        output_data: OutputData,
        padding: Paddings,
    ) -> Result<Self, AppErrors> {
        let inner_width = (terminal_width as usize).saturating_sub(2);

        let max_header_width = HEADERS
            .iter()
            .map(|header| header.chars().count())
            .max()
            .unwrap_or(0);

        let current_provider_width = CURRENT_PROVIDER_PREFIX.chars().count()
            + format!("{:?}", output_data.current_provider)
                .chars()
                .count();

        let max_private_width = [
            &output_data.groq_cloud.private,
            &output_data.google_ai_studio.private,
            &output_data.open_router.private,
        ]
        .iter()
        .map(|key| PRIVATE_PREFIX.chars().count() + key.chars().count())
        .max()
        .unwrap_or(0);

        let max_content_width = max_header_width
            .max(current_provider_width)
            .max(max_private_width);

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
            output_data,
        })
    }

    fn push_center_line(&mut self, text: &str) {
        let inner_width = self.data.terminal_inner_width;

        let available_width = inner_width
            .saturating_sub(self.data.padding.left)
            .saturating_sub(self.data.padding.right);

        let text_width = text.chars().count();

        let extra_space = available_width.saturating_sub(text_width);

        let left = self.data.padding.left + extra_space / 2;
        let right = inner_width.saturating_sub(left + text_width);

        self.data.component.push(format!(
            "│{}{}{}│",
            " ".repeat(left),
            text,
            " ".repeat(right)
        ));
    }

    fn write_provider(&mut self, header: &str, private: &str) {
        let hidden_key = format!("{}{}", PRIVATE_PREFIX, self.translate_into_hidden(private));
        self.push_center_line(header);
        self.push_center_line(&hidden_key);
    }

    fn translate_into_hidden(&self, key: &str) -> String {
        let mut result: Vec<char> = key.chars().collect();
        let to_show = result.len() / PERCENT_OF_SHOWN;
        for character in result.iter_mut().skip(to_show) {
            *character = '*';
        }

        result.into_iter().collect()
    }
}

impl TableComponent for ConfigStatusComponent {
    fn into_component(mut self, _mad_skin: Option<MadSkin>) -> Result<Self, AppErrors>
    where
        Self: Sized,
    {
        let empty_line = format!("│{}│", " ".repeat(self.data.terminal_inner_width));

        for _ in 0..self.data.padding.top {
            self.data.component.push(empty_line.clone());
        }

        self.push_center_line(HEADERS[0]);

        let current_provider = format!(
            "{}{:?}",
            CURRENT_PROVIDER_PREFIX, self.output_data.current_provider
        );
        self.push_center_line(&current_provider);

        let groq_private = self.output_data.groq_cloud.private.clone();
        self.write_provider(HEADERS[1], &groq_private);
        self.data.component.push(empty_line.clone());

        let google_private = self.output_data.google_ai_studio.private.clone();
        self.write_provider(HEADERS[2], &google_private);
        self.data.component.push(empty_line.clone());

        let open_router_private = self.output_data.open_router.private.clone();
        self.write_provider(HEADERS[3], &open_router_private);
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
mod test {
    use super::*;
    use rstest::rstest;

    fn test_output_data() -> OutputData {
        OutputData::default()
    }

    fn create_config_status_component(
        terminal_width: u16,
        padding: Paddings,
    ) -> Result<ConfigStatusComponent, AppErrors> {
        ConfigStatusComponent::new(
            terminal_width,
            test_output_data(),
            padding,
        )?
            .into_component(None)
    }

    fn assert_size_error(result: Result<ConfigStatusComponent, AppErrors>) {
        assert!(
            matches!(result, Err(AppErrors::SizeTerminalProblem())),
            "Expected SizeTerminalProblem"
        );
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(30, false)]
    #[case::invalid(10, false)]
    #[case::zero_width(0, false)]
    fn config_status_component_creation(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_config_status_component(
            terminal_width,
            Paddings::default(),
        );

        if should_succeed {
            let component = result.unwrap();
            let lines = component.to_solo_table();

            assert!(!lines.is_empty());
        } else {
            assert_size_error(result);
        }
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn config_status_component_works_solo_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_config_status_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_solo_table();

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
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn config_status_component_works_end_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_config_status_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_end_of_table();

        let inner_width = (terminal_width as usize).saturating_sub(2);

        let bottom_line_expected =
            format!("╰{}╯", "─".repeat(inner_width));

        assert_eq!(bottom_line_expected, lines[lines.len() - 1]);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn config_status_component_works_middle_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_config_status_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_middle_of_table();

        let inner_width = (terminal_width as usize).saturating_sub(2);

        let bottom_line_expected =
            format!("├{}┤", "─".repeat(inner_width));

        assert_eq!(bottom_line_expected, lines[lines.len() - 1]);
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::invalid(10, false)]
    #[case::zero(0, false)]
    fn config_status_component_works_start_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_config_status_component(
            terminal_width,
            Paddings::default(),
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_start_of_table();

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
    #[case::large_padding(120, 20, 20, true)]
    #[case::too_large_padding(50, 10, 10, false)]
    #[case::padding_exceeds_width(30, 20, 20, false)]
    fn config_status_component_horizontal_padding(
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

        let result = create_config_status_component(
            terminal_width,
            padding,
        );

        if !should_succeed {
            assert_size_error(result);
            return;
        }

        let component = result.unwrap();
        let lines = component.to_solo_table();

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
    fn config_status_component_vertical_padding(
        #[case] top: usize,
        #[case] bottom: usize,
    ) {
        let padding = Paddings {
            top,
            bottom,
            ..Paddings::default()
        };

        let component = create_config_status_component(
            120,
            padding,
        )
            .unwrap();

        let lines = component.to_solo_table();
        let inner_width = 118;

        let empty_line = format!(
            "│{}│",
            " ".repeat(inner_width)
        );

        let content_height = 10;
        let expected_count = 2 + top + content_height + bottom;

        assert_eq!(lines.len(), expected_count);

        for line in &lines[1..1 + top] {
            assert_eq!(line, &empty_line);
        }

        let bottom_start = lines.len() - 1 - bottom;

        for line in &lines[bottom_start..lines.len() - 1] {
            assert_eq!(line, &empty_line);
        }
    }

    #[rstest]
    #[case::empty("", "")]
    #[case::short("abc", "***")]
    #[case::ten_chars("1234567890", "1*********")]
    #[case::twenty_chars("12345678901234567890", "12******************")]
    fn config_status_component_hides_private_key(
        #[case] private_key: &str,
        #[case] expected: &str,
    ) {
        let component = ConfigStatusComponent::new(
            120,
            test_output_data(),
            Paddings::default(),
        )
            .unwrap();

        let hidden = component.translate_into_hidden(private_key);

        assert_eq!(hidden, expected);
    }
}