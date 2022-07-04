use cucumber::{given, when, then, World};

use async_trait::async_trait;

use dpcl::pipeline::Pipeline;

#[given("a fresh pipeline")]
fn fresh_pipeline() -> Pipeline {
  Pipeline::new()
}
