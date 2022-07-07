use cucumber::{given, when, then};

use dpcl::pipeline::{Task};

use crate::TestEnv;

#[given("a fresh pipeline")]
fn fresh_pipeline(world: &mut TestEnv) {
  world.pipeline = Some(Pipeline::new());
}
