# Plan: Enrich hybrid-algorithm.md with Research Metrics

## Context

The deep research report contains extensive concrete metrics, model benchmarks, failure mode analysis, and decision criteria that the current `hybrid-algorithm.md` lacks. The goal is to incorporate all key data from the research into the doc while keeping it scannable and well-structured.

## Gap Analysis: What the Research Has That the Doc Lacks

### 1. Hard metrics in "At a Glance" table (currently soft/vague)
The current table says "~50–200 ms", "$$$", "Thousands/sec". The research provides:
- Local bi-encoder: **~18,000 queries/s** on V100, **~750 q/s** on CPU
- Local cross-encoder: **~1,800 docs/s**
- LLM judge: **~0.34 eval/s** (~2.92s per eval)
- Cost per eval: LLM **~$0.0032** vs local **~$0.0000005** (~6,776× difference)
- Cost per 10k evals: LLM **~$32** vs local **~$0.005**
- GPT-4.1 pricing: **$2/1M input, $8/1M output**
- Claude Opus pricing: **$5/1M input, $25/1M output**

### 2. Accuracy vs humans metrics (entirely missing)
- G-EVAL (GPT-4) on SummEval: avg Spearman **ρ ≈ 0.514**, Kendall **τ ≈ 0.418**
- BERTScore baseline: avg Spearman **ρ ≈ 0.225**, Kendall **τ ≈ 0.175**
- LLM judges: **>80%** human agreement on pairwise preference
- SimCSE: avg Spearman **~81.6%** on STS tasks
- Length-controlled AlpacaEval: Spearman **0.94 → 0.98**

### 3. Model landscape table (entirely missing)
Research has a detailed table of representative models: GPT-4, Claude Opus, Llama 2/3/3.1, DeBERTa, RoBERTa, SBERT, SimCSE, MiniLM — with params, throughput, and strengths/weaknesses.

### 4. Bias and failure modes (entirely missing)
- **LLM biases:** position bias, length/verbosity bias, nondeterminism even at T=0
- **Local failure modes:** semantic overlap ≠ correctness, granularity mismatch (sentence-level NLI vs doc-level), adversarial brittleness (ANLI)

### 5. Decision criteria by use case (entirely missing)
- High-stakes (safety/compliance): local + human escalation
- Low-latency CI/nightly regressions: local dominant
- Budget-constrained: local wins on unit cost
- Reference-free (helpfulness/style): LLM judges most direct

### 6. Engineering considerations (entirely missing)
- Versioning, reproducibility, model drift
- Prompt engineering and judge reliability
- Privacy, data residency, offline capability

### 7. Benchmarks for meta-evaluation (entirely missing)
SummEval, MT-Bench/Chatbot Arena, WMT Metrics, HELM, MNLI, ANLI, STS-B

### 8. Hardware/optimization details (partially missing)
- MiniLM: **22.7M params** vs Llama 3.1 70B: **70B params** (3,084×)
- ONNX-O4 speedup: **~1.83×** GPU, **~3.08×** CPU
- GPT-4.1 TTFT: **~15s** at 128k context, **~1min** at 1M context
- vLLM/PagedAttention: **2–4×** throughput improvement

## File to Modify

`docs/loom/hybrid-algorithm.md`

## Plan: New Section Structure

Keep the existing flow but enrich it and add new sections. Final structure:

```
# 6.1.2 Hybrid Algorithm
[nav breadcrumb - keep as is]
[intro paragraph - keep as is]

## At a Glance                          ← ENRICH with hard metrics
## Research Basis                       ← NEW: benchmarks, models, accuracy data
## What "Hybrid" Means                  ← keep as is
## End-to-End Flow                      ← keep as is
## The Gating Logic                     ← keep as is
## Scoring + Thresholds Pattern         ← keep as is
## Reconciling Local vs LLM Outputs     ← keep as is
## Backpressure / Cost Control Loop     ← keep as is
## Known Biases and Failure Modes       ← NEW
## Decision Criteria by Use Case        ← NEW
## Engineering Considerations           ← NEW
## Performance Benchmarks               ← NEW: the real metrics table
## Why Hybrid Is Usually the Best Default ← keep as is
## Implementation Path                  ← keep as is
```

## Detailed Changes

### 1. Enrich "At a Glance" with hard numbers
Replace the soft values with concrete metrics from the research. Add rows for accuracy vs humans, cost/eval, and model size.

### 2. Add "Research Basis" section after At a Glance
- Brief paragraph on the two evaluation families (LLM judges vs local NLI/STS)
- Sub-table of representative models with params, throughput, strengths/weaknesses
- Sub-table of meta-evaluation benchmarks (SummEval, MT-Bench, WMT, etc.)
- Key accuracy findings: G-EVAL Spearman/Kendall vs BERTScore on SummEval

### 3. Add "Known Biases and Failure Modes" section after Backpressure
Two sub-sections:
- **LLM Judge biases:** position bias, length/verbosity bias, nondeterminism at T=0, prompt sensitivity, model drift
- **Local model failure modes:** semantic overlap ≠ correctness, granularity mismatch, adversarial brittleness, domain shift

### 4. Add "Decision Criteria by Use Case" section
Table with 4 use cases: high-stakes, low-latency CI, budget-constrained, reference-free — mapping each to recommended approach.

### 5. Add "Engineering Considerations" section
Three sub-sections:
- **Versioning & drift:** store prompt template, model version, decoding params, raw response; run canary suites
- **Prompt engineering:** structured form-filling, order randomization, multi-run stability
- **Privacy & offline:** local models for air-gapped/regulated; hosted open LLMs as middle ground

### 6. Add "Performance Benchmarks" section with charts AND table
Use **mermaid charts** to visualize key comparisons wherever possible:

- **Accuracy chart** (xychart-beta bar): G-EVAL vs BERTScore on SummEval — Spearman ρ and Kendall τ side by side
- **Throughput chart** (xychart-beta bar): bi-encoder (18,000 q/s) vs cross-encoder (1,800 d/s) vs LLM judge (0.34 e/s) — log-scale visual
- **Cost per 10k evals chart** (xychart-beta bar): LLM ~$32 vs local ~$0.005
- **Model size chart** (xychart-beta bar): MiniLM 22.7M vs Llama 3.1 70B params

Follow each chart with the full metrics comparison table from the research appendix (absolute values + percentage differences) for readers who want exact numbers.

Include the reference workload assumptions as a blockquote before the table.

## Verification

- All 5 original mermaid diagrams remain intact and render correctly
- All metrics from the research report are present in the doc
- New sections use tables consistently with existing style
- Doc remains scannable — no walls of prose
