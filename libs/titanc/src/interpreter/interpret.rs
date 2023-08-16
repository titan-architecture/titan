use crate::compiler::ast::{Scope, Statement, StatementKind};

use super::environment::Environment;

pub struct Interpreter<'a> {
  pub scope: &'a Scope
}

impl<'b> Interpreter<'b> {
  pub fn new(scope: &'b Scope) -> Self {
    Self {
      scope
    }
  }

  pub fn interpret(&self) {
    self.interpret_scope(self.scope);
  }

  pub fn interpret_scope(&self, scope: &Scope) {
    // TODO: support inheriting environments
    let mut scope_env = Environment::new();
    for statement in &scope.statements {
      scope_env = self.interpret_statement(statement, scope_env);
    }
  }

  pub fn interpret_statement(&self, statement: &Statement, mut env: Environment) -> Environment {
    match &statement.kind {
      StatementKind::Let { identifier, value, _type, .. } => {
        env.define_variable(identifier.clone(), value.clone());
      }
      StatementKind::Print { identifier, ..} => {
        // TODO: support printing expressions and literals
        let value = env.get_variable(identifier);
        if let Some(val) = value {
          println!("{}", val);
        } else {
          panic!("Unable to lookup variable: {}", identifier);
        }
      }
    }
    env
  }
}