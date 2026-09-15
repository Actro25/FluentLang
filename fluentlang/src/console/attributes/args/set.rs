use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Debug, Serialize, Deserialize, Default, Clone)]
pub struct SetArg {
    pub public: String,
    pub private: String,
}
