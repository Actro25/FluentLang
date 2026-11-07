use std::any::type_name_of_val;
use crate::tables::components::{
    NecessaryDataTable,
    Paddings,
    TableComponent,
};
use fluentlang_utils::error::AppErrors;
use std::sync::LazyLock;
use strip_ansi_escapes::strip;
use termimad::{FmtText, MadSkin};
use fluentlang_utils::error::AppErrors::TheComponentExpectsSkinButSkinNotFound;

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
    ) -> Result<Self, AppErrors> {
        let inner_width = (terminal_width as usize).saturating_sub(2);

        let compact_icon_width = 1;
        let compact_icon_spacing = 1;
        let min_text_width = 1;

        let required_width = padding
            .left
            .saturating_add(padding.right)
            .saturating_add(compact_icon_width)
            .saturating_add(compact_icon_spacing)
            .saturating_add(min_text_width);

        if inner_width < required_width {
            return Err(AppErrors::SizeTerminalProblem());
        }

        Ok(Self {
            data: NecessaryDataTable {
                component: vec![],
                terminal_inner_width: inner_width,
                padding,
            },
            error,
        })
    }

    fn push_line(&mut self, content: &str) {
        let width = self.data.terminal_inner_width;
        let left = self.data.padding.left;

        let stripped = strip(content);
        let clean_content = String::from_utf8_lossy(&stripped);
        let visible = clean_content.chars().count();
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

    fn render_error(&mut self, skin: &MadSkin) {
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

        let message = format!("ERROR\n{}", self.error);

        let rendered = format!(
            "{}",
            FmtText::from(skin, &message, Some(text_width))
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
    fn into_component(mut self, mad_skin: Option<MadSkin>) -> Result<Self, AppErrors> {
        if mad_skin.is_none() {
            return Err(TheComponentExpectsSkinButSkinNotFound(type_name_of_val(&self).to_string()));
        }
        let skin = mad_skin.unwrap();

        for _ in 0..self.data.padding.top {
            self.push_empty_line();
        }

        self.render_error(&skin);

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


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_ERROR: &str = "Something went wrong";

    fn create_error_component(
        terminal_width: u16,
        padding: Paddings,
    ) -> Result<ErrorComponent, AppErrors> {
        ErrorComponent::new(
            terminal_width,
            TEST_ERROR.to_string(),
            padding,
        )?
            .into_component(Some(MadSkin::default()))
    }

    fn assert_size_error(result: Result<ErrorComponent, AppErrors>) {
        assert!(
            matches!(result, Err(AppErrors::SizeTerminalProblem())),
            "Expected SizeTerminalProblem"
        );
    }

    fn visible_line(line: &str) -> String {
        String::from_utf8(strip(line)).unwrap()
    }

    fn assert_line_widths(lines: &[String], terminal_width: u16) {
        for line in lines {
            let clean_line = visible_line(line);

            assert_eq!(
                clean_line.chars().count(),
                terminal_width as usize,
                "Incorrect line width: {clean_line:?}"
            );
        }
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(30, true)]
    #[case::invalid(4, false)]
    #[case::zero(0, false)]
    fn error_component_creation(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_error_component(
            terminal_width,
            Paddings::default(),
        );

        if should_succeed {
            let component = result.unwrap();

            assert!(!component.data.component.is_empty());
        } else {
            assert_size_error(result);
        }
    }

    #[rstest]
    #[case::large(120, true)]
    #[case::medium(80, true)]
    #[case::small(30, true)]
    #[case::invalid(4, false)]
    #[case::zero(0, false)]
    fn error_component_works_solo_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_error_component(
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
    #[case::small(30, true)]
    #[case::invalid(4, false)]
    fn error_component_works_start_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_error_component(
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
    #[case::small(30, true)]
    #[case::invalid(4, false)]
    fn error_component_works_middle_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_error_component(
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
    #[case::small(30, true)]
    #[case::invalid(4, false)]
    fn error_component_works_end_of_table(
        #[case] terminal_width: u16,
        #[case] should_succeed: bool,
    ) {
        let result = create_error_component(
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
    #[case::large_padding(120, 40, 40, true)]
    #[case::compact_with_padding(30, 10, 10, true)]
    #[case::too_large_padding(30, 20, 20, false)]
    #[case::invalid_padding(10, 5, 5, false)]
    fn error_component_horizontal_padding(
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

        let result = create_error_component(
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

        for line in &lines[1..lines.len() - 1] {
            let clean_line = visible_line(line);
            let content = clean_line
                .strip_prefix('│')
                .unwrap()
                .strip_suffix('│')
                .unwrap();

            assert!(
                content.starts_with(&" ".repeat(left)),
                "Left padding is incorrect: {clean_line:?}"
            );

            assert!(
                content.ends_with(&" ".repeat(right)),
                "Right padding is incorrect: {clean_line:?}"
            );
        }
    }

    #[rstest]
    #[case::no_padding(0, 0)]
    #[case::top_only(3, 0)]
    #[case::bottom_only(0, 3)]
    #[case::both(2, 4)]
    #[case::large_padding(10, 10)]
    fn error_component_vertical_padding(
        #[case] top: usize,
        #[case] bottom: usize,
    ) {
        let padding = Paddings {
            top,
            bottom,
            ..Paddings::default()
        };

        let component = create_error_component(
            120,
            padding,
        )
            .unwrap();

        let lines = component.to_solo_table();

        let empty_line = format!(
            "│{}│",
            " ".repeat(118)
        );

        let content_height = component.data.component.len()
            - top
            - bottom;

        assert!(content_height > 0);

        assert_eq!(
            lines.len(),
            2 + top + content_height + bottom
        );

        for line in &lines[1..1 + top] {
            assert_eq!(line, &empty_line);
        }

        let bottom_start = lines.len() - 1 - bottom;

        for line in &lines[bottom_start..lines.len() - 1] {
            assert_eq!(line, &empty_line);
        }
    }

    #[test]
    fn error_component_wraps_long_message() {
        let text = "This is a very long error message that should \
                     be wrapped into multiple lines when the terminal \
                     width is not large enough to display everything.";

        let component = ErrorComponent::new(
            30,
            text.to_string(),
            Paddings::default(),
        )
            .unwrap()
            .into_component(Some(MadSkin::default()))
            .unwrap();

        let lines = component.to_solo_table();

        assert!(
            lines.len() > 6,
            "Expected the long error message to be wrapped"
        );

        assert_line_widths(&lines, 30);
    }

    #[test]
    fn error_component_requires_skin() {
        let result = ErrorComponent::new(
            80,
            TEST_ERROR.to_string(),
            Paddings::default(),
        )
            .unwrap()
            .into_component(None);

        assert!(
            matches!(
                result,
                Err(TheComponentExpectsSkinButSkinNotFound(_))
            ),
            "Expected TheComponentExpectsSkinButSkinNotFound"
        );
    }

    #[test]
    fn error_component_renders_error_message() {
        let component = create_error_component(
            120,
            Paddings::default(),
        )
            .unwrap();

        let rendered = component
            .to_solo_table()
            .iter()
            .map(|line| visible_line(line))
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("ERROR"));
        assert!(rendered.contains(TEST_ERROR));
    }
}