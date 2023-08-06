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
    value: String,
    _type: Option<Typing>,
    span: Span,
  },
}

#[derive(Debug)]
pub struct Typing {
  pub kind: TypeKind,
  pub span: Option<Span>,
}

#[derive(Debug)]
pub enum TypeKind {
  String,
  Integer,
  Inferred,
  Boolean,
}
