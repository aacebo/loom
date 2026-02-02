# Hindsight Algorithm

A biomimetic agent memory system that organizes memories into epistemically-distinct networks and retrieves them using multi-pathway fusion.

## Overview

Hindsight structures agent memory like human cognition—separating facts from experiences, observations from opinions. The system uses the TEMPR (Temporal Entity Memory Priming Retrieval) architecture for storage and retrieval, and CARA (Coherent Adaptive Reasoning Agents) for preference-conditioned reasoning.

## Architecture

```mermaid
flowchart TD
    A[Agent Interaction] --> B[Hindsight API]

    B --> C{Operation}

    C -->|Retain| D[TEMPR: Store]
    C -->|Recall| E[TEMPR: Retrieve]
    C -->|Reflect| F[CARA: Reason]

    subgraph MemoryNetworks [Memory Networks]
        W[World<br/>Objective Facts]
        X[Experience<br/>Agent Actions]
        O[Opinion<br/>Beliefs + Confidence]
        S[Observation<br/>Entity Summaries]
    end

    D --> MemoryNetworks
    E --> MemoryNetworks
    F --> MemoryNetworks

    E --> G[Reciprocal Rank Fusion]
    G --> H[Cross-Encoder Reranking]
    H --> I[Token-Limited Output]

    style W fill:#3b82f6,color:#fff
    style X fill:#22c55e,color:#fff
    style O fill:#f59e0b,color:#fff
    style S fill:#8b5cf6,color:#fff
```

## Memory Networks

Hindsight organizes memories into four epistemically-distinct networks:

```mermaid
flowchart LR
    subgraph World ["World (W)"]
        W1[Objective Facts]
        W2[Third-Person]
        W3[Environmental]
    end

    subgraph Experience ["Experience (B)"]
        X1[Agent Actions]
        X2[First-Person]
        X3[Recommendations]
    end

    subgraph Opinion ["Opinion (O)"]
        O1[Agent Beliefs]
        O2[Confidence c in 0,1]
        O3[Timestamps]
    end

    subgraph Observation ["Observation (S)"]
        S1[Entity Summaries]
        S2[Synthesized]
        S3[Preference-Neutral]
    end

    style World fill:#3b82f6,color:#fff
    style Experience fill:#22c55e,color:#fff
    style Opinion fill:#f59e0b,color:#fff
    style Observation fill:#8b5cf6,color:#fff
```

| Network | Symbol | Description | Example |
|---------|--------|-------------|---------|
| World | W | Objective, third-person environmental facts | "The stove gets hot when turned on" |
| Experience | B | First-person agent actions and outcomes | "I recommended the Italian restaurant and they loved it" |
| Opinion | O | Agent beliefs with confidence scores | "User prefers morning meetings (c=0.85)" |
| Observation | S | Synthesized entity profiles | "John: Software engineer, likes coffee, works remotely" |

## Core Operations

### Retain

Converts interactions into structured, time-aware memories.

```mermaid
flowchart TD
    A[Raw Transcript] --> B[LLM Extraction]
    B --> C[Narrative Facts<br/>2-5 per turn]

    C --> D{Classify}

    D --> E[World Facts]
    D --> F[Experience Facts]
    D --> G[Entities]
    D --> H[Temporal Metadata]

    E --> I[Memory Graph]
    F --> I
    G --> I
    H --> I

    I --> J[Embedding Generation]
    J --> K[Index Updates]
```

**Fact Representation:**

Each fact `f` contains:
- `u`: Unique identifier
- `b`: Bank identifier
- `t`: Narrative text
- `v ∈ ℝᵈ`: Embedding vector (d=384 default)
- `τ`: Temporal metadata (start/end timestamps)
- Entity links and type classifications

### Recall

Retrieves relevant memories using parallel multi-pathway search.

