use std::sync::atomic::{AtomicU8, Ordering};

///
/// ## TaskStatus
/// represents the state of a Task
///
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Pending,
    Cancelled,
    Error,
    Ok,
}

impl TaskStatus {
    pub fn is_pending(&self) -> bool {
        match self {
            Self::Pending => true,
            _ => false,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        match self {
            Self::Cancelled => true,
            _ => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            Self::Error => true,
            _ => false,
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok => true,
            _ => false,
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Self::Cancelled | Self::Error | Self::Ok => true,
            _ => false,
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Pending,
            1 => Self::Cancelled,
            2 => Self::Error,
            _ => Self::Ok,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl From<AtomicU8> for TaskStatus {
    fn from(value: AtomicU8) -> Self {
        match value.load(Ordering::Relaxed) {
            0 => Self::Pending,
            1 => Self::Cancelled,
            2 => Self::Error,
            _ => Self::Ok,
        }
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Error => write!(f, "error"),
            Self::Ok => write!(f, "ok"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn default_is_pending() {
        let status = TaskStatus::default();
        assert_eq!(status, TaskStatus::Pending);
    }

    #[test]
    fn is_pending() {
        assert!(TaskStatus::Pending.is_pending());
        assert!(!TaskStatus::Ok.is_pending());
        assert!(!TaskStatus::Error.is_pending());
        assert!(!TaskStatus::Cancelled.is_pending());
    }

    #[test]
    fn is_cancelled() {
        assert!(TaskStatus::Cancelled.is_cancelled());
        assert!(!TaskStatus::Pending.is_cancelled());
    }

    #[test]
    fn is_error() {
        assert!(TaskStatus::Error.is_error());
        assert!(!TaskStatus::Pending.is_error());
    }

    #[test]
    fn is_ok() {
        assert!(TaskStatus::Ok.is_ok());
        assert!(!TaskStatus::Pending.is_ok());
    }

    #[test]
    fn is_complete() {
        assert!(TaskStatus::Ok.is_complete());
        assert!(TaskStatus::Error.is_complete());
        assert!(TaskStatus::Cancelled.is_complete());
        assert!(!TaskStatus::Pending.is_complete());
    }

    #[test]
    fn from_u8() {
        assert_eq!(TaskStatus::from_u8(0), TaskStatus::Pending);
        assert_eq!(TaskStatus::from_u8(1), TaskStatus::Cancelled);
        assert_eq!(TaskStatus::from_u8(2), TaskStatus::Error);
        assert_eq!(TaskStatus::from_u8(3), TaskStatus::Ok);
        assert_eq!(TaskStatus::from_u8(255), TaskStatus::Ok); // Default case
    }

    #[test]
    fn as_u8() {
        assert_eq!(TaskStatus::Pending.as_u8(), 0);
        assert_eq!(TaskStatus::Cancelled.as_u8(), 1);
        assert_eq!(TaskStatus::Error.as_u8(), 2);
        assert_eq!(TaskStatus::Ok.as_u8(), 3);
    }

    #[test]
    fn roundtrip_u8() {
        for status in [
            TaskStatus::Pending,
            TaskStatus::Cancelled,
            TaskStatus::Error,
            TaskStatus::Ok,
        ] {
            assert_eq!(TaskStatus::from_u8(status.as_u8()), status);
        }
    }

    #[test]
    fn from_atomic_u8() {
        let atomic = AtomicU8::new(0);
        assert_eq!(TaskStatus::from(atomic), TaskStatus::Pending);

        let atomic = AtomicU8::new(1);
        assert_eq!(TaskStatus::from(atomic), TaskStatus::Cancelled);

        let atomic = AtomicU8::new(2);
        assert_eq!(TaskStatus::from(atomic), TaskStatus::Error);

        let atomic = AtomicU8::new(3);
        assert_eq!(TaskStatus::from(atomic), TaskStatus::Ok);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", TaskStatus::Pending), "pending");
        assert_eq!(format!("{}", TaskStatus::Cancelled), "cancelled");
        assert_eq!(format!("{}", TaskStatus::Error), "error");
        assert_eq!(format!("{}", TaskStatus::Ok), "ok");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", TaskStatus::Pending), "Pending");
        assert_eq!(format!("{:?}", TaskStatus::Cancelled), "Cancelled");
        assert_eq!(format!("{:?}", TaskStatus::Error), "Error");
        assert_eq!(format!("{:?}", TaskStatus::Ok), "Ok");
    }

    #[test]
    fn equality_and_hash() {
        assert_eq!(TaskStatus::Pending, TaskStatus::Pending);
        assert_ne!(TaskStatus::Pending, TaskStatus::Ok);

        let mut set = HashSet::new();
        set.insert(TaskStatus::Pending);
        set.insert(TaskStatus::Ok);
        set.insert(TaskStatus::Error);
        set.insert(TaskStatus::Cancelled);
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn clone_and_copy() {
        let status = TaskStatus::Ok;
        let cloned = status.clone();
        let copied: TaskStatus = status;

        assert_eq!(status, cloned);
        assert_eq!(status, copied);
    }
}
