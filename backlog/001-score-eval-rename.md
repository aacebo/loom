# Phase 001: Score -> Eval Rename

| Attribute   | Value |
|-------------|-------|
| Difficulty  | M |
| Scope       | L |

## Summary

Rename all `Score*` types to `Eval*` throughout the codebase for consistent terminology.

## Affected Crates

- `loom-eval`
- `loom-runtime`
- `loom-pipe`

## Tasks

- [x] Rename `ScoreLayer` to `EvalLayer`
- [x] Rename `ScoreConfig` to `EvalConfig`
- [x] Rename `ScoreCategoryConfig` to `CategoryConfig`
- [x] Rename `ScoreLabelConfig` to `LabelConfig`
- [x] Rename `ScoreModifierConfig` to `ModifierConfig`
- [x] Rename `ScoreResult` to `EvalOutput`
- [x] Rename `ScoreCategory` to `CategoryOutput`
- [x] Rename `ScoreLabel` to `LabelOutput`
- [x] Delete `ScoreLayerOutput`

## Acceptance Criteria

- No remaining `Score*` types in the codebase
- All references updated to use `Eval*` naming
