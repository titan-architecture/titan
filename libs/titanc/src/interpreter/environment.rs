use std::collections::HashMap;

// Very simple environment for now, just a map of strings to strings
// need to figure out how to handle different types of values
pub struct Environment {
  pub values: HashMap<String, String>,
}

impl Environment {
  pub fn new() -> Self {
    Environment { values: HashMap::new() }
  }

  pub fn define_variable(&mut self, name: String, value: String) {
    self.values.insert(name, value);
  }

  pub fn get_variable(&self, name: &String) -> Option<&String> {
    self.values.get(name)
  }
}
