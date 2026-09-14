use crate::console::json::Json;
use clap::Args;

#[derive(Args, Debug)]
pub struct ShowArgs {}

impl ShowArgs {
    pub fn show(&self) {
        let json = Json::new();
        let output = json.get_config_data();

        match output {
            Ok(output) => {
                println!("Public key: {}", output.keys.public);
                println!("Private key: {}", output.keys.private);
            }
            Err(err) => println!("There is an error: {}", err),
        }
    }
}
