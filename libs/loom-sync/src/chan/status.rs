#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    /// Senders may still send (subject to capacity).
    Open,

    /// No new sends possible, but receiver may still yield buffered messages.
    Draining,

    /// Closed and empty; receiver will never yield another message.
    Closed,
}

impl Status {
    pub fn is_open(&self) -> bool {
        match self {
            Self::Open => true,
            _ => false,
        }
    }

    pub fn is_draining(&self) -> bool {
        match self {
            Self::Draining => true,
            _ => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            Self::Closed => true,
            _ => false,
        }
    }

    pub fn is_closing(&self) -> bool {
        match self {
            Self::Draining | Self::Closed => true,
            _ => false,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::Closed
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Draining => write!(f, "draining"),
            Self::Closed => write!(f, "closed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // === Default ===

    #[test]
    fn default_is_closed() {
        assert_eq!(Status::default(), Status::Closed);
    }

    // === is_open ===

    #[test]
    fn is_open_true_for_open() {
        assert!(Status::Open.is_open());
    }

    #[test]
    fn is_open_false_for_draining() {
        assert!(!Status::Draining.is_open());
    }

    #[test]
    fn is_open_false_for_closed() {
        assert!(!Status::Closed.is_open());
    }

    // === is_draining ===

    #[test]
    fn is_draining_true_for_draining() {
        assert!(Status::Draining.is_draining());
    }

    #[test]
    fn is_draining_false_for_open() {
        assert!(!Status::Open.is_draining());
    }

    #[test]
    fn is_draining_false_for_closed() {
        assert!(!Status::Closed.is_draining());
    }

    // === is_closed ===

    #[test]
    fn is_closed_true_for_closed() {
        assert!(Status::Closed.is_closed());
    }

    #[test]
    fn is_closed_false_for_open() {
        assert!(!Status::Open.is_closed());
    }

    #[test]
    fn is_closed_false_for_draining() {
        assert!(!Status::Draining.is_closed());
    }

    // === is_closing ===

    #[test]
    fn is_closing_true_for_draining() {
        assert!(Status::Draining.is_closing());
    }

    #[test]
    fn is_closing_true_for_closed() {
        assert!(Status::Closed.is_closing());
    }

    #[test]
    fn is_closing_false_for_open() {
        assert!(!Status::Open.is_closing());
    }

    // === Display ===

    #[test]
    fn display_open() {
        assert_eq!(format!("{}", Status::Open), "open");
    }

    #[test]
    fn display_draining() {
        assert_eq!(format!("{}", Status::Draining), "draining");
    }

    #[test]
    fn display_closed() {
        assert_eq!(format!("{}", Status::Closed), "closed");
    }

    // === Debug ===

    #[test]
    fn debug_format() {
        assert_eq!(format!("{:?}", Status::Open), "Open");
        assert_eq!(format!("{:?}", Status::Draining), "Draining");
        assert_eq!(format!("{:?}", Status::Closed), "Closed");
    }

    // === Clone and Copy ===

    #[test]
    fn clone_works() {
        let status = Status::Open;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn copy_works() {
        let status = Status::Draining;
        let copied: Status = status;
        assert_eq!(status, copied);
    }

    // === Equality ===

    #[test]
    fn equality() {
        assert_eq!(Status::Open, Status::Open);
        assert_eq!(Status::Draining, Status::Draining);
        assert_eq!(Status::Closed, Status::Closed);
        assert_ne!(Status::Open, Status::Closed);
        assert_ne!(Status::Open, Status::Draining);
        assert_ne!(Status::Draining, Status::Closed);
    }

    // === Hash ===

    #[test]
    fn hash_all_variants_unique() {
        let mut set = HashSet::new();
        set.insert(Status::Open);
        set.insert(Status::Draining);
        set.insert(Status::Closed);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn hash_contains() {
        let mut set = HashSet::new();
        set.insert(Status::Open);
        assert!(set.contains(&Status::Open));
        assert!(!set.contains(&Status::Closed));
    }
}
