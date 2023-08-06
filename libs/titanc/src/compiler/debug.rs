// Right now the span is very simple, we will probably add more to it later
// but they hold the start and end column of a node
#[derive(Debug)]
pub struct Span {
  pub start: usize,
  pub end: usize,
}
