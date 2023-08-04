use std::fs::File;
use std::io::prelude::*;


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


  let parse_tree = match parser.parse(source_code, None) {
    Some(tree) => tree,
    None => {
      println!("Error parsing source code");
      return;
    }
  };

  // Dump out parse tree
  dbg!(parse_tree.root_node().to_sexp());
}
