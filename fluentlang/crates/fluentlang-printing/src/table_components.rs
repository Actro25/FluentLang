pub mod art_component;
pub mod chat_component;
pub mod status_component;
pub mod config_status_component;

pub trait TableComponent {
    fn get_component(self) -> Self
    where
        Self: Sized;
    fn to_start_of_table(&self) -> Vec<String>;
    fn to_middle_of_table(&self) -> Vec<String>;
    fn to_end_of_table(&self) -> Vec<String>;
    fn to_solo_table(&self) -> Vec<String>;
}