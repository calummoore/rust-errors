use crate::indexer::Indexer;

pub type Result<T> = std::result::Result<T, DbError>;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
  #[error("Record not found")]
  RecordNotFound,
  #[error("Key too long, maximum length is 10")]
  KeyTooLong,
}

pub struct Db {
  indexer: Indexer
}

impl Db {
  pub fn new() -> Db {
    Db { indexer: Indexer::new() }
  }

  pub async fn get(&self, key: &String) -> Result<&String> {
    if key.len() > 10 {
      return Err(DbError::KeyTooLong);
    }

    let record = self.indexer.get(key);

    match record {
      Some(record) => Ok(record),
      None => Err(DbError::RecordNotFound)
    }
  }

  pub async fn set(&mut self, id: String, value: String) {
    self.indexer.set(id, value);
  }
}