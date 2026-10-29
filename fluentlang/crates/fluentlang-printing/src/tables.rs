pub mod fluent_lang_table;

use crate::table_components::TableComponent;
use termimad::crossterm::style::Color;

pub trait Table {
    /// The function was created for adding `component` into `components` list
    ///
    /// * `component` - a component.
    fn add_component(&mut self, component: Box<dyn TableComponent>) -> &mut Self;

    /// The function was created for printing `components` using `fg` and `bg`.
    ///
    /// * `fg` - a color of the symbols.
    /// * `bg` - a color of the background in the terminal.
    fn render(&self, fg: Color, bg: Color);
}
