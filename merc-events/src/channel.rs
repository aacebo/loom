use std::collections::HashMap;

use lapin::{Channel, Connection, ConnectionProperties, options, types};
use merc_error::{Error, Result};

use crate::{Consumer, Key, Producer};

pub struct ChannelConnection {
    app_id: String,
    conn: Connection,
    channel: Channel,
    queues: HashMap<Key, lapin::Queue>,
}

impl ChannelConnection {
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    pub fn queue(&self, key: Key) -> Option<&lapin::Queue> {
        self.queues.get(&key)
    }

    pub async fn consume(self, key: Key) -> Result<Consumer> {
        if !self.queues.contains_key(&key) {
            return Err(Error::builder().message("queue not found").build());
        }

        Consumer::connect(self, key.queue()).await
    }

    pub fn produce(self) -> Producer {
        Producer::connect(self)
    }
}

pub struct ChannelConnector {
    app_id: String,
    uri: String,
    queues: Vec<Key>,
}

impl ChannelConnector {
    pub fn new(uri: &str) -> Self {
        Self {
            app_id: String::new(),
            uri: uri.to_string(),
            queues: vec![],
        }
    }

    pub fn with_app_id(mut self, app_id: &str) -> Self {
        self.app_id = app_id.to_string();
        self
    }

    pub fn with_queue(mut self, key: Key) -> Self {
        self.queues.push(key);
        self
    }

    pub async fn connect(self) -> Result<ChannelConnection> {
        let conn = Connection::connect(&self.uri, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        let mut queues = HashMap::new();

        for key in self.queues {
            channel
                .exchange_declare(
                    key.exchange(),
                    lapin::ExchangeKind::Topic,
                    options::ExchangeDeclareOptions::default(),
                    types::FieldTable::default(),
                )
                .await?;

            let queue = channel
                .queue_declare(
                    key.queue(),
                    options::QueueDeclareOptions::default(),
                    types::FieldTable::default(),
                )
                .await?;

            channel
                .queue_bind(
                    key.queue(),
                    key.exchange(),
                    &key.to_string(),
                    options::QueueBindOptions::default(),
                    types::FieldTable::default(),
                )
                .await?;

            queues.insert(key, queue);
        }

        Ok(ChannelConnection {
            app_id: self.app_id,
            conn,
            channel,
            queues,
        })
    }
}
