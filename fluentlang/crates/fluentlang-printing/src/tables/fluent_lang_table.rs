use crate::table_components::TableComponent;
use crate::tables::Table;
use termimad::crossterm::style::{Color, Stylize};

#[derive(Default)]
pub struct FluentLangTable {
    components: Vec<Box<dyn TableComponent>>,
}

impl Table for FluentLangTable {
    fn add_component(&mut self, component: Box<dyn TableComponent>) -> &mut Self {
        self.components.push(component);
        self
    }

    fn render(&self, fg: Color, bg: Color) {
        match self.components.len() {
            1 => self.render_table(self.components[0].to_solo_table(), fg, bg),
            2 => {
                for (index, component) in self.components.iter().enumerate() {
                    let res = if index == 0 {
                        component.to_start_of_table()
                    } else {
                        component.to_end_of_table()
                    };

                    self.render_table(res, fg, bg);
                }
            }
            _ => {
                for (index, component) in self.components.iter().enumerate() {
                    let res = if index == 0 {
                        component.to_start_of_table()
                    } else if index == self.components.len() - 1 {
                        component.to_end_of_table()
                    } else {
                        component.to_middle_of_table()
                    };

                    self.render_table(res, fg, bg);
                }
            }
        }
    }
}

impl FluentLangTable {
    #[inline]
    fn render_table(&self, res: Vec<String>, fg: Color, bg: Color) {
        for line in res {
            println!("{}", line.as_str().with(fg).on(bg));
        }
    }
}