# 6.3 Analysis

Comparative analysis of Loom against other memory systems.

<pre>
├── <a href="../README.md">..</a>
├── <a href="../1.memory.md">▸ 1. Memory</a>
├── <a href="../2.ingestion.md">▸ 2. Ingestion</a>
├── <a href="../3.guards.md">▸ 3. Guards</a>
├── <a href="../4.recall.md">▸ 4. Recall</a>
├── <a href="../5.classification.md">▸ 5. Classification</a>
└── <a href="../README.md">▾ 6. Research/</a>
    ├── <a href="../loom/README.md">▸ 6.1 Loom/</a>
    ├── <a href="../reference/README.md">▸ 6.2 Reference/</a>
    └── <span><a href="./README.md"><b>▾ 6.3 Analysis/</b></a> 👈</span>
        ├── <a href="./1.loom-vs-hindsight.md">6.3.1 Loom vs Hindsight</a>
        ├── <a href="./2.loom-vs-zep.md">6.3.2 Loom vs Zep</a>
        ├── <a href="./3.loom-vs-enterprise.md">6.3.3 Loom vs Enterprise</a>
        └── <a href="./4.hindsight-vs-zep.md">6.3.4 Hindsight vs Zep</a>
</pre>

---

## Quick Comparison

| Aspect | Loom | Hindsight | Zep | Enterprise |
|--------|------|-----------|-----|------------|
| **Filtering** | Write-time | Read-time | Read-time | Both |
| **Classification** | Zero-shot | LLM extraction | Entity-based | Policy-based |
| **Latency** | <200ms | LLM-dependent | Graph query | Variable |
| **Storage** | Stateless | Everything | Knowledge graph | Audit-complete |

---

## Key Differentiators

### Loom's Approach
- **Write-time gating** — Filter before storage, not after
- **Zero-shot classification** — No LLM calls for scoring
- **Stateless** — No conversation context (by design)

### Trade-offs
- Lower storage costs vs. potential missed context
- Fast scoring vs. simpler classification
- Independence vs. downstream integration

See [reference/](../reference/) for detailed documentation on external systems.
