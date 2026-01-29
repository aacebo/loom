#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum State {
    /// Senders may still send (subject to capacity).
    Open,

    /// No new sends possible, but receiver may still yield buffered messages.
    Draining,

    /// Closed and empty; receiver will never yield another message.
    Closed,
}

impl State {
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

impl Default for State {
    fn default() -> Self {
        Self::Closed
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Draining => write!(f, "draining"),
            Self::Closed => write!(f, "closed"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Status {
    /// channel open/closed
    state: State,

    /// Messages currently buffered (0 if unknown).
    len: usize,

    /// Capacity (None = unbounded or unknown).
    capacity: Option<usize>,
}

impl Status {
    pub fn bound(capacity: usize) -> Self {
        Self {
            capacity: Some(capacity),
            ..Default::default()
        }
    }

    pub fn is_full(&self) -> bool {
        match self.capacity {
            None => false,
            Some(cap) => self.len == cap,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn with_state(mut self, state: State) -> Self {
        self.state = state;
        self
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn with_len(mut self, len: usize) -> Self {
        self.len = len;
        self
    }

    pub fn capacity(&self) -> Option<usize> {
        self.capacity
    }
}

// impl std::ops::Deref for Status {
//     type Target = State;

//     fn deref(&self) -> &Self::Target {
//         &self.state
//     }
// }
