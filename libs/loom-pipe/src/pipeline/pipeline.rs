use std::any::Any;

use loom_error::{Error, ErrorCode, Result};

use super::AnyLayer;

/// Internal stage representation
pub enum PipelineStage {
    Layer(Box<dyn AnyLayer>),
}

/// Compiled pipeline ready for execution
pub struct Pipeline<Input, Output> {
    stages: Vec<PipelineStage>,
    _marker: std::marker::PhantomData<(Input, Output)>,
}

impl<Input: Send + 'static, Output: Send + 'static> Pipeline<Input, Output> {
    pub(crate) fn new(stages: Vec<PipelineStage>) -> Self {
        Self {
            stages,
            _marker: std::marker::PhantomData,
        }
    }

    /// Execute pipeline synchronously
    pub fn execute(&self, input: Input) -> Result<Output> {
        let mut current: Box<dyn Any + Send> = Box::new(input);

        for stage in &self.stages {
            current = match stage {
                PipelineStage::Layer(layer) => layer.process_any(current)?,
            };
        }

        current.downcast::<Output>().map(|b| *b).map_err(|_| {
            Error::builder()
                .code(ErrorCode::Unknown)
                .message("Pipeline output type mismatch")
                .build()
        })
    }

    /// Get the number of stages in the pipeline
    pub fn len(&self) -> usize {
        self.stages.len()
    }

    /// Check if the pipeline is empty
    pub fn is_empty(&self) -> bool {
        self.stages.is_empty()
    }
}
