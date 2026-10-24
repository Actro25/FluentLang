use crate::table_components::TableComponent;
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
    component: Vec<String>,
    width: usize
}

impl ArtComponent {
    pub fn new(terminal_width: u16) -> Self {
        Self {
            component: Vec::new(),
            width: (terminal_width as usize).saturating_sub(2)
        }
    }

    fn get_art(&self) -> Vec<String> {
        let full_art_max_width = FULL_ART
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap();

        if (self.width / full_art_max_width) as f64 > 1.5 {
            FULL_ART.clone()
        } else {
            SMALL_ART.clone()
        }
    }
}

impl TableComponent for ArtComponent {
    fn get_component(mut self) -> Self {
        let mut current_art = self.get_art();

        let art_width = current_art.iter().map(|l| l.chars().count()).max().unwrap();

        let total_pad = self.width.saturating_sub(art_width);
        let left = total_pad / 2;

        for line in current_art.iter_mut() {
            let line_width = line.chars().count();
            let right = self.width.saturating_sub(left + line_width);

            *line = format!("│{}{}{}│", " ".repeat(left), line, " ".repeat(right));
        }

        self.component = current_art;

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
