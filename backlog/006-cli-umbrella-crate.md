# Phase 006: CLI + Umbrella Crate

| Attribute   | Value |
|-------------|-------|
| Difficulty  | S |
| Scope       | S |

## Summary

Simplify CLI to only the `run` command and update the `loom` umbrella crate with an `eval` feature flag.

## Affected Crates

- `loom-cli`
- `loom`

## Tasks

- [x] Remove `validate` command from CLI
- [x] Remove `score` command from CLI
- [x] Remove `train` command from CLI
- [x] Add `eval` feature flag to umbrella crate

## Acceptance Criteria

- CLI only exposes the `run` command
- `loom` umbrella crate re-exports `loom-eval` behind the `eval` feature
