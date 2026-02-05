use crate::{Build, Operator, Source};

/// Filter items in a Vec based on a predicate
pub struct Filter<T> {
    predicate: Box<dyn Fn(&T) -> bool + Send + Sync>,
}

impl<T> Filter<T>
where
    T: Send + 'static,
{
    pub fn new<P>(predicate: P) -> Self
    where
        P: Fn(&T) -> bool + Send + Sync + 'static,
    {
        Self {
            predicate: Box::new(predicate),
        }
    }
}

impl<T> Operator<Vec<T>> for Filter<T>
where
    T: Send + 'static,
{
    type Output = Vec<T>;

    fn apply(self, src: Source<Vec<T>>) -> Source<Self::Output> {
        Source::new(move || {
            let items = src.build();
            items
                .into_iter()
                .filter(|item| (self.predicate)(item))
                .collect()
        })
    }
}
