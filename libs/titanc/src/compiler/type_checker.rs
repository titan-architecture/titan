use super::{ast::{Scope, Statement, StatementKind, TypeKind, ExpressionKind, Literal}, symbol_table::{SymbolTable, Symbol}};

pub struct TypeChecker {
  symbol_table: SymbolTable,
}

impl TypeChecker {
  pub fn new() -> TypeChecker {
    TypeChecker {
      symbol_table: SymbolTable::new()
    }
  }

  pub fn type_check_scope(&mut self, scope: &Scope) {
    self.symbol_table.push_scope();

    for statement in &scope.statements {
      self.type_check_statement(&statement);
    }
  }

  pub fn type_check_statement(&mut self, statement: &Statement,) {
    match &statement.kind {
      StatementKind::Let {
        identifier,
        value,
        type_annotation,
        ..
      } => {
        if let None = &type_annotation {
          panic!("Cannot infer types of variables yet");
        }

        match self.symbol_table.add_symbol(Symbol { name: identifier.clone(), type_: type_annotation.as_ref().unwrap().kind }) {
          Err(msg) => {
            panic!("{}", msg);
          }
          _ => {}
        }
        
        // check type annotation against expression type
        match type_annotation.as_ref().unwrap().kind {
          TypeKind::String => {
            match &value.kind {
              ExpressionKind::Literal(lit) => {
                if !matches!(lit, Literal::String(_)) {
                  panic!("expected string but got: {:?}", lit);
                }
              }
            }
          }
          TypeKind::Boolean => {
            match &value.kind {
              ExpressionKind::Literal(lit) => {
                if !matches!(lit, Literal::Boolean(_)) {
                  panic!("expected boolean but got: {:?}", lit);
                }
              }
            }
          }
          TypeKind::Integer => {
            match &value.kind {
              ExpressionKind::Literal(lit) => {
                if !matches!(lit, Literal::Integer(_)) {
                  panic!("expected integer but got: {:?}", lit);
                }
              }
            }
          }
          _ => panic!("unable to verify type")
        }
      }
      _ => {}
    }
  }
}
