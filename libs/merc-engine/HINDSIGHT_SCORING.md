# Hindsight Scoring & Evaluation

How Hindsight scores, ranks, and evaluates memories during retrieval.

## Scoring Pipeline

```mermaid
flowchart TD
    A[Query] --> B[4 Parallel Retrievers]

    subgraph Retrieval [Channel Scores]
        B1[Semantic<br/>cosine similarity]
        B2[Lexical<br/>BM25 score]
        B3[Graph<br/>activation score]
        B4[Temporal<br/>recency score]
    end

    B --> B1
    B --> B2
    B --> B3
    B --> B4

    B1 --> C[Convert to Rankings]
    B2 --> C
    B3 --> C
    B4 --> C

    C --> D[Reciprocal Rank Fusion]
    D --> E[Cross-Encoder Reranking]
    E --> F[Final Scored Results]

    style D fill:#3b82f6,color:#fff
    style E fill:#22c55e,color:#fff
```

## Channel Scoring

### 1. Semantic Scoring

Measures conceptual similarity using embedding vectors.

```mermaid
flowchart LR
    A[Query] --> B[Embed: v_q]
    C[Memory] --> D[Embed: v_m]
    B --> E["score = cos(v_q, v_m)"]
    D --> E
    E --> F["score in -1, 1"]
```

**Formula:**

```
semantic_score(q, m) = (v_q · v_m) / (||v_q|| × ||v_m||)
```

Where:
- `v_q` = query embedding vector (ℝ³⁸⁴)
- `v_m` = memory embedding vector (ℝ³⁸⁴)

### 2. Lexical Scoring (BM25)

Measures term-based relevance using Okapi BM25.

```mermaid
flowchart LR
    A[Query Terms] --> B[Term Frequency]
    C[Memory Text] --> D[Document Frequency]
    B --> E[BM25 Score]
    D --> E
    E --> F["score >= 0"]
```

**Formula:**

```
BM25(q, m) = Σ IDF(t) × (f(t,m) × (k₁ + 1)) / (f(t,m) + k₁ × (1 - b + b × |m|/avgdl))
```

Where:
- `t` = query term
- `f(t,m)` = term frequency in memory m
- `|m|` = memory length
- `avgdl` = average document length
- `k₁ = 1.2`, `b = 0.75` (standard parameters)

### 3. Graph Scoring

Spreading activation across memory graph edges.

```mermaid
flowchart TD
    A[Seed Entities] --> B[Initial Activation = 1.0]
    B --> C{Edge Type}

    C -->|Temporal| D["score * decay"]
    C -->|Semantic| E["score * similarity"]
    C -->|Entity| F["score * 1.0"]
    C -->|Causal| G["score * confidence"]

    D --> H[Propagate to Neighbors]
    E --> H
    F --> H
    G --> H

    H --> I{Iterations < max?}
    I -->|Yes| C
    I -->|No| J[Final Activation Scores]
```

**Activation Decay:**

```
activation(n, t+1) = Σ activation(m, t) × edge_weight(m, n) × damping
```

Where `damping = 0.85` (prevents infinite propagation)

### 4. Temporal Scoring

Filters and scores by time relevance.

```mermaid
flowchart LR
    A[Query Time Expression] --> B[Parse to Window]
    B --> C["[start, end]"]
    C --> D{Memory timestamp in window?}
    D -->|Yes| E[score = recency_decay]
    D -->|No| F[score = 0]
```

**Recency Decay:**

```
temporal_score(m) = e^(-λ × age(m))
```

Where:
- `λ` = decay rate
- `age(m)` = time since memory creation

## Reciprocal Rank Fusion (RRF)

Combines rankings from all channels without score normalization.

```mermaid
flowchart TD
    subgraph Rankings [Per-Channel Rankings]
        R1["Semantic: [m₃, m₁, m₇, ...]"]
        R2["Lexical: [m₁, m₃, m₂, ...]"]
        R3["Graph: [m₇, m₁, m₃, ...]"]
        R4["Temporal: [m₂, m₃, m₁, ...]"]
    end

    R1 --> F[RRF Fusion]
    R2 --> F
    R3 --> F
    R4 --> F

    F --> O["Fused: [m₁, m₃, m₇, m₂, ...]"]
```

**Formula:**

```
RRF(m) = Σᵢ 1 / (k + rankᵢ(m))
```

**Example Calculation:**

For memory `m₁` with ranks: Semantic=2, Lexical=1, Graph=2, Temporal=3

```
RRF(m₁) = 1/(60+2) + 1/(60+1) + 1/(60+2) + 1/(60+3)
        = 1/62 + 1/61 + 1/62 + 1/63
        = 0.0161 + 0.0164 + 0.0161 + 0.0159
        = 0.0645
```

**Why k = 60?**

```mermaid
flowchart LR
    subgraph KValues [Effect of k]
        K1["k=1: Top ranks dominate"]
        K2["k=60: Balanced weighting"]
        K3["k=1000: Nearly uniform"]
    end

    K2 --> O[Production Default]
    style K2 fill:#22c55e,color:#fff
```

