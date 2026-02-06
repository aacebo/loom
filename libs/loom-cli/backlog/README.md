# Loom Architecture & Scaling Backlog

## Pipeline Architecture Overview

```
┌─────────────────────────────────────┐
│   HTTP API Layer (Actix-web)        │ bins/api
├─────────────────────────────────────┤
│   Event/Message Queue (RabbitMQ)    │ crates/events
├─────────────────────────────────────┤
│   Worker/Job Processing             │ bins/worker
├─────────────────────────────────────┤
│   Runtime Orchestration             │ loom-runtime
├─────────────────────────────────────┤
│   Pipeline (Lazy, Type-Erased)      │ loom-pipe
├─────────────────────────────────────┤
│   ML Scoring & Traits               │ loom-cortex
├─────────────────────────────────────┤
│   Task Execution Abstraction        │ loom-sync
├─────────────────────────────────────┤
│   Storage (PostgreSQL + sqlx)       │ crates/storage
├─────────────────────────────────────┤
│   Codecs & I/O Abstraction          │ loom-codec, loom-io
└─────────────────────────────────────┘
```

## Layer Descriptions

### 1. Runtime Layer (`loom-runtime`)
- Central orchestration point
- Manages data sources (FileSystem, etc.)
- Handles codecs (JSON, YAML, TOML)
- Provides pipeline builder factory

### 2. Pipeline/Processing Layer (`loom-pipe`)
- Lazy/pull-based pipeline system
- Type-erased stage execution
- Operators: Spawn, Parallel, FanOut, Router

### 3. ML/Scoring Layer (`loom-cortex`)
- Traits: `Scorer` (sync), `AsyncScorer` (async), and `ScorerOutput`
- Async runners with `spawn_blocking` for CPU-bound inference
- Platt calibration for probability calibration
- Machine learning model implementations

### 4. Event/Message Layer (`crates/events`)
- RabbitMQ-based async event queue
- Producer/Consumer pattern
- Topic-based routing via exchange bindings

### 5. Worker Layer (`bins/worker`)
- Consumes events from RabbitMQ
- Processes long-running work asynchronously
- Configurable via environment variables

### 6. Storage Layer (`crates/storage`)
- PostgreSQL with sqlx
- Migrations-based schema management

---

## Horizontal Scaling Assessment

### Current Readiness

| Component | Status | Notes |
|-----------|--------|-------|
| API Server | Ready | Actix multi-worker, stateless |
| Message Queue | Ready | RabbitMQ topic routing supports N workers |
| Worker Process | Partial | Stub implementation, doesn't process events yet |
| ML Inference | Improved | Async runners added, serialized via Mutex (single model) |
| Database | Tight | Pool max=5, needs tuning for N workers |

### Target Topology

```
Load Balancer
    ↓
[API Server 1] ────┐
[API Server 2] ──→ RabbitMQ ← [Worker 1]
[API Server 3] ────┤  (amqp)    [Worker 2]
                   ↓            [Worker N]
                PostgreSQL
```

---

## Critical Bottlenecks

### 1. ~~Synchronous Scoring Pipeline~~ RESOLVED
**Status: FIXED**
- ✅ Async runner functions added (`run_async_with_config`, `export_async_with_config`)
- ✅ Uses `tokio::task::spawn_blocking` for CPU-bound inference
- ✅ Progress callbacks fire in real-time as each sample completes
- ⚠️ Serialized via `Arc<Mutex<S>>` - single model instance limitation

**Location:** `libs/loom-runtime/src/bench/runner.rs`

### 2. ~~Scorer Trait is Synchronous~~ RESOLVED
**Status: FIXED**
```rust
#[async_trait]
pub trait AsyncScorer: Send + Sync {
    type Output: ScorerOutput + Send;
    type Error: Send;
    async fn score_async(&self, text: &str) -> Result<Self::Output, Self::Error>;
}
```
- ✅ `AsyncScorer` trait added alongside sync `Scorer`
- ✅ CLI updated with `--concurrency` flag

