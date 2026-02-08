# loom-pipe

Pipeline and operator traits for the Loom ecosystem.

## Key Traits

### Operator

Transform a source into a new source:

```rust
pub trait Operator<Input> {
    type Output;
    fn apply(self, src: Source<Input>) -> Source<Self::Output>;
}
```

### Pipe

Chain operators together:

```rust
pub trait Pipe<Input> {
    fn pipe<Op: Operator<Input>>(self, op: Op) -> Source<Op::Output>;
}
```

### Build

Execute the pipeline and produce a result:

```rust
pub trait Build {
    type Output;
    fn build(self) -> Self::Output;
}
```

## Key Types

### Source

Wrapper around a lazy computation:

```rust
let source = Source::from(value);
let source = Source::new(|| compute_value());
```

### Transformer

Transform input to output:

```rust
let transformer = Transformer::new(source, |input| transform(input));
```

## Built-in Operators

### Map & Filter

```rust
// Transform values
let result = Source::from(1).map(|x| x * 2).build();

// Filter values
let result = Source::from(vec![1, 2, 3])
    .filter(|x| x > 1)
    .build();

// Fallible transformation
let result = Source::from("42")
    .try_map(|s| s.parse::<i32>())
    .build();
```

### Control Flow

- `Branch` - Conditional if-then-else execution
- `Router` - Route based on predicates to different handlers
- `FanOut` - Distribute to multiple operators in parallel

### Sequence Operators

- `Flatten` - Flatten `Vec<Vec<T>>` to `Vec<T>`
- `FlatMap` - Map and flatten in one step
- `Chunk` - Group items into fixed-size batches
- `Window` - Sliding window over sequences
- `Concat` - Merge multiple sequences

### Result/Option Operators

- `Unwrap` / `UnwrapOr` / `UnwrapOrElse` - Safe Result unwrapping
- `Expect` - Unwrap with error context
- `OptionOkOr` - Convert `Option` to `Result`

### Async Operators

- `Fork` - Execute work asynchronously, returns `Task` handle
- `Await` - Join/await on spawned tasks

### Time Operators

- `Timeout` - Fail if operation exceeds duration
- `Delay` - Add delay to execution

### Retry & Error Handling

- `Retry` - Automatic retry with configurable strategy
- `RetryBuilder` - Fluent retry configuration
- `Or` - Fallback values on error
- `OrElseMap` - Transform errors

### Parallel Operators

- `Parallel` - Concurrent operator execution
- `ParallelBuilder` - Fluent parallel composition

### Logical Operators

- `And` - Logical AND validation (both must pass)
- `Or` - Fallback to alternative on failure

## Usage

```toml
[dependencies]
loom-pipe = "0.0.1"
```

```rust
use loom_pipe::{Source, Build, Pipe};

let result = Source::from(42)
    .map(|x| x * 2)
    .build();

assert_eq!(result, 84);
```

## Navigation

[← Back to Libraries](../README.md)
