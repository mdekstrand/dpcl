use std::convert::Infallible;

use cucumber::{World, WorldInit};
use async_trait::async_trait;

mod pipeline;

static TESTS: &[&str] = &[
  "pipeline-tasks"
];

#[derive(Debug, WorldInit)]
struct DailyPlanet;

#[async_trait(?Send)]
impl World for DailyPlanet {
  type Error = Infallible;

  async fn new() -> Result<Self, Infallible> {
    Ok(DailyPlanet)
  }
}


fn main() {
  for feature in TESTS {
    let path = format!("tests/features/{}.feature", feature);
    futures::executor::block_on(DailyPlanet::run(path))
  }
}
