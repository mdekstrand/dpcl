//! Types and algorithms to represent and manipulate pipelines.
//!
//! A pipeline is represented as a directed bipartite graph of tasks and files.

pub mod artifact;
pub mod task;
#[cfg(test)]
mod test;

use std::collections::HashMap;

use petgraph::{Graph, Directed, graph::NodeIndex};
pub use task::Task;
pub use artifact::Artifact;

use log::*;
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
  tasks: HashMap<String, NodeIndex<u32>>,
  artifacts: HashMap<String, NodeIndex<u32>>,
}

impl Pipeline {
  pub fn new() -> Pipeline {
    Pipeline {
      graph: Graph::new(),
      tasks: HashMap::new(),
      artifacts: HashMap::new(),
    }
  }

  /// Add a new task to the pipeline.
  pub fn add_task(&mut self, task: Task) {
    let name = task.name.clone();
    debug!("adding task {}", name);
    let node = PipeNode::Task(task);
    let idx = self.graph.add_node(node);
    self.tasks.insert(name, idx);
  }

  /// Get a task by name.
  pub fn get_task(&self, name: &str) -> Option<&Task> {
    let idx = self.tasks.get(name);
    let node = idx.and_then(|i| self.graph.node_weight(*i));
    node.map(|n| match n {
      PipeNode::Task(task) => task,
      _ => panic!("task node does not have task weight") // internal error
    })
  }

  /// Get the dependencies of a task.
  pub fn task_dependencies(&self, name: &str) -> Vec<&Artifact> {
    Vec::new()
  }

  /// Get the outputs of a task.
  pub fn task_outputs(&self, name: &str) -> Vec<&Artifact> {
    Vec::new()
  }

  /// Get the number of tasks in the pipeline.
  pub fn task_count(&self) -> usize {
    self.tasks.len()
  }

  /// Get the number of artifacts in the pipeline.
  pub fn artifact_count(&self) -> usize {
    self.artifacts.len()
  }
}
