use std::sync::Arc;

use lapin::{options, protocol};
use merc_error::Result;

use crate::{ChannelConnection, Event};

#[derive(Clone)]
pub struct Producer {
    conn: Arc<ChannelConnection>,
}

impl Producer {
    pub fn connect(conn: ChannelConnection) -> Self {
        Self {
            conn: Arc::new(conn),
        }
    }

    pub fn conn(&self) -> &ChannelConnection {
        &self.conn
    }

    pub async fn enqueue<TBody: serde::Serialize>(&self, event: Event<TBody>) -> Result<()> {
        let payload = serde_json::to_vec(&event)?;
        let _ = self
            .conn
            .channel()
            .basic_publish(
                event.key.exchange(),
                &event.key.to_string(),
                options::BasicPublishOptions::default(),
                &payload,
                protocol::basic::AMQPProperties::default()
                    .with_app_id(self.conn().app_id().into())
                    .with_content_type("application/json".into()),
            )
            .await?;

        Ok(())
    }
}
