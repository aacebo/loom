use chrono::{DateTime, Utc};
use sqlx::PgPool;

use events::Socket;
use storage::Storage;

#[derive(Clone)]
pub struct Context {
    pool: PgPool,
    amqp: Socket,
    start_time: DateTime<Utc>,
}

impl Context {
    pub fn new(pool: PgPool, amqp: Socket) -> Self {
        Self {
            pool,
            amqp,
            start_time: Utc::now(),
        }
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn storage(&self) -> Storage<'_> {
        Storage::new(&self.pool)
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn amqp(&self) -> &Socket {
        &self.amqp
    }
}
