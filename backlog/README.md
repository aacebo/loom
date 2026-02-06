# Loom Backlog

## Status Overview

| Phase | Description | Crate | Status |
|-------|-------------|-------|--------|
| [01-result-metadata](01-result-metadata.md) | Add timing & resource metrics | runtime | PENDING |
| [02-error-aggregation](02-error-aggregation.md) | Hierarchical layer errors | runtime | PENDING |
| [03-control-flow-ops](03-control-flow-ops.md) | if/then/else, and/or | pipe | PENDING |
| [04-result-operators](04-result-operators.md) | retry, expect/unwrap | pipe | PENDING |
| [05-collection-ops](05-collection-ops.md) | flatten, chunk, window | pipe | PENDING |
| [06-time-operators](06-time-operators.md) | timeout, debounce | pipe | PENDING |
| [07-multi-file-merge](07-multi-file-merge.md) | Config file includes/refs | config | PENDING |

## Priority Tiers

### Tier 1: Runtime Infrastructure
- **Phase 01**: Result metadata - Timing and resource metrics
- **Phase 02**: Error aggregation - Hierarchical error support

### Tier 2: Pipe Operators - Foundation
- **Phase 03**: Control flow - if/then/else, and/or
- **Phase 04**: Result operators - retry, expect/unwrap

### Tier 3: Pipe Operators - Advanced
- **Phase 05**: Collection operators - flatten, chunk, window
- **Phase 06**: Time operators - timeout, debounce

### Tier 4: Config Enhancement
- **Phase 07**: Multi-file config merge

## Dependencies

```
Phase 01 (Metadata) ──► Phase 02 (Errors)


Phase 03 (Control) ──┬─► Phase 04 (Result)
                     │
                     └─► Phase 05 (Collection)
                              │
                              └──► Phase 06 (Time)


Phase 07 (Config) - Independent
```

## Completed Work Summary

The following phases have been completed and their documentation archived:

- **Config Integration** - loom-config crate integrated with env var support
- **Validation** - Config validation with garde derive macros
- **Pipeline Rewrite** - Pipeline infrastructure with Layer trait
- **Dynamic Layers** - Runner removal, config simplification
- **Output Behavior** - CLI output path handling (auto-naming)
- **Fork/Join** - Renamed spawn→fork, added .join()
- **Simplify Structure** - Merged modules, flattened CLI

## Environment Variable Support

Override config via environment variables:

```bash
LOOM_CONCURRENCY=16 loom run -c config.toml ...
LOOM_BATCH__SIZE=32 loom run -c config.toml ...
LOOM_LAYERS_SCORE_THRESHOLD=0.8 loom run -c config.toml ...
```
