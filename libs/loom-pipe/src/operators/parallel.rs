use loom_sync::tasks::{Task, TaskError, TaskResult};

use crate::{Build, Operator, Source};

/// Parallel: execute multiple operators concurrently using tasks
/// Unlike FanOut which executes sequentially, Parallel spawns tasks for each branch
pub struct Parallel<Input, Output> {
    branches: Vec<Box<dyn FnOnce(Input) -> Output + Send>>,
    _marker: std::marker::PhantomData<fn(Input) -> Output>,
}

impl<Input, Output> Parallel<Input, Output>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
{
    pub fn new() -> Self {
        Self {
            branches: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn add<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Input) -> Output + Send + 'static,
    {
        self.branches.push(Box::new(f));
        self
    }
}

impl<Input, Output> Default for Parallel<Input, Output>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Input, Output> Operator<Input> for Parallel<Input, Output>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
{
    type Output = Vec<TaskResult<Output>>;

    fn apply(self, src: Source<Input>) -> Source<Self::Output> {
        Source::new(move || {
            let input = src.build();

            // Spawn all branches as tasks
            let tasks: Vec<Task<Output>> = self
                .branches
                .into_iter()
                .map(|f| {
                    let cloned = input.clone();
                    loom_sync::spawn!(|| f(cloned))
                })
                .collect();

            // Wait for all tasks to complete
            tasks
                .into_iter()
                .map(|mut t| match t.wait() {
                    Ok(result) => result,
                    Err(recv_err) => TaskResult::Error(TaskError::from(recv_err)),
                })
                .collect()
        })
    }
}
