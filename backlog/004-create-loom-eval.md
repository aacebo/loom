# Phase 004: Create loom-eval Crate

| Attribute   | Value |
|-------------|-------|
| Difficulty  | L |
| Scope       | M |

## Summary

Extract eval logic from `loom-runtime` into a standalone `loom-eval` crate with the new `Eval*` naming.

## Affected Crates

- `loom-eval` (new)
- `loom-runtime`

## Tasks

- [x] Create `loom-eval` crate
- [x] Move eval logic out of `loom-runtime`
- [x] Implement `EvalLayer` with `Layer<Input = RunContext>`
- [x] Implement `EvalConfig`, `EvalOutput`, `EvalResult`
- [x] Emit `eval.scored` signals from `EvalLayer`

## Acceptance Criteria

- `loom-eval` is a standalone crate with no circular dependencies
- `loom-runtime` has no eval-specific code
- `EvalLayer` emits signals via the `LayerContext` interface
