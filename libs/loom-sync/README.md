# loom-sync

Synchronization primitives for the Loom ecosystem.

## Features

- `tokio` - Tokio async runtime support

## Key Types

### Task

A reference to an async operation that will eventually resolve to a value:

```rust
use loom_sync::tasks::Task;

let (task, resolver): (Task<i32>, _) = spawn!();

// From another thread/task
resolver.ok(42).unwrap();

// Wait for result
let result = task.await; // or task.wait() for sync
assert_eq!(result.unwrap(), 42);
```

### TaskStatus

Represents the state of a Task:

```rust
pub enum TaskStatus {
    Pending,    // Not yet started/completed
    Cancelled,  // Cancelled before completion
    Error,      // Completed with error
    Ok,         // Completed successfully
}
```

Methods: `is_pending()`, `is_cancelled()`, `is_error()`, `is_ok()`, `is_complete()`

### TaskId

Unique identifier for each task.

### TaskResolver

Used to complete a Task from another thread/task:

```rust
resolver.ok(value);      // Complete successfully
resolver.error(msg);     // Complete with error
resolver.cancel();       // Cancel the task
```

## Channel Module

### Channel Traits

```rust
pub trait Channel {
    fn status(&self) -> Status;
    fn len(&self) -> usize;
    fn capacity(&self) -> Option<usize>;
}

pub trait Sender: Channel + Send + Sync {
    type Item: Send;
    fn send(&self, item: Self::Item) -> Result<(), SendError>;
}

pub trait Receiver: Channel + Send {
    type Item: Send;
    fn close(&mut self);
    fn recv(&mut self) -> Result<Self::Item, RecvError>;
}
```

### Async Variants

```rust
#[async_trait]
pub trait AsyncSender: Sender {
    async fn send_async(&self, item: Self::Item) -> Result<(), SendError>;
}

#[async_trait]
pub trait AsyncReceiver: Receiver {
    async fn recv_async(&mut self) -> Result<Self::Item, RecvError>;
}
```

### Channel Status

```rust
pub enum Status {
    Open,    // Channel is open
    Closed,  // Channel is closed
}
```

## Usage

```toml
[dependencies]
loom-sync = { version = "0.0.1", features = ["tokio"] }
```

```rust
use loom_sync::tasks::{Task, TaskStatus, TaskResolver};
use loom_sync::chan::{Channel, Sender, Receiver};
```

## Navigation

[← Back to Libraries](../README.md)
