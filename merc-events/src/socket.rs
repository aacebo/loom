use std::{collections::HashMap, sync::Arc};

use lapin::{Channel, Connection, ConnectionProperties, options, types};
use merc_error::{Error, Result};

use crate::{Key, SocketConsumer, SocketProducer};

#[derive(Clone)]
pub struct Socket {
    app_id: String,
    conn: Arc<Connection>,
    channel: Arc<Channel>,
    queues: HashMap<Key, lapin::Queue>,
}

impl Socket {
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

    pub async fn consume(&self, key: Key) -> Result<SocketConsumer<'_>> {
        if !self.queues.contains_key(&key) {
            return Err(Error::builder().message("queue not found").build());
        }

        let consumer = self
            .channel()
            .basic_consume(
                key.queue(),
                self.app_id(),
                options::BasicConsumeOptions::default(),
                types::FieldTable::default(),
            )
            .await?;

        Ok(SocketConsumer {
            socket: self,
            consumer,
        })
    }

    pub fn produce(&self) -> SocketProducer<'_> {
        SocketProducer { socket: self }
    }
}

pub struct SocketOptions {
    app_id: String,
    uri: String,
    queues: Vec<Key>,
}

impl SocketOptions {
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

    pub async fn connect(self) -> Result<Socket> {
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

        Ok(Socket {
            app_id: self.app_id,
            conn: Arc::new(conn),
            channel: Arc::new(channel),
            queues,
        })
    }
}
