pub mod chan;
pub mod tasks;

/// Re-exported dependencies for macro use.
/// Not intended for direct use by consumers.
#[doc(hidden)]
pub mod internal {
    #[cfg(feature = "tokio")]
    pub use futures;
    #[cfg(feature = "tokio")]
    pub use tokio;
}
