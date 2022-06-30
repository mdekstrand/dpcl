//! Pipeline tasks.

/// A single task in a pipeline.
#[derive(Debug, Clone)]
pub struct Task {
  name: String,
  command: Option<String>,
  dependencies: Vec<String>,
  outputs: Vec<String>,
}
