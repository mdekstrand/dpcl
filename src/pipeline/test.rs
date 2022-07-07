use super::*;

#[test]
fn test_empty_pipeline() {
  let pipe = Pipeline::new();

  assert_eq!(pipe.task_count(), 0);
  assert_eq!(pipe.artifact_count(), 0);
}


#[test]
fn test_add_empty_task() {
  let mut pipe = Pipeline::new();

  let task = Task::builder().name("bob").build();
  pipe.add_task(task);

  assert_eq!(pipe.task_count(), 1);
  assert_eq!(pipe.artifact_count(), 0);

  let task = pipe.get_task("bob");
  assert!(task.is_some());
  let task = task.expect("no task found");
  assert_eq!(task.name.as_str(), "bob");
  assert_eq!(task.dependencies.len(), 0);
  assert_eq!(task.outputs.len(), 0);

  assert_eq!(pipe.task_dependencies("bob").len(), 0);
  assert_eq!(pipe.task_outputs("bob").len(), 0);
}
