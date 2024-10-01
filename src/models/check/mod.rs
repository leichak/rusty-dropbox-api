pub mod app;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EchoArg {
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EchoResult {
    result: String,
}
