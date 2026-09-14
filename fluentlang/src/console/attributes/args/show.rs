use crate::console::json::Json;
use clap::Args;

#[derive(Args, Debug)]
pub struct ShowArgs {}

impl ShowArgs {
    pub fn show(&self) {
        let json = Json::new();
        let args = json.get_config_data();
        print!("Public key: {}", args.public);
        print!("Private key: {}", args.private);
    }
}
