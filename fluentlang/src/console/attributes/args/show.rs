use clap::Args;

#[derive(Args, Debug)]
pub struct ShowArgs{}

impl ShowArgs {
    pub fn show(&self) {
        todo!("Implement showing data.")
    }
}