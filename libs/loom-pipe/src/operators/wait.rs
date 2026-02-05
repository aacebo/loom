use loom_sync::tasks::{Task, TaskError, TaskResult};

use crate::{Build, Operator, Source};

/// Await: wait for a Task to complete and extract its result
pub struct Await;

impl Await {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Await {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Operator<Task<T>> for Await
where
    T: Send + 'static,
{
    type Output = TaskResult<T>;

    fn apply(self, src: Source<Task<T>>) -> Source<Self::Output> {
        Source::new(move || {
            let mut task = src.build();
            match task.wait() {
                Ok(result) => result,
                Err(recv_err) => TaskResult::Error(TaskError::from(recv_err)),
            }
        })
    }
}
