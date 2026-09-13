use clap::{Args};
use serde::{Deserialize, Serialize};
use crate::console::json::Json;

#[derive(Args,Debug,Serialize,Deserialize)]
pub struct SetArg{
    pub public: String,
    pub private: String,
}

impl SetArg{
    pub fn new(public: String, private: String) -> Self {
        Self { public, private }
    }
    pub fn set(&self){
        let json = Json::new();
        json.set_config_data(&self);
    }
}