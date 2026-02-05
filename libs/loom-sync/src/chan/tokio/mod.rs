mod receiver;
mod sender;

pub use receiver::*;
pub use sender::*;

/// Create a channel for async communication.
///
/// # Patterns
/// - `open!()` - unbounded channel
/// - `open!(capacity)` - bounded channel with specified capacity
///
/// # Examples
/// ```ignore
/// let (tx, rx) = open!();        // unbounded
/// let (tx, rx) = open!(100);     // bounded with capacity 100
/// ```
#[macro_export]
macro_rules! open {
    () => {{
        let (sender, receiver) = $crate::internal::tokio::sync::mpsc::unbounded_channel();
        (
            $crate::chan::tokio::TokioSender::new($crate::chan::tokio::MpscSender::from(sender)),
            $crate::chan::tokio::TokioReceiver::new($crate::chan::tokio::MpscReceiver::from(
                receiver,
            )),
        )
    }};
    ($capacity:expr) => {{
        let (sender, receiver) = $crate::internal::tokio::sync::mpsc::channel($capacity);
        (
            $crate::chan::tokio::TokioSender::new($crate::chan::tokio::MpscSender::from(sender)),
            $crate::chan::tokio::TokioReceiver::new($crate::chan::tokio::MpscReceiver::from(
                receiver,
            )),
        )
    }};
}
