#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ChanError {
    Send(SendError),
    Recv(RecvError),
}

impl std::fmt::Display for ChanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Send(v) => write!(f, "{}", v),
            Self::Recv(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for ChanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Send(err) => Some(err),
            Self::Recv(err) => Some(err),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SendError {
    /// the channel is closed
    Closed,

    /// the channel is full
    Full,

    /// timeout
    Timeout,
}

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed => write!(f, "closed"),
            Self::Full => write!(f, "full"),
            Self::Timeout => write!(f, "timeout"),
        }
    }
}

impl std::error::Error for SendError {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RecvError {
    /// the channel is closed
    Closed,

    /// the channel is empty (no messages available)
    Empty,
}

impl std::fmt::Display for RecvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed => write!(f, "closed"),
            Self::Empty => write!(f, "empty"),
        }
    }
}

impl std::error::Error for RecvError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::error::Error;

    // === SendError Tests ===

    #[test]
    fn send_error_display_closed() {
        assert_eq!(format!("{}", SendError::Closed), "closed");
    }

    #[test]
    fn send_error_display_full() {
        assert_eq!(format!("{}", SendError::Full), "full");
    }

    #[test]
    fn send_error_display_timeout() {
        assert_eq!(format!("{}", SendError::Timeout), "timeout");
    }

    #[test]
    fn send_error_debug() {
        assert_eq!(format!("{:?}", SendError::Closed), "Closed");
        assert_eq!(format!("{:?}", SendError::Full), "Full");
        assert_eq!(format!("{:?}", SendError::Timeout), "Timeout");
    }

    #[test]
    fn send_error_equality() {
        assert_eq!(SendError::Closed, SendError::Closed);
        assert_eq!(SendError::Full, SendError::Full);
        assert_eq!(SendError::Timeout, SendError::Timeout);
        assert_ne!(SendError::Closed, SendError::Full);
        assert_ne!(SendError::Closed, SendError::Timeout);
        assert_ne!(SendError::Full, SendError::Timeout);
    }

    #[test]
    fn send_error_clone() {
        let err = SendError::Closed;
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn send_error_copy() {
        let err = SendError::Full;
        let copied: SendError = err;
        assert_eq!(err, copied);
    }

    #[test]
    fn send_error_hash() {
        let mut set = HashSet::new();
        set.insert(SendError::Closed);
        set.insert(SendError::Full);
        set.insert(SendError::Timeout);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn send_error_is_error_trait() {
        let err: &dyn std::error::Error = &SendError::Closed;
        assert!(err.source().is_none());
    }

    // === RecvError Tests ===

    #[test]
    fn recv_error_display_closed() {
        assert_eq!(format!("{}", RecvError::Closed), "closed");
    }

    #[test]
    fn recv_error_display_empty() {
        assert_eq!(format!("{}", RecvError::Empty), "empty");
    }

    #[test]
    fn recv_error_debug() {
        assert_eq!(format!("{:?}", RecvError::Closed), "Closed");
        assert_eq!(format!("{:?}", RecvError::Empty), "Empty");
    }

    #[test]
    fn recv_error_equality() {
        assert_eq!(RecvError::Closed, RecvError::Closed);
        assert_eq!(RecvError::Empty, RecvError::Empty);
        assert_ne!(RecvError::Closed, RecvError::Empty);
    }

    #[test]
    fn recv_error_clone() {
        let err = RecvError::Closed;
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn recv_error_copy() {
        let err = RecvError::Empty;
        let copied: RecvError = err;
        assert_eq!(err, copied);
    }

    #[test]
    fn recv_error_hash() {
        let mut set = HashSet::new();
        set.insert(RecvError::Closed);
        set.insert(RecvError::Empty);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn recv_error_is_error_trait() {
        let err: &dyn std::error::Error = &RecvError::Closed;
        assert!(err.source().is_none());
    }

    // === ChanError Tests ===

    #[test]
    fn chan_error_send_display() {
        let err = ChanError::Send(SendError::Closed);
        assert_eq!(format!("{}", err), "closed");
    }

    #[test]
    fn chan_error_recv_display() {
        let err = ChanError::Recv(RecvError::Empty);
        assert_eq!(format!("{}", err), "empty");
    }

    #[test]
    fn chan_error_debug() {
        let err = ChanError::Send(SendError::Full);
        assert!(format!("{:?}", err).contains("Send"));
        assert!(format!("{:?}", err).contains("Full"));
    }

    #[test]
    fn chan_error_equality() {
        assert_eq!(
            ChanError::Send(SendError::Closed),
            ChanError::Send(SendError::Closed)
        );
        assert_ne!(
            ChanError::Send(SendError::Closed),
            ChanError::Recv(RecvError::Closed)
        );
    }

    #[test]
    fn chan_error_source_send() {
        let err = ChanError::Send(SendError::Closed);
        assert!(err.source().is_some());
    }

    #[test]
    fn chan_error_source_recv() {
        let err = ChanError::Recv(RecvError::Empty);
        assert!(err.source().is_some());
    }

    #[test]
    fn chan_error_clone() {
        let err = ChanError::Send(SendError::Timeout);
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn chan_error_copy() {
        let err = ChanError::Recv(RecvError::Closed);
        let copied: ChanError = err;
        assert_eq!(err, copied);
    }

    #[test]
    fn chan_error_hash() {
        let mut set = HashSet::new();
        set.insert(ChanError::Send(SendError::Closed));
        set.insert(ChanError::Recv(RecvError::Closed));
        assert_eq!(set.len(), 2);
    }
}
