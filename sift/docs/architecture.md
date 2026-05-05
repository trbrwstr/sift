# Sift Architecture

## Overview

Sift is a high-performance log processing engine built in Rust.

It is designed around a streaming + parallel hybrid model.

---

## Pipeline

Input → Chunker → Parser → Filter → Aggregator → Output

---

## Design Principles

- Zero unnecessary allocations
- Parallel execution where safe
- Pluggable parsers
- CLI-first UX
- Deterministic outputs

---

## Performance Model

- Memory-mapped file reading (large logs)
- Rayon-based parallel chunk processing
- O(n) aggregation with merge reduction

---

## Future Extensions

- Query DSL expansion (AND/OR grouping)
- Real-time tail ingestion
- Distributed mode (optional)