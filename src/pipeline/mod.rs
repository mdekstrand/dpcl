//! Types and algorithms to represent and manipulate pipelines.
//!
//! A pipeline is represented as a directed bipartite graph of tasks and files.

pub mod artifact;
pub mod task;

use std::collections::HashMap;

use petgraph::{Graph, Directed};
use task::Task;
use artifact::Artifact;

use thiserror::Error;

/// Error encountered when setting up the pipeline.
#[derive(Error, Debug)]
pub enum SetupError {
  #[error("duplicate tasks detected")]
  DuplicateTask(String),
}

#[derive(Debug, Clone)]
enum PipeNode {
  Task(Task),
  Artifact(Artifact),
}

pub struct Pipeline {
  graph: Graph<PipeNode, (), Directed>,
  tasks: HashMap<String, u32>,
  files: HashMap<String, u32>,
}
