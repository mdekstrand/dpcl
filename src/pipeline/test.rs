use super::*;
use hamcrest2::prelude::*;

/// Initial conditions for an empty pipeline.
#[test]
fn test_empty_pipeline() {
  let pipe = Pipeline::new();

  assert_that!(pipe.task_count(), eq(0));
  assert_that!(pipe.artifact_count(), eq(0));
  assert_that!(pipe.get_task("bob"), none());
  assert_that!(pipe.get_task("bob"), none());
}


/// Test a single task with no inputs or outputs.
#[test]
fn test_add_empty_task() {
  let mut pipe = Pipeline::new();

  let task = Task::builder().name("bob").build();
  pipe.add_task(task);

  assert_that!(pipe.task_count(), eq(1));
  assert_that!(pipe.artifact_count(), eq(0));

  let task = pipe.get_task("bob");
  assert_that!(task, some());
  let task = task.expect("no task found");
  assert_that!(&task.name, eq("bob"));
  assert_that!(&task.dependencies, len(0));
  assert_that!(&task.outputs, len(0));

  // look at the graph
  assert_that!(&pipe.task_dependencies("bob"), len(0));
  assert_that!(&pipe.task_outputs("bob"), len(0));
}

/// Test a single task with one input and one output.
#[test]
fn test_add_single_inout() {
  let mut pipe = Pipeline::new();

  let task = Task::builder().name("bob").dependency("bob.in").output("bob.out").build();
  pipe.add_task(task);

  assert_that!(pipe.task_count(), eq(1));
  assert_that!(pipe.artifact_count(), eq(2));

  let task = pipe.get_task("bob");
  assert_that!(task, some());
  let task = task.expect("no task found");
  assert_that!(task.name.as_str(), eq("bob"));
  assert_that!(&task.dependencies, len(1));
  assert_that!(&task.outputs, len(1));

  // now let's look at that graph
  let deps = pipe.task_dependencies("bob");
  assert_that!(&deps, len(1));
  assert_that!(&deps[0].path, eq("bob.in"));

  let outs = pipe.task_outputs("bob");
  assert_that!(&outs, len(1));
  assert_that!(&outs[0].path, eq("bob.out"));

  // and can we find the artifacts?
  let dep = pipe.get_artifact("bob.in");
  assert_that!(dep, some());
  assert_that!(&dep.expect("no artifact").path, eq("bob.in"));

  let out = pipe.get_artifact("bob.out");
  assert_that!(out, some());
  assert_that!(&out.expect("no artifact").path, eq("bob.out"));
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
  assert!(task.is_some());
  let task = task.expect("no task found");
  assert_eq!(task.name.as_str(), "bob");
  assert_eq!(task.dependencies.len(), 2);
  assert_eq!(task.outputs.len(), 1);

  let task = pipe.get_task("alice");
  assert!(task.is_some());
  let task = task.expect("no task found");
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

  // and can we find the artifacts?
  let dep = pipe.get_artifact("bob.in");
  assert!(dep.is_some());
  assert_eq!(dep.expect("no artifact").path, "bob.in");

  let out = pipe.get_artifact("bob.out");
  assert!(out.is_some());
  assert_eq!(out.expect("no artifact").path, "bob.out");
}
