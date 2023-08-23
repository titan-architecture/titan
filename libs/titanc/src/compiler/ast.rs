use crate::compiler::debug::Span;

// This file should contain all our AST nodes
// the root of our ast is always a scope

#[derive(Debug)]
pub struct Scope {
  pub statements: Vec<Statement>,
  pub span: Span,
}

#[derive(Debug)]
pub struct Statement {
  pub kind: StatementKind,
  pub span: Span,
}

#[derive(Debug)]
pub enum StatementKind {
  Let {
    identifier: String,
    value: Expression,
    type_annotation: Option<Typing>,
  },
  Print {
    identifier: String, // TODO: handle expressions and literals
  },
}

#[derive(Debug)]
pub struct Typing {
  pub kind: TypeKind,
}

#[derive(Debug, Clone, Copy)]
pub enum TypeKind {
  String,
  Integer,
  Inferred,
  Boolean,
}

#[derive(Debug)]
pub struct Expression {
  pub kind: ExpressionKind,
}

#[derive(Debug)]
pub enum ExpressionKind {
  Literal(Literal),
}

#[derive(Debug)]
pub enum Literal {
  String(String),
  Boolean(bool),
  Integer(f64),
}
