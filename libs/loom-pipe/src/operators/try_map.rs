use loom_error::Result;

use crate::{Build, Operator, Source};

/// TryMap: transform input with a fallible function
pub struct TryMap<Input, Output> {
    f: Box<dyn FnOnce(Input) -> Result<Output> + Send>,
}

impl<Input, Output> TryMap<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Input) -> Result<Output> + Send + 'static,
    {
        Self { f: Box::new(f) }
    }
}

impl<Input, Output> Operator<Input> for TryMap<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    type Output = Result<Output>;

    fn apply(self, src: Source<Input>) -> Source<Self::Output> {
        Source::new(move || (self.f)(src.build()))
    }
}
