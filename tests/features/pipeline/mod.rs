use cucumber::{given, when, then};

use dpcl::pipeline::Pipeline;

use crate::TestEnv;

#[given("a fresh pipeline")]
fn fresh_pipeline(world: &mut TestEnv) {
  world.pipeline = Some(Pipeline::new());
}

#[then(expr="the pipeline has {int} tasks")]
fn pipeline_tasks_is_empty(world: &mut TestEnv, n: usize) {
  assert_eq!(world.pipeline().task_count(), n);
}

#[then(expr="the pipeline has {int} artifacts")]
fn pipeline_artifacts_is_empty(world: &mut TestEnv, n: usize) {
  assert_eq!(world.pipeline().artifact_count(), n);
}
