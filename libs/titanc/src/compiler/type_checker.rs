use super::ast::{Scope, Statement, StatementKind, TypeKind};

pub struct TypeChecker<> {

}

impl TypeChecker {
  pub fn new() -> TypeChecker {
    TypeChecker {}
  }

  pub fn type_check_scope(&self, scope: &Scope) {
    for statement in &scope.statements {
      self.type_check_statement(&statement);
    }
  }

  pub fn type_check_statement(&self, statement: &Statement) {
    match &statement.kind {
      StatementKind::Let { identifier, value, _type, .. } => {
        if let None = &_type {
          panic!("Cannot infer types of variables yet");
        }

        if let Some(typing) = _type {
          match typing.kind {
            TypeKind::String => {
              if value.starts_with("\"") && value.ends_with("\"") {
                return;
              } else {
                panic!("expected string literal for variable {:?} but got {:?}", identifier, value);
              }
            }
            TypeKind::Integer => {
              if value.parse::<i32>().is_ok() {
                return;
              } else {
                panic!("expected integer literal for variable {:?} but got {:?}", identifier, value);
              }
            }
            TypeKind::Boolean => {
              if value == "true" || value == "false" {
                return;
              } else {
                panic!("expected boolean literal for variable {:?} but got {:?}", identifier, value);
              }
            }
            _ => {
              panic!("cannot type check {:?} yet", typing.kind);
            }
          }
        }
      }
    }
  }
}