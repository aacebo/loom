# Phase 005: Runtime Integration

| Attribute   | Value |
|-------------|-------|
| Difficulty  | M |
| Scope       | M |

## Summary

Simplify runtime to hold `Pipeline<RunContext>` and expose a clean `execute()` API. Layers are injected externally.

## Affected Crates

- `loom-runtime`

## Tasks

- [x] Refactor `Runtime` to hold `Pipeline<RunContext>`
- [x] Implement `execute(input) -> Result<Value>`
- [x] Implement `RunContext` with `LayerContext` trait (emitter + data source access)
- [x] Remove eval module from runtime
- [x] Support external layer injection via `Runtime::new().layer()`

## Acceptance Criteria

- Runtime has no knowledge of specific layer types
- `execute()` creates `RunContext` and iterates layers
- Layers are injected externally, not constructed by runtime
