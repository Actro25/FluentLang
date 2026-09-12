use clap::{Args};

#[derive(Args,Debug)]
pub struct SetArg{
    pub public: String,
    pub private: String,
}

impl SetArg{
    pub fn set(&self){
        todo!("Implement setting api keys,")
    }
}