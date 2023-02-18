use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, IndexerError>;

#[derive(Debug, thiserror::Error)]
pub enum IndexerError {
  #[error("Index record not found")]
  RecordNotFound,

  #[error("Key too long, maximum length is 10")]
  KeyTooLong
}


pub struct Indexer {
  state: HashMap<String, String>
}

impl Indexer {
  pub fn new() -> Indexer {
    Indexer { state: HashMap::new() }
  }

  pub fn get(&self, key: &String) -> Result<&String> {
    if key.len() > 10 {
      return Err(IndexerError::KeyTooLong);
    }

    let record = self.state.get(key);

    match record {
      Some(record) => Ok(record),
      None => Err(IndexerError::RecordNotFound)
    }
  }

  pub fn set(&mut self, id: String, value: String) {
    self.state.insert(id, value);
  }
}