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
  pub fn name<'a, S: AsRef<str>>(&'a mut self, name: S) -> &'a mut Self {
    self.name = Some(name.as_ref().to_string());
    self
  }

  /// Add a dependency for this task.
  pub fn dependency<'a, S: AsRef<str>>(&'a mut self, dep: S) -> &'a mut Self {
    self.dependencies.push(dep.as_ref().to_string());
    self
  }

  /// Add an output for this task.
  pub fn output<'a, S: AsRef<str>>(&'a mut self, dep: S) -> &'a mut Self {
    self.outputs.push(dep.as_ref().to_string());
    self
  }

  /// Build the task.
  pub fn build(&self) -> Task {
    Task {
      name: self.name.clone().expect("no name specified"),
      command: self.command.clone(),
      dependencies: self.dependencies.clone(),
      outputs: self.outputs.clone(),
    }
  }
}

#[test]
pub fn test_no_deps() {
  let task = Task::builder().name("bob").build();
  assert_eq!(task.dependencies.len(), 0);
  assert_eq!(task.outputs.len(), 0);
}

#[test]
pub fn test_one_dep_out() {
  let task = Task::builder().name("bob").dependency("bob.in").output("bob.out").build();
  assert_eq!(task.dependencies.len(), 1);
  assert_eq!(task.dependencies[0], "bob.in");
  assert_eq!(task.outputs.len(), 1);
  assert_eq!(task.outputs[0], "bob.out");
}
