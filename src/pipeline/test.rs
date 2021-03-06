use super::*;

/// Initial conditions for an empty pipeline.
#[test]
fn test_empty_pipeline() {
  let pipe = Pipeline::new();

  assert_eq!(pipe.task_count(), 0);
  assert_eq!(pipe.artifact_count(), 0);
}


/// Test a single task with no inputs or outputs.
#[test]
fn test_add_empty_task() {
  let mut pipe = Pipeline::new();

  let task = Task::builder().name("bob").build();
  pipe.add_task(task);

  assert_eq!(pipe.task_count(), 1);
  assert_eq!(pipe.artifact_count(), 0);

  let task = pipe.get_task("bob");
  let task = task.expect("task 'bob' not found");
  assert_eq!(task.name.as_str(), "bob");
  assert_eq!(task.dependencies.len(), 0);
  assert_eq!(task.outputs.len(), 0);

  // look at the graph
  assert_eq!(pipe.task_dependencies("bob").len(), 0);
  assert_eq!(pipe.task_outputs("bob").len(), 0);
}

/// Test a single task with one input and one output.
#[test]
fn test_add_single_inout() {
  let mut pipe = Pipeline::new();

  let task = Task::builder().name("bob").dependency("bob.in").output("bob.out").build();
  pipe.add_task(task);

  assert_eq!(pipe.task_count(), 1);
  assert_eq!(pipe.artifact_count(), 2);

  let task = pipe.get_task("bob");
  let task = task.expect("task 'bob' not found");
  assert_eq!(task.name.as_str(), "bob");
  assert_eq!(task.dependencies.len(), 1);
  assert_eq!(task.outputs.len(), 1);

  // now let's look at that graph
  let deps = pipe.task_dependencies("bob");
  assert_eq!(deps.len(), 1);
  assert_eq!(deps[0].path, "bob.in");

  let outs = pipe.task_outputs("bob");
  assert_eq!(outs.len(), 1);
  assert_eq!(outs[0].path, "bob.out");

  // and can we find the artifacts?
  let dep = pipe.get_artifact("bob.in");
  assert!(dep.is_some());
  assert_eq!(dep.expect("no artifact").path, "bob.in");

  let out = pipe.get_artifact("bob.out");
  assert!(out.is_some());
  assert_eq!(out.expect("no artifact").path, "bob.out");
}

/// Test wiring two tasks together
#[test]
fn test_add_task_wire() {
  let mut pipe = Pipeline::new();

  let task = Task::builder().name("alice").output("alice.out").build();
  pipe.add_task(task);

  let task = Task::builder().name("bob").dependency("bob.in").dependency("alice.out").output("bob.out").build();
  pipe.add_task(task);

  assert_eq!(pipe.task_count(), 2);
  assert_eq!(pipe.artifact_count(), 3);

  let task = pipe.get_task("bob");
  let task = task.expect("task 'bob' not found");
  assert_eq!(task.name.as_str(), "bob");
  assert_eq!(task.dependencies.len(), 2);
  assert_eq!(task.outputs.len(), 1);

  let task = pipe.get_task("alice");
  let task = task.expect("task 'alice' not found");
  assert_eq!(task.name.as_str(), "alice");
  assert_eq!(task.dependencies.len(), 0);
  assert_eq!(task.outputs.len(), 1);

  // now let's look at that graph
  let deps = pipe.task_dependencies("bob");
  assert_eq!(deps.len(), 2);
  let paths: Vec<_> = deps.into_iter().map(|a| a.path.as_str()).collect();
  assert!(paths.contains(&"bob.in"));
  assert!(paths.contains(&"alice.out"));

  let outs = pipe.task_outputs("bob");
  assert_eq!(outs.len(), 1);
  assert_eq!(outs[0].path, "bob.out");

  assert_eq!(pipe.task_dependencies("alice").len(), 0);

  let outs = pipe.task_outputs("alice");
  assert_eq!(outs.len(), 1);
  assert_eq!(outs[0].path, "alice.out");

  // and can we find the artifacts?
  let dep = pipe.get_artifact("bob.in");
  assert_eq!(dep.expect("no artifact").path, "bob.in");

  let out = pipe.get_artifact("bob.out");
  assert_eq!(out.expect("no artifact").path, "bob.out");

  let out = pipe.get_artifact("alice.out");
  assert_eq!(out.expect("no artifact").path, "alice.out");
}
