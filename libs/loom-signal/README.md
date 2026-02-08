# loom-signal

Observability abstractions for the Loom ecosystem.

## Overview

This crate provides traits and types for instrumenting Loom applications with telemetry, metrics, tracing, and logging.

## Key Types

### Signal

A telemetry event with structured data:

```rust
let signal = Signal::new()
    .name("request.completed")
    .level(Level::Info)
    .otype(Type::Event)
    .attr("duration_ms", 42)
    .attr("status", "success")
    .build();
```

Fields:
- `otype` - Signal type (Event, Span, Metric, Log)
- `level` - Log level (Trace, Debug, Info, Warn, Error)
- `name` - Human-readable identifier
- `attributes` - Key-value context data
- `created_at` - Timestamp

### Level

Log levels for signal severity:

```rust
pub enum Level {
    Trace,  // Fine-grained debugging
    Debug,  // Debugging information
    Info,   // General information
    Warn,   // Warning conditions
    Error,  // Error conditions
}
```

### Type

Signal type classification:

```rust
pub enum Type {
    Event,   // Generic events
    Span,    // Timed operations (has duration)
    Metric,  // Numeric measurements
    Log,     // Log-style messages
}
```

### Emitter

Trait for emitting signals:

```rust
pub trait Emitter {
    fn emit(&self, signal: Signal);
}
```

### SignalBroadcaster

Broadcasts signals to multiple emitters:

```rust
let broadcaster = SignalBroadcaster::new()
    .add(StdoutEmitter::new())
    .add(FileEmitter::new("signals.jsonl")?);

broadcaster.emit(signal); // Sends to both
```

### NoopEmitter

Discards all signals (useful for testing or when signals are disabled):

```rust
let emitter = NoopEmitter::default();
```

## Built-in Consumers

### StdoutEmitter

Prints signals to stdout:

```rust
use loom_signal::consumers::StdoutEmitter;

let emitter = StdoutEmitter::new();
```

### FileEmitter

Writes signals to a file (JSONL format):

```rust
use loom_signal::consumers::FileEmitter;

let emitter = FileEmitter::new("signals.jsonl")?;
```

### MemoryEmitter

Stores signals in memory:

```rust
use loom_signal::consumers::MemoryEmitter;

let emitter = MemoryEmitter::new();
// Later: emitter.signals() to retrieve
```

## Usage

```toml
[dependencies]
loom-signal = "0.0.1"
```

```rust
use loom_signal::{Signal, Emitter, Level, Type, SignalBroadcaster};
use loom_signal::consumers::StdoutEmitter;

let emitter = StdoutEmitter::new();
let signal = Signal::new()
    .name("app.started")
    .level(Level::Info)
    .build();

emitter.emit(signal);
```

## Navigation

[← Back to Libraries](../README.md)