| k Value | Behavior | Use Case |
|---------|----------|----------|
| Small (1-10) | Top-1 dominates | High precision needed |
| Medium (60) | Balanced | General retrieval |
| Large (100+) | Flattened ranks | Diversity prioritized |

## Cross-Encoder Reranking

Neural reranker scores query-memory pairs directly.

```mermaid
flowchart TD
    A[RRF Top-300 Candidates] --> B[Cross-Encoder Model]

    subgraph Model [ms-marco-MiniLM-L-6-v2]
        C["Input: [CLS] query [SEP] memory [SEP]"]
        D[Transformer Encoding]
        E[Relevance Score]
    end

    B --> C
    C --> D
    D --> E

    E --> F[Reranked Results]
    F --> G[Token Budget Trim]
```

**Scoring:**

```
rerank_score(q, m) = sigmoid(model([CLS] q [SEP] m [SEP]))
```

Output: `score ∈ [0, 1]` representing relevance probability

**Configuration:**

| Parameter | Default | Description |
|-----------|---------|-------------|
| Max Candidates | 300 | Pre-filter before reranking |
| Max Concurrent | 4 | Prevent CPU thrashing |
| Batch Size | 128 | TEI batching |

## Opinion Confidence Scoring

Beliefs maintain confidence scores that update with evidence.

```mermaid
flowchart TD
    A[Opinion: c = 0.7] --> B[New Evidence]

    B --> C{Evidence Type}

    C -->|Strong Support| D["c' = 0.76"]
    C -->|Weak Support| E["c' = 0.715"]
    C -->|Neutral| F["c' = 0.7"]
    C -->|Contradiction| G["c' = 0.35"]

    style D fill:#22c55e,color:#fff
    style E fill:#3b82f6,color:#fff
    style F fill:#6b7280,color:#fff
    style G fill:#ef4444,color:#fff
```

**Update Rules:**

| Evidence | Formula | Effect |
|----------|---------|--------|
| Strong support | `c' = c + α(1 - c)` | Approaches 1.0 |
| Weak support | `c' = c + α'(1 - c)` | Slow increase |
| Contradiction | `c' = c × γ` | Multiplicative decay |
| Strong contradiction | Content also reduced | Opinion weakened |

Where:
- `α` = learning rate (strong) ≈ 0.2
- `α'` = learning rate (weak) ≈ 0.05
- `γ` = contradiction decay ≈ 0.5

## Evaluation Benchmarks

> **Caveat:** These scores are self-reported by Hindsight's creators and have not been independently peer-reviewed. The benchmark methodology was also designed by the same team. Treat these numbers as claims, not verified facts.

### LongMemEval

Tests long-term memory across conversational AI scenarios.

```mermaid
flowchart LR
    subgraph Tasks [Evaluation Tasks]
        T1[Single-Session<br/>Recall]
        T2[Multi-Session<br/>Reasoning]
        T3[Temporal<br/>Queries]
        T4[Entity<br/>Tracking]
    end

    T1 --> M[Accuracy Metric]
    T2 --> M
    T3 --> M
    T4 --> M

    M --> R[Final Score]
```

**Claimed Results (unverified):**

| System | Accuracy |
|--------|----------|
| Baseline (RAG) | 39% |
| Hindsight (20B) | 83.6% |
| Hindsight (Gemini-3) | 91.4% |

### LoCoMo

Tests long-context conversation modeling.

```mermaid
flowchart LR
    A[Long Conversation] --> B[Memory System]
    B --> C[Query Response]
    C --> D[Ground Truth Compare]
    D --> E[Accuracy Score]
```

**Claimed Results (unverified):**

| System | Accuracy |
|--------|----------|
| GPT-4o (full context) | 85.2% |
| Hindsight | 89.61% |

## Scoring Summary

```mermaid
flowchart TD
    subgraph Stage1 [Stage 1: Channel Scoring]
        A1[Semantic: cosine]
        A2[Lexical: BM25]
        A3[Graph: activation]
        A4[Temporal: decay]
    end

    subgraph Stage2 [Stage 2: Rank Fusion]
        B1["RRF: sum 1/(k + rank)"]
    end

    subgraph Stage3 [Stage 3: Neural Rerank]
        C1[Cross-Encoder]
    end

    subgraph Stage4 [Stage 4: Output]
        D1[Token-Limited Context]
    end

    Stage1 --> Stage2
    Stage2 --> Stage3
    Stage3 --> Stage4

    style Stage1 fill:#3b82f6,color:#fff
    style Stage2 fill:#22c55e,color:#fff
    style Stage3 fill:#f59e0b,color:#fff
    style Stage4 fill:#8b5cf6,color:#fff
```

| Stage | Method | Output |
|-------|--------|--------|
| 1. Channel Scoring | Cosine, BM25, Activation, Decay | Raw scores per channel |
| 2. Rank Fusion | RRF (k=60) | Unified ranking |
| 3. Neural Rerank | Cross-encoder | Refined ranking |
| 4. Output | Token budget trim | Final context |
