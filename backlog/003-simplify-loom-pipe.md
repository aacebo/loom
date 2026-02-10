# Phase 003: Simplify loom-pipe

| Attribute   | Value |
|-------------|-------|
| Difficulty  | L |
| Scope       | L |

## Summary

Refactor pipeline infrastructure to use a `LayerContext` trait and make `Pipeline` generic over the context type.

## Affected Crates

- `loom-pipe`
- `loom-runtime`
- `loom-eval`

## Tasks

- [x] Create `LayerContext` trait with `input()`, `meta()`, `data_source()`, `emit()` methods
- [x] Add `type Input: LayerContext` associated type to `Layer` trait
- [x] Make `Pipeline<C>` generic over context type
- [x] Make `PipelineBuilder<C>` generic over context type

## Acceptance Criteria

- `LayerContext` is a trait, not a concrete type
- `Pipeline` and `PipelineBuilder` are generic over any `LayerContext` implementor
- Existing layers compile against the new trait
