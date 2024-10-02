pub mod app;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EchoArg {
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EchoResult {
    pub result: String,
}
