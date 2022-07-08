//! Types and algorithms to represent and manipulate pipelines.
//!
//! A pipeline is represented as a directed bipartite graph of tasks and files.

pub mod artifact;
pub mod task;
#[cfg(test)]
mod test;

use std::collections::HashMap;

use petgraph::{Graph, Directed, graph::NodeIndex, Direction, data::Build};
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
    let node = PipeNode::Task(task.clone());
    let idx = self.graph.add_node(node);
    self.tasks.insert(name, idx);

    for dep in task.dependencies {
      let di = self.find_or_insert_artifact(&dep);
      self.graph.add_edge(di, idx, ());
    }

    for out in task.outputs {
      let oi = self.find_or_insert_artifact(&out);
      self.graph.add_edge(idx, oi, ());
    }
  }

  /// Get a task by name.
  pub fn get_task(&self, name: &str) -> Option<&Task> {
    let idx = self.tasks.get(name);
    idx.and_then(|i| self.get_task_by_index(*i))
  }

  /// Get an artifact by name.
  pub fn get_artifact(&self, path: &str) -> Option<&Artifact> {
    let idx = self.artifacts.get(path);
    idx.and_then(|i| self.get_artifact_by_index(*i))
  }

  /// Get the task at an index.
  fn get_task_by_index(&self, idx: NodeIndex) -> Option<&Task> {
    let node = self.graph.node_weight(idx);
    node.map(|n| match n {
      PipeNode::Task(task) => task,
      _ => panic!("task node does not have task weight") // internal error
    })
  }

  /// Get the atrifact at an index.
  fn get_artifact_by_index(&self, idx: NodeIndex) -> Option<&Artifact> {
    let node = self.graph.node_weight(idx);
    node.map(|n| match n {
      PipeNode::Artifact(art) => art,
      _ => panic!("artifact node does not have artifact weight") // internal error
    })
  }

  /// Look up or insert an artifact by path.
  fn find_or_insert_artifact(&mut self, path: &str) -> NodeIndex {
    let e = self.artifacts.entry(path.to_string()).or_insert_with(|| {
      let art = Artifact { path: path.to_string() };
      self.graph.add_node(PipeNode::Artifact(art))
    });
    *e
  }

  /// Get the dependencies of a task.
  pub fn task_dependencies(&self, name: &str) -> Vec<&Artifact> {
    if let Some(idx) = self.tasks.get(name) {
      let deps = self.graph.neighbors_directed(*idx, Direction::Incoming);
      deps.into_iter().map(|i| self.get_artifact_by_index(i).expect("index doesn't exist")).collect()
    } else {
      Vec::new()
    }
  }

  /// Get the outputs of a task.
  pub fn task_outputs(&self, name: &str) -> Vec<&Artifact> {
    if let Some(idx) = self.tasks.get(name) {
      let deps = self.graph.neighbors_directed(*idx, Direction::Outgoing);
      deps.into_iter().map(|i| self.get_artifact_by_index(i).expect("index doesn't exist")).collect()
    } else {
      Vec::new()
    }
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
