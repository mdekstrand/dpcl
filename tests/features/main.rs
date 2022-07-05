use std::convert::Infallible;

use cucumber::{World, WorldInit};
use async_trait::async_trait;
use dpcl::pipeline::Pipeline;

mod pipeline;

static TESTS: &[&str] = &[
  "pipeline-tasks"
];

#[derive(WorldInit, Debug)]
struct TestEnv {
  pipeline: Option<Pipeline>,
}

impl TestEnv {
  /// Get the test environment pipeline.
  fn pipeline(&mut self) -> &mut Pipeline {
    self.pipeline.as_mut().expect("pipeline not initialized")
  }
}

#[async_trait(?Send)]
impl World for TestEnv {
  type Error = Infallible;

  async fn new() -> Result<Self, Infallible> {
    Ok(TestEnv {
      pipeline: None,
    })
  }
}


fn main() {
  for feature in TESTS {
    let path = format!("tests/features/{}.feature", feature);
    futures::executor::block_on(TestEnv::run(path))
  }
}
