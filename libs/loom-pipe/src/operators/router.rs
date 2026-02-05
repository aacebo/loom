use crate::{Build, Operator, Source};

/// Route: send input to one of several operators based on predicates
pub struct Router<Input, Output> {
    routes: Vec<(
        Box<dyn Fn(&Input) -> bool + Send + Sync>,
        Box<dyn FnOnce(Source<Input>) -> Source<Output> + Send>,
    )>,
    default: Option<Box<dyn FnOnce(Source<Input>) -> Source<Output> + Send>>,
}

impl<Input, Output> Router<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            default: None,
        }
    }

    pub fn route<P, Op>(mut self, predicate: P, op: Op) -> Self
    where
        P: Fn(&Input) -> bool + Send + Sync + 'static,
        Op: Operator<Input, Output = Output> + Send + 'static,
    {
        self.routes.push((
            Box::new(predicate),
            Box::new(move |src: Source<Input>| op.apply(src)),
        ));
        self
    }

    pub fn default<Op>(mut self, op: Op) -> Self
    where
        Op: Operator<Input, Output = Output> + Send + 'static,
    {
        self.default = Some(Box::new(move |src: Source<Input>| op.apply(src)));
        self
    }
}

impl<Input, Output> Default for Router<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Input, Output> Operator<Input> for Router<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    type Output = Option<Output>;

    fn apply(mut self, src: Source<Input>) -> Source<Self::Output> {
        Source::new(move || {
            let input = src.build();

            // Find matching route
            for (predicate, route_fn) in self.routes.into_iter() {
                if predicate(&input) {
                    let output = route_fn(Source::from(input)).build();
                    return Some(output);
                }
            }

            // Try default
            if let Some(default_fn) = self.default.take() {
                let output = default_fn(Source::from(input)).build();
                return Some(output);
            }

            None
        })
    }
}
