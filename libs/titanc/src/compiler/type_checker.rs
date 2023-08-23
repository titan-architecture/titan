use super::{
  ast::{Expression, ExpressionKind, Literal, Scope, Statement, StatementKind, TypeKind},
  symbol_table::{Symbol, SymbolTable},
};

pub struct TypeChecker {
  symbol_table: SymbolTable,
}

impl TypeChecker {
  pub fn new() -> TypeChecker {
    TypeChecker {
      symbol_table: SymbolTable::new(),
    }
  }

  pub fn type_check_scope(&mut self, scope: &Scope) {
    self.symbol_table.push_scope();

    for statement in &scope.statements {
      self.type_check_statement(&statement);
    }

    self.symbol_table.pop_scope();
  }

  pub fn type_of_expression(&mut self, expr: &Expression) -> TypeKind {
    match &expr.kind {
      ExpressionKind::Literal(lit) => match &lit {
        Literal::Boolean(_) => TypeKind::Boolean,
        Literal::Integer(_) => TypeKind::Integer,
        Literal::String(_) => TypeKind::String,
        _ => panic!("unexpected literal kind: {:?}", lit),
      },
      _ => panic!("unexpected expression kind: {:?}", &expr.kind),
    }
  }

  pub fn assert_type_matches(&mut self, actual_type: &TypeKind, expected_type: &TypeKind) {
    if !matches!(actual_type, expected_type) {
      panic!("expected type: \"{:?}\" but got: \"{:?}\"", expected_type, actual_type);
    }
  }

  pub fn type_check_statement(&mut self, statement: &Statement) {
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

        match self.symbol_table.add_symbol(Symbol {
          name: identifier.clone(),
          type_: type_annotation.as_ref().unwrap().kind,
        }) {
          Err(msg) => {
            panic!("{}", msg);
          }
          _ => {}
        }

        // check type annotation against expression type
        let expr_type = self.type_of_expression(value);
        self.assert_type_matches(&expr_type, &type_annotation.as_ref().unwrap().kind)
      }
      _ => {}
    }
  }
}
