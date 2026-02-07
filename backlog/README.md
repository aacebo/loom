# Loom Backlog

## Status Overview

No pending phases. All planned work has been completed.

## Completed Work Summary

The following phases have been completed and their documentation archived:

- **CLI Command Structs** - Refactored CLI commands to dedicated structs with clap validation
- **Multi-File Config Merge** - $include directive for config composition
- **Time Operators** - timeout, delay
- **Sequence Operators** - flatten, flat_map, chunk, window, concat
- **Control Flow & Result Ops** - branch, and/or, retry, unwrap/expect operators
- **Config Integration** - loom-config crate integrated with env var support
- **Validation** - Config validation with garde derive macros
- **Pipeline Rewrite** - Pipeline infrastructure with Layer trait
- **Dynamic Layers** - Runner removal, config simplification
- **Output Behavior** - CLI output path handling (auto-naming)
- **Fork/Join** - Renamed spawn→fork, added .join()
- **Simplify Structure** - Merged modules, flattened CLI
- **Result Metadata** - Timing and resource metrics (elapsed_ms, throughput)
- **Error Aggregation** - Hierarchical layer errors with loom_error::Result support

## Environment Variable Support

Override config via environment variables:

```bash
LOOM_CONCURRENCY=16 loom run -c config.toml ...
LOOM_BATCH__SIZE=32 loom run -c config.toml ...
LOOM_LAYERS_SCORE_THRESHOLD=0.8 loom run -c config.toml ...
```