```mermaid
flowchart TD
    A[Query] --> B[TEMPR Retrieval]

    subgraph Channels [Parallel Channels]
        C1[Semantic<br/>Cosine Similarity]
        C2[Lexical<br/>BM25]
        C3[Graph<br/>Entity Links]
        C4[Temporal<br/>Time Windows]
    end

    B --> C1
    B --> C2
    B --> C3
    B --> C4

    C1 --> D[Reciprocal Rank Fusion]
    C2 --> D
    C3 --> D
    C4 --> D

    D --> E[Cross-Encoder Reranking]
    E --> F[Token Budget Trim]
    F --> G[Context Output]

    style C1 fill:#3b82f6,color:#fff
    style C2 fill:#22c55e,color:#fff
    style C3 fill:#f59e0b,color:#fff
    style C4 fill:#8b5cf6,color:#fff
```

### Reflect

Performs deeper analysis and belief updating.

```mermaid
flowchart TD
    A[Reflection Query] --> B[Recall Relevant Memories]
    B --> C[CARA Reasoning]

    C --> D{Evidence Type}

    D -->|Supporting| E[Increase Confidence]
    D -->|Weak| F[Decrease Confidence]
    D -->|Contradicting| G[Reduce Both<br/>c and Content]

    E --> H[Update Opinion Network]
    F --> H
    G --> H

    H --> I[Generate Response]
```

## TEMPR System

**T**emporal **E**ntity **M**emory **P**riming **R**etrieval

### Retrieval Channels

```mermaid
flowchart LR
    subgraph Semantic [Semantic Channel]
        S1[Query Embedding]
        S2[Cosine Similarity]
        S3[Vector Index]
    end

    subgraph Lexical [Lexical Channel]
        L1[Query Tokens]
        L2[BM25 Scoring]
        L3[GIN Index]
    end

    subgraph Graph [Graph Channel]
        G1[Seed Entities]
        G2[Spreading Activation]
        G3[Multi-hop Traversal]
    end

    subgraph Temporal [Temporal Channel]
        T1[Time Expression]
        T2[Hybrid Parsing]
        T3[Window Filter]
    end

    style Semantic fill:#3b82f6,color:#fff
    style Lexical fill:#22c55e,color:#fff
    style Graph fill:#f59e0b,color:#fff
    style Temporal fill:#8b5cf6,color:#fff
```

| Channel | Method | Index Type | Use Case |
|---------|--------|------------|----------|
| Semantic | Cosine similarity on embeddings | Vector (pgvector) | Conceptual similarity, paraphrasing |
| Lexical | BM25 ranking | GIN full-text | Names, technical terms, exact matches |
| Graph | Spreading activation | Entity links | Related entities, indirect connections |
| Temporal | Window filtering | B-tree timestamps | "Last spring", "yesterday", time ranges |

### Memory Graph

The memory graph `G = (V, E)` connects facts through multiple edge types:

```mermaid
flowchart TD
    subgraph Vertices [Vertices V]
        F1[Fact 1]
        F2[Fact 2]
        F3[Fact 3]
        E1[Entity: John]
    end

    F1 -->|temporal| F2
    F1 -->|semantic| F3
    F1 -->|entity| E1
    F2 -->|entity| E1
    F2 -->|causal| F3
```

| Edge Type | Connection Criteria | Weight Function |
|-----------|---------------------|-----------------|
| Temporal | Close-in-time pairs | Time decay |
| Semantic | cosine(v₁, v₂) > θₛ | Similarity score |
| Entity | Shared entity reference | 1.0 (binary) |
| Causal | Extracted cause-effect | LLM confidence |

## Scoring & Fusion

### Reciprocal Rank Fusion (RRF)

Merges results from multiple retrieval channels without score normalization:

```mermaid
flowchart LR
    subgraph Inputs [Channel Rankings]
        R1["Semantic<br/>rank(d)"]
        R2["Lexical<br/>rank(d)"]
        R3["Graph<br/>rank(d)"]
        R4["Temporal<br/>rank(d)"]
    end

    R1 --> F["RRF(d) = sum of 1/(k + rank)"]
    R2 --> F
    R3 --> F
    R4 --> F

    F --> O[Fused Ranking]
```

**Formula:**

```
RRF(d) = Σᵢ 1 / (k + rankᵢ(d))
```

Where:
- `d` = document/fact
- `k = 60` (regularization constant)
- `rankᵢ(d)` = position of d in channel i's results

