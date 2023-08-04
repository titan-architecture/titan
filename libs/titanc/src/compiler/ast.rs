use crate::compiler::debug::Span;

// This file should contain all our AST nodes
// the root of our ast is always a scope

#[derive(Debug)]
pub struct Scope {
  pub text: String, // this is only here until we add change the grammar
  pub span: Span
}