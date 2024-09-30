use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Trait for both sync and async calls
#[allow(unused)]
pub trait Service<O: Sized, F: Sized> {
    fn call_sync(&self) -> Result<Option<O>>;
    fn call(&self) -> Result<F>;
}

pub trait Utils<'a> {
    type T: Serialize + Deserialize<'a>;
    fn payload(&self) -> Option<&Self::T>;
}