**Why k = 60?**
- Balances weight between top-ranked and lower-ranked documents
- Prevents single-channel dominance
- Industry-standard value for production systems

### Cross-Encoder Reranking

After RRF fusion, a neural reranker refines the ordering:

```mermaid
flowchart LR
    A[RRF Candidates<br/>max 300] --> B[Cross-Encoder<br/>ms-marco-MiniLM]
    B --> C[Reranked Results]
    C --> D[Token Budget Trim]
    D --> E[Final Context]
```

## CARA System

**C**oherent **A**daptive **R**easoning **A**gents

### Behavioral Profiles

Each agent has a profile `Θ = (S, L, E, β)`:

```mermaid
flowchart TD
    subgraph Profile ["Behavioral Profile Θ"]
        S["S: Skepticism<br/>Cautious ↔ Trusting"]
        L["L: Literalism<br/>Strict ↔ Interpretive"]
        E["E: Empathy<br/>Feeling-aware ↔ Neutral"]
        B["β: Bias Strength<br/>Profile influence intensity"]
    end

    Profile --> R[Preference-Conditioned<br/>Reasoning]
    R --> O[Response Generation]
```

| Parameter | Low Value | High Value |
|-----------|-----------|------------|
| S (Skepticism) | Trusting, accepts claims | Cautious, requires evidence |
| L (Literalism) | Interpretive, infers intent | Strict, follows exactly |
| E (Empathy) | Neutral, task-focused | Feeling-aware, emotional |
| β (Bias) | Weak profile influence | Strong profile influence |

### Opinion Confidence Updates

Opinions maintain a confidence score `c ∈ [0, 1]` that updates with new evidence:

```mermaid
flowchart TD
    A[New Evidence] --> B{Evidence Type}

    B -->|Supporting| C["c' = c + α(1 - c)"]
    B -->|Weak| D["c' = c - δc"]
    B -->|Contradicting| E["c' = c * y<br/>content reduced"]

    C --> F[Updated Opinion]
    D --> F
    E --> F

    style C fill:#22c55e,color:#fff
    style D fill:#f59e0b,color:#fff
    style E fill:#ef4444,color:#fff
```

Where:
- `α` = learning rate for supporting evidence
- `δ` = decay rate for weak evidence
- `γ` = reduction factor for contradictions (γ < 1)

## Configuration Defaults

| Parameter | Default Value | Description |
|-----------|---------------|-------------|
| Embedding Model | BAAI/bge-small-en-v1.5 | Local embedding model |
| Embedding Dimensions | 384 | Vector size |
| Reranker | cross-encoder/ms-marco-MiniLM-L-6-v2 | Neural reranking model |
| Max Reranker Candidates | 300 | Pre-filter limit |
| RRF Constant k | 60 | Rank fusion regularization |
| Graph Retriever | link_expansion | Traversal algorithm |
| MPFP Top-K Neighbors | 20 | Graph expansion limit |
| Recall Connection Budget | 4 | Max concurrent retrievals |
| Recall Max Concurrent | 32 | Global concurrency limit |
| Retain Chunk Size | 3000 chars | Text chunking |
| Reflect Max Iterations | 10 | Tool call limit |

## Retrieval Priority

During reflection, memory tiers are prioritized:

```mermaid
flowchart LR
    A["Mental Models<br/>(User-curated)"] --> B["Observations<br/>(Synthesized)"]
    B --> C["Raw Facts<br/>(World + Experience)"]

    style A fill:#22c55e,color:#fff
    style B fill:#3b82f6,color:#fff
    style C fill:#6b7280,color:#fff
```

## Performance

> **Caveat:** These scores are self-reported by Hindsight's creators and not independently peer-reviewed.

| Benchmark | Claimed Score | Comparison |
|-----------|---------------|------------|
| LongMemEval | 91.4% | vs. 39% baseline (self-reported) |
| LoCoMo | 89.61% | vs. GPT-4o full-context (self-reported) |

## Sources

- [GitHub Repository](https://github.com/vectorize-io/hindsight)
- [Research Paper (arXiv:2512.12818)](https://arxiv.org/abs/2512.12818)
- [Official Documentation](https://hindsight.vectorize.io/)
