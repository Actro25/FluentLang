use std::sync::LazyLock;
use crate::art::ArtDraw;

pub(crate) static FULL_ART: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        r"______  _                  _        _".to_string(),
        r"| ____|| |_   _  ___ _ __ | |_     | |    __ _ _ __   __ _".to_string(),
        r"| |_   | | | | |/ _ \ '_ \| __|____| |   / _` | '_ \ / _` |".to_string(),
        r"|  _|  | | |_| |  __/ | | | ||_____| |__| (_| | | | | (_| |".to_string(),
        r"|_|    |_|\__,_|\___|_| |_|\__|    |_____\__,_|_| |_|\__, |".to_string(),
        r"                                                     |___/".to_string(),
    ]
});
pub(crate) static SMALL_ART: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        r"    ______ _".to_string(),
        r"   | ____|| | ".to_string(),
        r"  | |_   | |".to_string(),
        r" |  _|  | |___".to_string(),
        r"|_|    |_____|".to_string(),
    ]
});

#[derive(Default)]
pub struct FluentLangArts {}

impl ArtDraw for FluentLangArts {
    fn get_art(&self, terminal_width: u16) -> Vec<String> {
        let inner_width = (terminal_width as usize).saturating_sub(2);
        let full_art_max_width = FULL_ART.iter().map(|l| l.chars().count()).max().unwrap();

        if (inner_width / full_art_max_width) as f64 > 1.5 {
            FULL_ART.clone()
        }
        else {
            SMALL_ART.clone()
        }
    }
}