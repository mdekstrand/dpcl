//! Types and algorithms to represent and manipulate pipelines.
//!
//! A pipeline is represented as a directed bipartite graph of tasks and files.

pub mod artifact;
pub mod task;
#[cfg(test)]
mod test;

use std::collections::HashMap;

use petgraph::{Graph, Directed};
use task::Task;
use artifact::Artifact;

use thiserror::Error;

/// Error encountered when setting up the pipeline.
#[derive(Error, Debug)]
pub enum SetupError {
  #[error("task with name ‘{0}’ already exists")]
  DuplicateTask(String),
}

#[derive(Debug, Clone)]
enum PipeNode {
  Task(Task),
  Artifact(Artifact),
}

#[derive(Debug, Clone)]
pub struct Pipeline {
  graph: Graph<PipeNode, (), Directed>,
  tasks: HashMap<String, u32>,
  artifacts: HashMap<String, u32>,
}

impl Pipeline {
  pub fn new() -> Pipeline {
    Pipeline {
      graph: Graph::new(),
      tasks: HashMap::new(),
      artifacts: HashMap::new(),
    }
  }

  pub fn task_count(&self) -> usize {
    self.tasks.len()
  }

  pub fn artifact_count(&self) -> usize {
    self.artifacts.len()
  }
}
