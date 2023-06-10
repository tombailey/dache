use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum KeyValueStoreError {
    InvalidConfiguration { reason: String },
    StoreError { error: Box<dyn Error> },
}

impl KeyValueStoreError {
    pub fn invalid_configuration(reason: String) -> KeyValueStoreError {
        KeyValueStoreError::InvalidConfiguration { reason }
    }
}

impl Display for KeyValueStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyValueStoreError.{:?}", self)
    }
}

impl Error for KeyValueStoreError {}
