# Pipeline System for loom-pipe

**Status: IMPLEMENTED**

## Overview

Expand `loom-pipe` with a Layer-based pipeline system that pipes layer outputs as inputs to subsequent layers, supports branching/DAG structures via Operators.

## Implemented Features

### LayerContext Trait
```rust
pub trait LayerContext: Send + 'static {
    fn text(&self) -> &str;
    fn step(&self) -> usize;
    fn meta(&self) -> &Map;
    fn meta_mut(&mut self) -> &mut Map;
}
```

### LayerResult Struct
```rust
pub struct LayerResult<T> {
    pub meta: Map,
    pub output: T,
}
```

### Layer Trait
```rust
pub trait Layer: Send {
    type Input: LayerContext;
    type Output: Send + 'static;
    fn process(&self, input: Self::Input) -> Result<LayerResult<Self::Output>>;
    fn name(&self) -> &'static str;
}
```

### Pipeline Builder (type-safe chaining)
```rust
let pipeline = PipelineBuilder::new()
    .then(score_layer)
    .build();

let result = pipeline.execute(context)?;
```

### Operators

All operators are in `loom-pipe/src/operators/`:

| Operator | Input | Output | Description |
|----------|-------|--------|-------------|
| **MapOperator** | `T` | `U` | Transform input with closure |
| **FanOut** | `T` (Clone) | `Vec<U>` | Clone to multiple ops, collect sequentially |
| **Router** | `T` | `Option<U>` | Route to first matching predicate |
| **Guard** | `T` | `Option<T>` | Allow/block based on predicate |
| **Filter** | `Vec<T>` | `Vec<T>` | Filter items in collection |
| **TryMap** | `T` | `Result<U>` | Fallible transformation |
| **Spawn** | `T` | `Task<U>` | Spawn async work via `loom-sync` |
| **Await** | `Task<T>` | `TaskResult<T>` | Wait for task completion |
| **Parallel** | `T` (Clone) | `Vec<TaskResult<U>>` | Execute branches concurrently |

## File Structure

```
libs/loom-pipe/src/
├── lib.rs
├── pipeline/
│   ├── mod.rs
│   ├── context.rs     # LayerContext trait, LayerResult struct
│   ├── layer.rs       # Layer trait
│   ├── node.rs        # AnyLayer (type-erased), LayerNode wrapper
│   ├── builder.rs     # PipelineBuilder
│   └── pipeline.rs    # Pipeline execution
├── operators/
│   ├── mod.rs
│   ├── map.rs         # MapOperator
│   ├── fan_out.rs     # FanOut operator
│   ├── router.rs      # Router operator
│   ├── guard.rs       # Guard operator
│   ├── filter.rs      # Filter operator
│   ├── try_map.rs     # TryMap operator
│   ├── spawn.rs       # Spawn async operator
│   ├── wait.rs        # Await operator
│   └── parallel.rs    # Parallel operator
├── source.rs
└── transformer.rs
```

## Runtime Integration

**loom-runtime** now:
- Re-exports pipeline types (`Layer`, `LayerContext`, `LayerResult`, `Pipeline`, `PipelineBuilder`)
- Re-exports operators (`FanOut`, `Router`, `Guard`, `Filter`, `TryMap`, `Spawn`, `Await`, `Parallel`)
- Provides `Runtime::pipeline()` method
- `Context<Input>` implements `LayerContext` trait
- `ScoreLayer` implements `Layer` trait

### Example Usage
```rust
use loom_runtime::{Runtime, Context, PipelineBuilder, Layer, LayerResult};
use loom_runtime::score::ScoreConfig;

let runtime = Runtime::new().build();
let score_layer = ScoreConfig::default().build()?;

// Direct layer usage
let context = Context::new("Hello!", ());
let result: LayerResult<ScoreResult> = score_layer.process(context)?;

// Or with pipeline builder
let pipeline = runtime.pipeline()
    .then(score_layer)
    .build();

let result = pipeline.execute(context)?;
```

### Using Operators
```rust
use loom_pipe::{Source, Build, Pipe};
use loom_pipe::operators::{Guard, FanOut, Parallel, MapOperator};

// Guard: conditionally allow/block
let result = Source::from(42)
    .pipe(Guard::allow(|x| *x > 0))
    .build(); // Some(42)

// FanOut: sequential fan-out
let results = Source::from("hello")
    .pipe(FanOut::new()
        .add(MapOperator::new(|s: &str| s.len()))
        .add(MapOperator::new(|s: &str| s.to_uppercase())))
    .build(); // [5, "HELLO"]

// Parallel: concurrent execution with tasks
let results = Source::from(10)
    .pipe(Parallel::new()
        .add(|x| x * 2)
        .add(|x| x + 5))
    .build(); // [TaskResult::Ok(20), TaskResult::Ok(15)]
```

## Future Work

- [x] Parallel branch with `loom-sync` for concurrent execution
- [ ] StreamingPipeline with channel-based inter-stage communication
- [ ] Async pipeline execution (`process_async`)
