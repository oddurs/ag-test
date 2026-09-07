---
id: 5
key: v0.5
title: v0.5 — Simulation
type: milestone
status: backlog
created: 2026-09-07
updated: 2026-09-07
priority: p2
due: 2027-10-31
---

## 2026-09-07

**Rung 4: the expensive one, and correctly the last.** Deterministic simulation for concurrency and distributed logic, plus decision-boundary replay for model sampling — the fifth entropy source classical simulation testing never had to model.

The seam matters more than the machinery: record what the model *decided* and replay that against the **live** tool. An HTTP-level cassette freezes the decision and the tool execution into one blob, so the suite keeps passing after the tool breaks. That is the most common way an agent test suite becomes decorative while staying green.

Most teams should never adopt this milestone, and the documentation should say so.
