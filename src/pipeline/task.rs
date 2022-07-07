//! Pipeline tasks.

/// A single task in a pipeline.
#[derive(Debug, Clone)]
pub struct Task {
  pub name: String,
  pub command: Option<String>,
  pub dependencies: Vec<String>,
  pub outputs: Vec<String>,
}

/// Builder to construct a task.
///
/// Use [Task::builder] to construct this.
#[derive(Default, Debug)]
pub struct TaskBuilder {
  name: Option<String>,
  command: Option<String>,
  dependencies: Vec<String>,
  outputs: Vec<String>,
}

impl Task {
  /// Construct a new task builder.
  pub fn builder() -> TaskBuilder {
    TaskBuilder::default()
  }
}

impl TaskBuilder {
  /// Set the name for this task.
  pub fn name<S: AsRef<str>>(mut self, name: S) -> Self {
    self.name = Some(name.as_ref().to_string());
    self
  }

  /// Build the task.
  pub fn build(self) -> Task {
    Task {
      name: self.name.expect("no name specified"),
      command: self.command,
      dependencies: self.dependencies,
      outputs: self.outputs,
    }
  }
}
