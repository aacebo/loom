use loom_sync::tasks::Task;

use crate::{Build, Operator, Source};

/// Spawn: execute work asynchronously, return a Task handle
pub struct Spawn<Input, Output> {
    f: Box<dyn FnOnce(Input) -> Output + Send>,
}

impl<Input, Output> Spawn<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Input) -> Output + Send + 'static,
    {
        Self { f: Box::new(f) }
    }
}

impl<Input, Output> Operator<Input> for Spawn<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    type Output = Task<Output>;

    fn apply(self, src: Source<Input>) -> Source<Self::Output> {
        Source::new(move || {
            let input = src.build();
            let f = self.f;
            loom_sync::spawn!(|| f(input))
        })
    }
}
