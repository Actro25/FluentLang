pub mod fl_art;

pub trait ArtDraw {
    fn get_art(&self, terminal_width: u16) -> Vec<String>;
}