///
/// ## TaskStatus
/// represents the state of a Task
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Pending,
    Running,
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

    pub fn is_running(&self) -> bool {
        match self {
            Self::Running => true,
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
            Self::Running => write!(f, "running"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Error => write!(f, "error"),
            Self::Ok => write!(f, "ok"),
        }
    }
}
