use cucumber::{given, when, then};

use dpcl::pipeline::{Pipeline, Task};

use crate::TestEnv;

#[given("a fresh pipeline")]
fn fresh_pipeline(world: &mut TestEnv) {
  world.pipeline = Some(Pipeline::new());
}

#[when(expr="an empty task named {word} is added")]
fn add_empty_task(world: &mut TestEnv, name: String) {
  let pipe = world.pipeline();
  let task = Task::builder().name(&name);
  pipe.add_task(task.build());
}

#[then(expr="the pipeline has {int} tasks")]
fn pipeline_tasks_is_empty(world: &mut TestEnv, n: usize) {
  assert_eq!(world.pipeline().task_count(), n);
}

#[then(expr="the pipeline has {int} artifacts")]
fn pipeline_artifacts_is_empty(world: &mut TestEnv, n: usize) {
  assert_eq!(world.pipeline().artifact_count(), n);
}

#[then(expr="the pipeline has a task named {word}")]
fn pipeline_has_named_task(world: &mut TestEnv, name: String) {
  let task = world.pipeline().get_task(&name);
  assert!(task.is_some());
  assert_eq!(task.expect("no task").name, name);
}

#[then(expr="the task {word} has {int} dependencies")]
fn pipeline_task_has_n_deps(world: &mut TestEnv, name: String, n: usize) {
  // check the task object
  let task = world.pipeline().get_task(&name);
  let task = task.expect("no task");
  assert_eq!(task.dependencies.len(), n);

  // check the pipeline graph
  let deps = world.pipeline().task_dependencies(&name);
  assert_eq!(deps.len(), n);
}

#[then(expr="the task {word} has {int} outputs")]
fn pipeline_task_has_n_outs(world: &mut TestEnv, name: String, n: usize) {
  // check the task object
  let task = world.pipeline().get_task(&name);
  let task = task.expect("no task");
  assert_eq!(task.outputs.len(), n);

  // check the pipeline graph
  let deps = world.pipeline().task_outputs(&name);
  assert_eq!(deps.len(), n);
}
