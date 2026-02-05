use crate::chan::error::{RecvError, SendError};

/// Errors that can occur during task execution or when awaiting a task
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError {
    /// Task was cancelled before completion
    Cancelled,

    /// Task panicked during execution
    Panic(String),

    /// Custom error with a message
    Custom(String),

    /// Task handle was dropped without sending a result
    Dropped,

    /// Failed to receive the task result
    Recv(RecvError),

    /// Failed to send the task result
    Send(SendError),
}

impl TaskError {
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled)
    }

    pub fn is_panic(&self) -> bool {
        matches!(self, Self::Panic(_))
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    pub fn is_dropped(&self) -> bool {
        matches!(self, Self::Dropped)
    }

    pub fn is_recv(&self) -> bool {
        matches!(self, Self::Recv(_))
    }

    pub fn is_send(&self) -> bool {
        matches!(self, Self::Send(_))
    }

    /// Create a custom error from any error type
    pub fn custom<E: std::error::Error>(err: E) -> Self {
        Self::Custom(err.to_string())
    }

    /// Create a panic error from panic payload
    pub fn panic<S: Into<String>>(msg: S) -> Self {
        Self::Panic(msg.into())
    }
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => write!(f, "task cancelled"),
            Self::Panic(msg) => write!(f, "task panicked: {}", msg),
            Self::Custom(msg) => write!(f, "{}", msg),
            Self::Dropped => write!(f, "task handle dropped"),
            Self::Recv(e) => write!(f, "recv error: {}", e),
            Self::Send(e) => write!(f, "send error: {}", e),
        }
    }
}

impl std::error::Error for TaskError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Recv(e) => Some(e),
            Self::Send(e) => Some(e),
            _ => None,
        }
    }
}

impl From<RecvError> for TaskError {
    fn from(err: RecvError) -> Self {
        Self::Recv(err)
    }
}

impl From<SendError> for TaskError {
    fn from(err: SendError) -> Self {
        Self::Send(err)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    // === TaskError Construction ===

    #[test]
    fn task_error_cancelled() {
        let err = TaskError::Cancelled;
        assert!(err.is_cancelled());
        assert!(!err.is_panic());
        assert!(!err.is_custom());
        assert!(!err.is_dropped());
        assert!(!err.is_recv());
        assert!(!err.is_send());
    }

    #[test]
    fn task_error_panic() {
        let err = TaskError::panic("oops");
        assert!(err.is_panic());
        assert!(!err.is_cancelled());
    }

    #[test]
    fn task_error_custom() {
        let err = TaskError::Custom("custom error".to_string());
        assert!(err.is_custom());
        assert!(!err.is_cancelled());
    }

    #[test]
    fn task_error_custom_from_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "io error");
        let err = TaskError::custom(io_err);
        assert!(err.is_custom());
        assert!(err.to_string().contains("io error"));
    }

    #[test]
    fn task_error_dropped() {
        let err = TaskError::Dropped;
        assert!(err.is_dropped());
    }

    #[test]
    fn task_error_recv() {
        let err = TaskError::Recv(RecvError::Closed);
        assert!(err.is_recv());
    }

    #[test]
    fn task_error_send() {
        let err = TaskError::Send(SendError::Closed);
        assert!(err.is_send());
    }

    // === Display ===

    #[test]
    fn display_cancelled() {
        assert_eq!(format!("{}", TaskError::Cancelled), "task cancelled");
    }

    #[test]
    fn display_panic() {
        let err = TaskError::Panic("oh no".to_string());
        assert_eq!(format!("{}", err), "task panicked: oh no");
    }

    #[test]
    fn display_custom() {
        let err = TaskError::Custom("custom msg".to_string());
        assert_eq!(format!("{}", err), "custom msg");
    }

    #[test]
    fn display_dropped() {
        assert_eq!(format!("{}", TaskError::Dropped), "task handle dropped");
    }

    #[test]
    fn display_recv() {
        let err = TaskError::Recv(RecvError::Closed);
        assert_eq!(format!("{}", err), "recv error: closed");
    }

    #[test]
    fn display_send() {
        let err = TaskError::Send(SendError::Full);
        assert_eq!(format!("{}", err), "send error: full");
    }

    // === Error Source ===

    #[test]
    fn source_recv() {
        let err = TaskError::Recv(RecvError::Closed);
        assert!(err.source().is_some());
    }

    #[test]
    fn source_send() {
        let err = TaskError::Send(SendError::Timeout);
        assert!(err.source().is_some());
    }

    #[test]
    fn source_none_for_others() {
        assert!(TaskError::Cancelled.source().is_none());
        assert!(TaskError::Panic("x".to_string()).source().is_none());
        assert!(TaskError::Custom("x".to_string()).source().is_none());
        assert!(TaskError::Dropped.source().is_none());
    }

    // === From Conversions ===

    #[test]
    fn from_recv_error() {
        let recv_err = RecvError::Closed;
        let task_err: TaskError = recv_err.into();
        assert!(task_err.is_recv());
    }

    #[test]
    fn from_send_error() {
        let send_err = SendError::Closed;
        let task_err: TaskError = send_err.into();
        assert!(task_err.is_send());
    }

    // === Equality ===

    #[test]
    fn equality() {
        assert_eq!(TaskError::Cancelled, TaskError::Cancelled);
        assert_eq!(TaskError::Dropped, TaskError::Dropped);
        assert_eq!(
            TaskError::Panic("x".to_string()),
            TaskError::Panic("x".to_string())
        );
        assert_ne!(TaskError::Cancelled, TaskError::Dropped);
    }

    #[test]
    fn clone_test() {
        let err = TaskError::Custom("test".to_string());
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn debug() {
        let err = TaskError::Cancelled;
        assert_eq!(format!("{:?}", err), "Cancelled");
    }

    #[test]
    fn debug_panic() {
        let err = TaskError::Panic("test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("Panic"));
        assert!(debug.contains("test"));
    }
}
