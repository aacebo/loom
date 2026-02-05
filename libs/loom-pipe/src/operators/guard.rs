use crate::{Build, Operator, Source};

/// Guard: conditionally allow or block pipeline continuation
/// Returns Option<T> - Some(input) if allowed, None if blocked
pub struct Guard<T> {
    predicate: Box<dyn Fn(&T) -> bool + Send + Sync>,
}

impl<T> Guard<T>
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

    /// Create a guard that allows values matching the predicate
    pub fn allow<P>(predicate: P) -> Self
    where
        P: Fn(&T) -> bool + Send + Sync + 'static,
    {
        Self::new(predicate)
    }

    /// Create a guard that blocks values matching the predicate
    pub fn block<P>(predicate: P) -> Self
    where
        P: Fn(&T) -> bool + Send + Sync + 'static,
    {
        Self::new(move |x| !predicate(x))
    }
}

impl<T> Operator<T> for Guard<T>
where
    T: Send + 'static,
{
    type Output = Option<T>;

    fn apply(self, src: Source<T>) -> Source<Self::Output> {
        Source::new(move || {
            let input = src.build();
            if (self.predicate)(&input) {
                Some(input)
            } else {
                None
            }
        })
    }
}
