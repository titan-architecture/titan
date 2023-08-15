// until this is converted to a rust library,
// any files you add in compiler will need to be listed here as mods
// to be able to use them in other files
mod compiler {
  pub mod ast;
  pub mod debug;
  pub mod parser;
  pub mod type_checker;
}

use std::fs::File;
use std::io::prelude::*;

use crate::compiler::parser::Parser;

// This main entry point will be for debugging
// eventually we wont need it since this compiler should become
// a library imported by the cli (still to be created) and there will
// be a compile function that will be called from the cli
// for now to test, just add some code in the test.titan file in the tree-sitter-titan lib
// then from here run `cargo run`
fn main() {
  // Initial language setup
  let language = tree_sitter_titan::language();
  let mut parser = tree_sitter::Parser::new();
  parser.set_language(language).unwrap();

  // Read source file
  let mut file = File::open("../tree-sitter-titan/test.titan").unwrap();
  let mut source_code = String::new();
  file.read_to_string(&mut source_code).unwrap();

  let parse_tree = match parser.parse(&source_code, None) {
    Some(tree) => tree,
    None => {
      println!("Error parsing source code");
      return;
    }
  };

  let parser = Parser::new(&source_code.as_bytes());

  // This will be the root of our ast. (which is a scope)
  let root = parser.parse(&parse_tree.root_node());

  // Type check
  let mut type_checker = compiler::type_checker::TypeChecker::new();
  type_checker.type_check_scope(&root);

  // dump the ast to terminal
  dbg!(root);
}
