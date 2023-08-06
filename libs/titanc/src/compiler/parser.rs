use crate::compiler::ast::Scope;
use crate::compiler::debug::Span;
use std::str;
use tree_sitter::Node;

use super::ast::{Statement, StatementKind, TypeKind, Typing};

// This file will contain our parser, it is responsible for taking the tree-sitter parse tree
// and converting it into our AST that we will then use in the rest of the compiler
// when a parser is created it will hold the source code as a byte array so that if we
// need to extract something from the source code we can do it easily. IE if we need to
// get the text of a node, we can just get the byte range of the node and then convert
// that to a string using the source code byte array
pub struct Parser<'a> {
  pub source_code: &'a [u8],
}

impl<'b> Parser<'b> {
  pub fn new(source_code: &'b [u8]) -> Self {
    Self { source_code }
  }

  pub fn parse(&self, root: &Node) -> Scope {
    match root.kind() {
      "source_file" => self.build_scope(&root),
      _ => panic!("Unexpected root node kind: {}", root.kind()), // tree-sitter parse tree should always start  with source
    }
  }

  pub fn build_scope(&self, root: &Node) -> Scope {
    let span = self.node_span(&root);

    let mut cursor = root.walk();

    let scope = Scope {
      statements: root
        .named_children(&mut cursor)
        .enumerate()
        .filter_map(|(_, statement_node)| self.build_statement(&statement_node))
        .collect(),
      span,
    };

    scope
  }

  // helper function that takes a node and creates a Span from it
  pub fn node_span(&self, node: &Node) -> Span {
    let node_range = node.range();
    Span {
      start: node_range.start_byte,
      end: node_range.end_byte,
    }
  }

  fn build_statement(&self, statement_node: &Node) -> Option<Statement> {
    let span = self.node_span(statement_node);
    match statement_node.kind() {
      "variable_definition" => {
        let kind = self.build_definition_statement(&statement_node);
        Some(Statement { kind, span })
      }
      _ => None,
    }
  }

  fn build_definition_statement(&self, statement_node: &Node) -> StatementKind {
    let _type = if let Some(type_node) = statement_node.child_by_field_name("type") {
      self.build_type_kind(Some(type_node)).ok()
    } else {
      None
    };
    let pattern = self
      .node_text(&statement_node.child_by_field_name("pattern").unwrap())
      .into();
    let value = self
      .node_text(&statement_node.child_by_field_name("value").unwrap())
      .into();

    StatementKind::Let {
      identifier: pattern,
      value,
      _type,
      span: self.node_span(statement_node),
    }
  }

  fn node_text<'a>(&'a self, node: &Node) -> &'a str {
    return str::from_utf8(&self.source_code[node.byte_range()]).unwrap();
  }

  fn build_type_kind(&self, type_node: Option<Node>) -> Result<Typing, ()> {
    let type_node = &match type_node {
      Some(node) => node,
      None => {
        return Ok(Typing {
          kind: TypeKind::Inferred,
          span: None,
        })
      }
    };

    let span = Some(self.node_span(type_node));

    match type_node.kind() {
      "alpha_identifier" => match self.node_text(type_node) {
        "string" => Ok(Typing {
          kind: TypeKind::String,
          span,
        }),
        "int" => Ok(Typing {
          kind: TypeKind::Integer,
          span,
        }),
        "bool" => Ok(Typing {
          kind: TypeKind::Boolean,
          span,
        }),
        _ => Err(()),
      },
      _ => Err(()),
    }
  }
}
