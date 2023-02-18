// use std::backtrace::Backtrace;
use crate::indexer::{Indexer, IndexerError};

pub type Result<T> = std::result::Result<T, DbError>;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
  #[error("Db record not found")]
  RecordNotFound {
    source: IndexerError
  },

  #[error(transparent)]
  IndexerErr{
    #[from]
    source: IndexerError,
  }
}

pub struct Db {
  indexer: Indexer
}

impl Db {
  pub fn new() -> Db {
    Db { indexer: Indexer::new() }
  }

  pub async fn get(&self, key: &String) -> Result<&String> {
    match self.indexer.get(key) {
      Ok(record) => Ok(record),
      Err(err) => {
        match err {
          IndexerError::RecordNotFound => Err(DbError::RecordNotFound { source: err }),
          _ => Err(DbError::IndexerErr { source: err })
        }
      },
    }
  }

  pub async fn set(&mut self, id: String, value: String) {
    self.indexer.set(id, value);
  }
}