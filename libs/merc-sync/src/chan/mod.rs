pub mod error;
mod status;

#[cfg(feature = "tokio")]
pub mod tokio;

pub use status::*;

use async_trait::async_trait;

pub trait Channel {
    fn status(&self) -> Status;
    fn len(&self) -> usize;
    fn capacity(&self) -> Option<usize>;
}

#[async_trait]
pub trait Sender: Channel + Send + Sync + 'static {
    type Item;

    async fn send(&self, item: Self::Item) -> Result<(), error::SendError>;
}

#[async_trait]
pub trait Receiver: Channel + Send + 'static {
    type Item;

    async fn recv(&self) -> Result<Self::Item, error::RecvError>;
}
