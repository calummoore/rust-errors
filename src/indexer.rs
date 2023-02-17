use std::collections::HashMap;

pub struct Indexer {
  state: HashMap<String, String>
}

impl Indexer {
  pub fn new() -> Indexer {
    Indexer { state: HashMap::new() }
  }

  pub fn get(&self, id: &String) -> Option<&String> {
    self.state.get(id)
  }

  pub fn set(&mut self, id: String, value: String) {
    self.state.insert(id, value);
  }
}