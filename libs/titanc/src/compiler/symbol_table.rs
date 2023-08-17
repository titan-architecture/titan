use std::collections::HashMap;

use super::ast::TypeKind;

#[derive(Clone)]
pub struct Symbol {
  pub(crate) name: String,
  pub(crate) type_: TypeKind,
}

pub struct SymbolTable {
  current_scope: HashMap<String, Symbol>,
  outer_scope: Option<Box<SymbolTable>>,
}

impl SymbolTable {
  pub fn new() -> Self {
    SymbolTable {
      current_scope: HashMap::new(),
      outer_scope: None,
    }
  }

  pub fn push_scope(&mut self) {
    let new_table = SymbolTable {
      current_scope: HashMap::new(),
      outer_scope: Some(Box::new(self.clone())),
    };
    *self = new_table;
  }

  pub fn pop_scope(&mut self) {
    if let Some(outer) = self.outer_scope.take() {
      *self = *outer;
    }
  }

  // Add a symbol to the current scope
  pub fn add_symbol(&mut self, symbol: Symbol) -> Result<(), String> {
    if self.current_scope.contains_key(&symbol.name) {
      return Err(format!("Symbol {} already exists in the current scope.", symbol.name));
    }
    self.current_scope.insert(symbol.name.clone(), symbol);
    Ok(())
  }

  // Find a symbol, starting from the current scope and moving outward
  pub fn find_symbol(&self, name: &str) -> Option<&Symbol> {
    if let Some(symbol) = self.current_scope.get(name) {
      Some(symbol)
    } else if let Some(outer) = &self.outer_scope {
      outer.find_symbol(name)
    } else {
      None
    }
  }
}

impl Clone for SymbolTable {
  fn clone(&self) -> Self {
    Self {
      current_scope: self.current_scope.clone(),
      outer_scope: self.outer_scope.clone(),
    }
  }
}
