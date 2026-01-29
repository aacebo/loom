mod receiver;
mod sender;

use std::sync::Arc;

pub use receiver::*;
pub use sender::*;

use tokio::sync::mpsc;

use crate::chan::{Channel, State, Status};

pub fn open<T: std::fmt::Debug>() -> TokioChannel<T> {
    TokioChannel::new()
}

pub fn alloc<T: std::fmt::Debug>(capacity: usize) -> TokioChannel<T> {
    TokioChannel::bound(capacity)
}

#[derive(Debug)]
pub struct TokioChannel<T: std::fmt::Debug> {
    status: Status,
    sender: MpscSender<T>,
    receiver: MpscReceiver<T>,
}

impl<T: std::fmt::Debug> TokioChannel<T> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let status = Status::default().with_state(State::Open);

        Self {
            status,
            sender: MpscSender::from(sender),
            receiver: MpscReceiver::from(receiver),
        }
    }

    pub fn bound(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        let status = Status::bound(capacity).with_state(State::Open);

        Self {
            status,
            sender: MpscSender::from(sender),
            receiver: MpscReceiver::from(receiver),
        }
    }

    pub fn sender(self) -> TokioSender<T> {
        TokioSender::new(Arc::new(self))
    }

    pub fn receiver(self) -> TokioReceiver<T> {
        TokioReceiver::new(Arc::new(self))
    }
}

impl<T: std::fmt::Debug> Channel for TokioChannel<T> {
    fn status(&self) -> super::Status {
        self.status
    }
}
