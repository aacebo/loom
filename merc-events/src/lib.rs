mod channel;
mod consumer;
mod event;
mod key;
mod producer;

pub use channel::*;
pub use consumer::*;
pub use event::*;
pub use key::*;
pub use producer::*;

pub fn new(uri: &str) -> ChannelConnector {
    ChannelConnector::new(uri)
}
