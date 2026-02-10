# Phase 002: Cleanup

| Attribute   | Value |
|-------------|-------|
| Difficulty  | S |
| Scope       | M |

## Summary

Delete old backlogs, roadmaps, and unused CLI commands to reduce project clutter.

## Affected Crates

- `loom-cli`

## Tasks

- [x] Delete old backlog files
- [x] Delete roadmap documents
- [x] Remove `validate` CLI command
- [x] Remove `score` CLI command
- [x] Remove `train` CLI command
- [x] Remove per-crate `BACKLOG.md` files
- [x] Remove `docs/loom/roadmap/` directory

## Acceptance Criteria

- No stale backlog or roadmap files remain
- CLI only exposes active commands
