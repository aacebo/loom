use crate::tasks::TaskError;

pub enum TaskResult<T: Send + 'static> {
    Cancelled,
    Error(TaskError),
    Ok(T),
}

impl<T: Send + 'static> TaskResult<T> {
    pub fn is_cancelled(&self) -> bool {
        match self {
            Self::Cancelled => true,
            _ => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            Self::Error(_) => true,
            _ => false,
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            _ => false,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Cancelled => panic!("called `TaskResult::unwrap()` on a `Cancelled` value"),
            Self::Error(err) => panic!("called `TaskResult::unwrap()` on an `Error` value: {err}"),
        }
    }

    pub fn unwrap_err(self) -> TaskError {
        match self {
            Self::Error(err) => err,
            Self::Ok(_) => panic!("called `TaskResult::unwrap_err()` on an `Ok` value"),
            Self::Cancelled => panic!("called `TaskResult::unwrap_err()` on a `Cancelled` value"),
        }
    }
}

impl<T: Send + 'static> std::fmt::Debug for TaskResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => write!(f, "TaskResult::Cancelled"),
            Self::Error(err) => f.debug_tuple("TaskResult::Err").field(err).finish(),
            Self::Ok(_) => f.debug_tuple("TaskResult::Ok").field(&"<value>").finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_ok() {
        let result: TaskResult<i32> = TaskResult::Ok(42);
        assert!(result.is_ok());
        assert!(!result.is_error());
        assert!(!result.is_cancelled());
    }

    #[test]
    fn result_error() {
        let result: TaskResult<i32> = TaskResult::Error(TaskError::Custom("err".to_string()));
        assert!(result.is_error());
        assert!(!result.is_ok());
        assert!(!result.is_cancelled());
    }

    #[test]
    fn result_cancelled() {
        let result: TaskResult<i32> = TaskResult::Cancelled;
        assert!(result.is_cancelled());
        assert!(!result.is_ok());
        assert!(!result.is_error());
    }

    #[test]
    fn unwrap_ok() {
        let result: TaskResult<i32> = TaskResult::Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "Cancelled")]
    fn unwrap_cancelled_panics() {
        let result: TaskResult<i32> = TaskResult::Cancelled;
        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "Error")]
    fn unwrap_error_panics() {
        let result: TaskResult<i32> = TaskResult::Error(TaskError::Custom("err".to_string()));
        result.unwrap();
    }

    #[test]
    fn unwrap_err_returns_error() {
        let result: TaskResult<i32> = TaskResult::Error(TaskError::Cancelled);
        let err = result.unwrap_err();
        assert!(err.is_cancelled());
    }

    #[test]
    #[should_panic(expected = "Ok")]
    fn unwrap_err_on_ok_panics() {
        let result: TaskResult<i32> = TaskResult::Ok(42);
        result.unwrap_err();
    }

    #[test]
    #[should_panic(expected = "Cancelled")]
    fn unwrap_err_on_cancelled_panics() {
        let result: TaskResult<i32> = TaskResult::Cancelled;
        result.unwrap_err();
    }

    #[test]
    fn debug_ok() {
        let result: TaskResult<i32> = TaskResult::Ok(42);
        let debug = format!("{:?}", result);
        assert!(debug.contains("TaskResult::Ok"));
    }

    #[test]
    fn debug_error() {
        let result: TaskResult<i32> = TaskResult::Error(TaskError::Cancelled);
        let debug = format!("{:?}", result);
        assert!(debug.contains("TaskResult::Err"));
    }

    #[test]
    fn debug_cancelled() {
        let result: TaskResult<i32> = TaskResult::Cancelled;
        let debug = format!("{:?}", result);
        assert!(debug.contains("TaskResult::Cancelled"));
    }
}
