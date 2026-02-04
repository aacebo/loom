use futures_lite::StreamExt;
use loom_error::Result;

use crate::{Event, Socket};

#[derive(Clone)]
pub struct SocketConsumer<'a> {
    pub(crate) socket: &'a Socket,
    pub(crate) consumer: lapin::Consumer,
}

impl<'a> SocketConsumer<'a> {
    pub fn socket(&self) -> &'a Socket {
        &self.socket
    }

    pub async fn dequeue<T: for<'b> serde::Deserialize<'b>>(
        &mut self,
    ) -> Option<Result<(lapin::message::Delivery, Event<T>)>> {
        let delivery = match self.consumer.next().await? {
            Err(err) => return Some(Err(err.into())),
            Ok(v) => v,
        };

        let data: Event<T> = match serde_json::from_slice(&delivery.data) {
            Err(err) => return Some(Err(err.into())),
            Ok(v) => v,
        };

        Some(Ok((delivery, data)))
    }
}
