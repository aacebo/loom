use crate::{Build, Operator, Source};

/// Fan-out: send the same input to multiple operators, collect all outputs
pub struct FanOut<Input, Output> {
    branches: Vec<Box<dyn FnOnce(Source<Input>) -> Source<Output> + Send>>,
    _marker: std::marker::PhantomData<fn(Input) -> Output>,
}

impl<Input, Output> FanOut<Input, Output>
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

    pub fn add<Op>(mut self, op: Op) -> Self
    where
        Op: Operator<Input, Output = Output> + Send + 'static,
    {
        self.branches
            .push(Box::new(move |src: Source<Input>| op.apply(src)));
        self
    }
}

impl<Input, Output> Default for FanOut<Input, Output>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Input, Output> Operator<Input> for FanOut<Input, Output>
where
    Input: Clone + Send + 'static,
    Output: Send + 'static,
{
    type Output = Vec<Output>;

    fn apply(self, src: Source<Input>) -> Source<Self::Output> {
        Source::new(move || {
            let input = src.build();
            self.branches
                .into_iter()
                .map(|branch| {
                    let cloned_input = input.clone();
                    branch(Source::from(cloned_input)).build()
                })
                .collect()
        })
    }
}
