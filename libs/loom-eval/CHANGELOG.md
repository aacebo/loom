# Changelog

All notable changes to `loom-eval` will be documented in this file.

## [Unreleased]

_No changes yet._

## Completed

- **Crate Creation** - Extracted eval logic from `loom-runtime` into standalone `loom-eval` crate
- **Score -> Eval Rename** - Renamed all `Score*` types to `Eval*` (`EvalLayer`, `EvalConfig`, `EvalOutput`, `EvalResult`)
- **Signal Emission** - `EvalLayer` emits `eval.scored` signals via `LayerContext`
