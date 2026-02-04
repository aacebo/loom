use lapin::{options, protocol};
use loom_error::Result;

use crate::{Event, Socket};

#[derive(Clone)]
pub struct SocketProducer<'a> {
    pub(crate) socket: &'a Socket,
}

impl<'a> SocketProducer<'a> {
    pub fn socket(&self) -> &'a Socket {
        &self.socket
    }

    pub async fn enqueue<TBody: serde::Serialize>(&self, event: Event<TBody>) -> Result<()> {
        let payload = serde_json::to_vec(&event)?;
        let _ = self
            .socket()
            .channel()
            .basic_publish(
                event.key.exchange(),
                &event.key.to_string(),
                options::BasicPublishOptions::default(),
                &payload,
                protocol::basic::AMQPProperties::default()
                    .with_app_id(self.socket().app_id().into())
                    .with_content_type("application/json".into()),
            )
            .await?;

        Ok(())
    }
}
