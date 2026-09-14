use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Debug, Serialize, Deserialize, Default)]
pub struct SetArg {
    pub public: String,
    pub private: String,
}

impl Clone for SetArg {
    fn clone(&self) -> Self {
        Self {
            private: self.private.clone(),
            public: self.public.clone(),
        }
    }
}
