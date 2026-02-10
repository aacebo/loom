# Loom Development Workflow

## Lifecycle

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#6366f1', 'primaryTextColor': '#fff', 'primaryBorderColor': '#4f46e5', 'lineColor': '#94a3b8', 'secondaryColor': '#f472b6', 'tertiaryColor': '#34d399', 'background': '#0f172a', 'mainBkg': '#1e293b', 'nodeBorder': '#475569', 'clusterBkg': '#1e293b', 'clusterBorder': '#475569', 'titleColor': '#f8fafc', 'edgeLabelBackground': '#1e293b'}}}%%

flowchart TB
    subgraph STAGE["STAGE"]
        direction TB
        todo[libs/*/TODO.md]
    end

    plan{{"Plan Phases"}}

    subgraph BACKLOG["BACKLOG"]
        direction TB
        index[(backlog/README.md)]
        phases[backlog/NNN-phase-name.md]

        index --- phases
    end

    pop{{"Pop Next Phase"}}

    subgraph WORK["EXECUTE"]
        direction TB
        impl[Implement]
        test[Test]
        review[Review]

        impl --> test
        test --> review
    end

    complete{{"Mark Complete"}}

    subgraph DONE["ARCHIVE"]
        direction TB
        summary[Update backlog/README.md]
        clear[Clear TODO.md]
    end

    subgraph LOG["CHANGELOG"]
        direction TB
        cl1[libs/loom-*/CHANGELOG.md]
    end

    STAGE --> plan
    plan --> BACKLOG
    BACKLOG --> pop
    pop --> WORK
    WORK --> complete
    complete --> DONE
    DONE --> LOG

    %% Styling
    classDef stage fill:#a855f7,stroke:#9333ea,stroke-width:2px,color:#fff
    classDef index fill:#f472b6,stroke:#db2777,stroke-width:2px,color:#fff
    classDef action fill:#1e293b,stroke:#94a3b8,stroke-width:2px,color:#94a3b8
    classDef work fill:#22c55e,stroke:#16a34a,stroke-width:2px,color:#fff
    classDef done fill:#06b6d4,stroke:#0891b2,stroke-width:2px,color:#fff
    classDef changelog fill:#fbbf24,stroke:#d97706,stroke-width:2px,color:#000

    class todo stage
    class index,phases index
    class plan,pop,complete action
    class impl,test,review work
    class summary,clear done
    class cl1 changelog

    linkStyle default stroke:#94a3b8,stroke-width:2px
```

## Structure

```
libs/
├── loom-assert/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-cli/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-codec/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-config/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-core/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-cortex/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-error/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-eval/
│   ├── TODO.md
│   └── ...
├── loom-io/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-pipe/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-runtime/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-signal/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
├── loom-sync/
│   ├── TODO.md
│   ├── CHANGELOG.md
│   └── ...
└── loom/
    ├── TODO.md
    ├── CHANGELOG.md
    └── ...

backlog/
├── README.md                      ← Summary index of all phases
├── 001-phase-name.md              ← Phase 1
├── 002-phase-name.md              ← Phase 2
└── ...
```

## TODO.md Format

Each `libs/*/TODO.md` is a staging area for raw work items in that crate:

```markdown
# TODO

[Backlog](../../backlog/README.md)

- [ ] Description of work item
- [ ] Another work item
```

Once all items have been planned into phases and added to the backlog, empty the file:

```markdown
# TODO

[Backlog](../../backlog/README.md)
```

## Backlog Phase File Format

Each phase is a numbered file in `backlog/` (e.g. `backlog/001-simplify-pipeline.md`):

```markdown
# Phase NNN: Phase Name

| Attribute   | Value |
|-------------|-------|
| Difficulty  | S/M/L/XL |
| Scope       | S/M/L/XL |

## Summary

Brief description of what this phase accomplishes.

## Affected Crates

- `loom-xxx`
- `loom-yyy`

## Tasks

- [ ] Task 1
- [ ] Task 2
- [ ] Task 3

## Acceptance Criteria

- Criterion 1
- Criterion 2
```

## Workflow Rules

| Step | Description |
|------|-------------|
| **Stage** | Add raw work items to the relevant `libs/*/TODO.md` |
| **Plan** | Break TODO items into incremental phases, assign difficulty (S/M/L/XL) and scope (S/M/L/XL), create numbered phase files in `backlog/` |
| **Backlog** | Update `backlog/README.md` with a summary of all active phases |
| **Clear** | Once all items from a `TODO.md` are in the backlog, empty it back to the `# TODO` header |
| **Execute** | Pick next phase file, implement, test, review |
| **Complete** | Move completed phase to the Completed Work section in `backlog/README.md`, update changelogs |

## Crate Changelogs

Each crate maintains its own `CHANGELOG.md`:

| Crate | Recent Changes |
|-------|----------------|
| `loom-error` | Serde support for `Error` and `ErrorCode` |
| `loom-runtime` | Pipeline<RunContext> architecture, execute(), RunContext, removed eval module |
| `loom-pipe` | LayerContext trait, Layer trait with associated Input type, Pipeline<C>, PipelineBuilder<C> |
| `loom-eval` | New crate: EvalLayer, EvalConfig, EvalOutput, EvalResult, Score->Eval rename |
| `loom-config` | Multi-file config merge ($include), config integration, validation |
| `loom-cli` | Simplified to `run` command only |
| `loom-assert` | -- |
| `loom-codec` | -- |
| `loom-core` | -- |
| `loom-cortex` | -- |
| `loom-io` | -- |
| `loom-signal` | -- |
| `loom-sync` | -- |
| `loom` | -- |

## Completed Work

Phases removed from stack after completion (also recorded in crate changelogs):

- **Context Refactor** - Context as active runtime client, BatchContext for batch processing
- **Multi-File Config Merge** - $include directive for config composition
- **Time Operators** - Timeout, delay
- **Sequence Operators** - Flatten, flat_map, chunk, window, concat
- **Control Flow & Result Ops** - Branch, and/or, retry, unwrap/expect operators
- **Error Aggregation** - `loom_error::Result<Value>` in `LayerResult`
- **Config Integration** - `loom-config` crate with env var support
- **Pipeline Rewrite** - Layer trait infrastructure
- **Fork/Join** - Renamed spawn->fork, added `.join()`
- **Result Metadata** - Timing metrics (`elapsed_ms`, `throughput`)
- **Eval Extraction** - Created `loom-eval` crate, Score->Eval rename
- **Runtime Simplification** - Pipeline<RunContext>, execute(), removed eval module
- **CLI Cleanup** - Removed validate, score, train commands
