use std::sync::Arc;

use futures_lite::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable};
use merc_error::Result;

use crate::{ChannelConnection, Event};

#[derive(Clone)]
pub struct Consumer {
    conn: Arc<ChannelConnection>,
    consumer: lapin::Consumer,
}

impl Consumer {
    pub async fn connect(conn: ChannelConnection, queue: &str) -> Result<Self> {
        let consumer = conn
            .channel()
            .basic_consume(
                queue,
                conn.app_id(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(Self {
            conn: Arc::new(conn),
            consumer,
        })
    }

    pub fn conn(&self) -> &ChannelConnection {
        &self.conn
    }

    pub async fn dequeue<T: for<'a> serde::Deserialize<'a>>(
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
