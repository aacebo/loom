# Loom Backlog

All planned refactoring phases have been completed.

## Completed Work

### Score -> Eval Rename

All `Score*` types have been renamed to `Eval*` throughout the codebase.

| Old Name | New Name |
|----------|----------|
| `ScoreLayer` | `EvalLayer` |
| `ScoreConfig` | `EvalConfig` |
| `ScoreCategoryConfig` | `CategoryConfig` |
| `ScoreLabelConfig` | `LabelConfig` |
| `ScoreModifierConfig` | `ModifierConfig` |
| `ScoreResult` | `EvalOutput` |
| `ScoreCategory` | `CategoryOutput` |
| `ScoreLabel` | `LabelOutput` |
| `ScoreLayerOutput` | DELETED |

### Phase 1: Cleanup (done)

Deleted old backlogs, roadmaps, and unused CLI commands (`validate`, `score`, `train`). Removed per-crate `BACKLOG.md` files and `docs/loom/roadmap/` directory.

### Phase 2: Simplify loom-pipe (done)

Refactored pipeline to use `LayerContext` trait with methods `input()`, `meta()`, `data_source()`, `emit()`. `Layer` trait now has `type Input: LayerContext` associated type. `Pipeline<C>` and `PipelineBuilder<C>` are generic over the context type.

### Phase 3: Create loom-eval Crate (done)

Extracted eval logic from `loom-runtime` into a new `loom-eval` crate. Applied Score -> Eval renames. `EvalLayer` implements `Layer<Input = RunContext>` and emits `eval.scored` signals.

### Phase 4: Runtime Integration (done)

Removed eval code from runtime. Runtime now holds `Pipeline<RunContext>` and exposes `execute(input) -> Result<Value>`. `RunContext` implements `LayerContext` with emitter and data source access.

### Phase 5: CLI + Umbrella Crate (done)

Simplified CLI to only have `run` command. Updated loom umbrella crate with `eval` feature.