**Note:** True parallelism requires multiple model instances because rust-bert models contain `tch::Tensor` with raw pointers that aren't `Sync`.

### 3. RabbitMQ Worker is Incomplete
- Consumer dequeue loop exists but discards events
- No actual event processing implemented
- One event processed at a time

**Location:** `bins/worker/src/main.rs`

### 4. No Distributed State Management
- No distributed caching for model results
- No request deduplication
- No batch size optimization for ML inference

### 5. Database Pool is Tight
- Only 5 connections max
- Would need tuning for N workers

---

## Backlog Items

### High Priority

- [x] **Make Scorer trait async** - `AsyncScorer` trait added to `loom-cortex/src/bench/scorer.rs`
- [x] **Add async runner functions** - `run_async_with_config`, `export_async_with_config` in `loom-runtime/src/bench/runner.rs`
- [x] **Add CLI concurrency flag** - `--concurrency` flag for `bench run` and `bench score`
- [ ] **Complete worker implementation** - Currently dequeues and discards messages
- [ ] **Complete API ingest route** - Currently returns 200 without processing
- [ ] **Add batch inference support** - Accumulate requests, process in parallel
- [ ] **Increase database connection pool** - Dynamic based on worker count

### Medium Priority

- [ ] **Add multiple model instances** - Enable true parallelism (current Mutex serializes)
- [ ] **Add AMQP acknowledgment logic** - Worker should ack/nack based on processing success
- [ ] **Add model caching/sharing** - Redis or shared model server across workers
- [ ] **Implement request deduplication** - In API layer
- [ ] **Add distributed tracing** - OpenTelemetry for observability
- [ ] **Add metrics collection** - API/Worker/Queue depth monitoring

### Low Priority

- [ ] **Pipeline DAG execution** - Type-erased system could support parallel stage execution
- [ ] **Model loading optimization** - Shared model server to avoid per-worker loading
- [ ] **Rate limiting / backpressure** - Prevent worker overload

---

## Key Insights

1. **Scoring bottleneck addressed** - Async runners added with `spawn_blocking`. Still serialized via Mutex due to rust-bert thread-safety.

2. **Architecture is ready for horizontal scaling** - But API/Worker implementation is incomplete (stubs only).

3. **Pipeline is clever** - Type-erased, lazy evaluation, composable operators.

4. **Event-driven foundation is solid** - RabbitMQ integration correct for multi-worker pattern.

5. **Thread-safety constraint** - rust-bert models contain `tch::Tensor` with raw pointers (`*mut C_tensor`) that aren't `Sync`. True parallelism requires N model instances.

6. **Storage layer is complete** - All DAOs implemented but not yet wired to API/Worker.

---

## File References

| Layer | Location |
|-------|----------|
| Pipeline | `libs/loom-pipe/` |
| Runtime | `libs/loom-runtime/` |
| Events | `crates/events/` |
| API | `bins/api/` |
| Worker | `bins/worker/` |
| Task Sync | `libs/loom-sync/` |
| ML Scoring | `libs/loom-cortex/` |

---

## Recent Changes

### 2026-02-06: Async Scoring Infrastructure

**Added:**
- `AsyncScorer` trait in `libs/loom-cortex/src/bench/scorer.rs`
- Async runner functions in `libs/loom-runtime/src/bench/runner.rs`:
  - `run_async()`, `run_async_with_config()`
  - `export_async()`, `export_async_with_config()`
- `AsyncRunConfig` struct with configurable concurrency
- `--concurrency` CLI flag for `bench run` and `bench score`

**Technical Approach:**
- Uses `Arc<Mutex<S>>` to wrap scorer (rust-bert not thread-safe)
- `tokio::task::spawn_blocking` for CPU-bound inference
- Progress callbacks fire in real-time via stream processing
- Foundation for future multi-model parallelism
