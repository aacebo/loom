# Merc Scoring Improvements

Roadmap for improving Merc's write-time scoring accuracy while maintaining <200ms latency.

<pre>
├── <a href="./README.md"><b>&lt;&lt;merc&gt;&gt;</b></a> 👈
├── <a href="./scoring-algorithm.md">Scoring Algorithm</a>
├── <a href="./1.foundation.md">1. Foundation</a>
├── <a href="./2.labels.md">2. Label Expansion</a>
├── <a href="./3.context.md">3. Context & Ensemble</a>
├── <a href="./4.learning.md">4. Learning Infrastructure</a>
└── <a href="./5.output.md">5. Output Enrichment</a>
</pre>

---

## What Merc Is

Merc is a **fast write-time gating filter** for AI memory systems:

- **Accept/reject in <200ms** — Local inference, no LLM API calls
- **Interpretable scores** — Clear 43-label breakdown across 6 categories
- **Zero LLM cost** — Uses zero-shot classification (rust_bert)
- **Pluggable** — Output feeds downstream systems (Zep, Hindsight, etc.)

## What Merc Is NOT

Merc focuses on one thing well—scoring. Other concerns are handled downstream:

| Concern | Where It's Handled |
|---------|-------------------|
| Retrieval | Zep, Hindsight |
| Entity extraction | Downstream LLM |
| Knowledge graph | Downstream storage |
| Contradiction resolution | Downstream temporal systems |
| PII masking | Downstream compliance layer |

---

## Research Context

These improvements are informed by analysis of production memory systems:

| System | Key Insight | How Merc Applies |
|--------|-------------|------------------|
| **Zep** | Bi-temporal timestamps (event time + ingestion time) | Temporal labels flag time-sensitive content |
| **Zep** | Automatic contradiction detection | `Temporal_Update` label flags changes |
| **Hindsight** | Epistemic networks (World/Experience/Opinion/Observation) | Similar to Context/Emotion/Outcome/Sentiment categories |
| **Hindsight** | Opinion confidence tracking | Platt calibration achieves similar goal |
| **Enterprise Model** | BERT local inference | Already using local zero-shot classification |
| **Enterprise Model** | Sensitivity tagging (0-3 scale) | Sensitivity labels (Sensitive, Confidential) |
| **All** | Redundancy for reliability | Lightweight 2-model ensemble |

See [research docs](../research/) and [analysis docs](../analysis/) for detailed comparisons.

---

## Phase Overview

| Phase | File | Latency Impact | Expected Gain |
|-------|------|----------------|---------------|
| 1. Foundation | [1.foundation.md](./1.foundation.md) | 0% | 20-35% |
| 2. Label Expansion | [2.labels.md](./2.labels.md) | +10-15% | 30-45% |
| 3. Context & Ensemble | [3.context.md](./3.context.md) | +50-100% | 25-45% |
| 4. Learning | [4.learning.md](./4.learning.md) | 0% runtime | Continuous |
| 5. Output | [5.output.md](./5.output.md) | 0% | Downstream compat |

---

## Label Summary

| Category | Current | Proposed | Change |
|----------|---------|----------|--------|
| Sentiment | 3 | 3 | — |
| Emotion | 7 | 8 | +1 (Concern) |
| Outcome | 7 | 8 | +1 (Commitment) |
| Context | 9 | 17 | +8 |
| Negative | 1 | 4 | +3 |
| Temporal | 0 | 3 | +3 (new category) |
| **Total** | **27** | **43** | **+16** |

---

## Success Metrics

| Metric | Current | Phase 1 | Phase 2 | Phase 3 |
|--------|---------|---------|---------|---------|
| Accuracy | Baseline | +20% | +35% | +50% |
| False Positives | Baseline | -15% | -30% | -40% |
| Noise Stored | Baseline | — | -25% | -30% |
| Latency | ~50ms | ~50ms | ~60ms | ~100-150ms |

---

## Quick Links

- [Current Scoring Algorithm](./scoring-algorithm.md) — How scoring works today
- [Phase 1: Foundation](./1.foundation.md) — Quick wins with zero latency cost
- [Phase 2: Labels](./2.labels.md) — Expanded label taxonomy
- [Phase 3: Context](./3.context.md) — Context window and ensemble
- [Phase 4: Learning](./4.learning.md) — Feedback and tuning infrastructure
- [Phase 5: Output](./5.output.md) — Structured output for downstream systems
