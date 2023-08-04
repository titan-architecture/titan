use tree_sitter::Node;
use crate::compiler::ast::Scope;
use crate::compiler::debug::Span;
use std::str;

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
    Self {
      source_code
    }
  }

  pub fn parse(&self, root: &Node) -> Scope {
    match root.kind() {
      "source_file" => self.build_scope(&root),
      _ => panic!("Unexpected root node kind: {}", root.kind()) // tree-sitter parse tree should always start  with source
    }
  }

  pub fn build_scope(&self, root: &Node) -> Scope {
    let span = self.node_span(&root);
    let node_text = self.node_text(&root);
    
    Scope {
      text: node_text.to_string(),
      span
    }
  }

  // helper function that takes a node and creates a Span from it
  pub fn node_span(&self, node: &Node) -> Span {
    let node_range = node.range();
    Span {
      start: node_range.start_byte,
      end: node_range.end_byte,
    }
  }

  // helper that gets the text out of a node
  fn node_text<'a>(&'a self, node: &Node) -> &'a str {
		return str::from_utf8(&self.source_code[node.byte_range()]).unwrap();
	}
}