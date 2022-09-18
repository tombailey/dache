use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum KeyValueStoreError {
    InvalidState
}

impl Display for KeyValueStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyValueStoreError.{:?}", self)
    }
}

impl Error for KeyValueStoreError {}
