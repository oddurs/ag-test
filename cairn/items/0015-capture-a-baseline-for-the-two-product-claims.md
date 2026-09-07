---
id: 15
title: Capture a baseline for the two product claims
type: chore
status: backlog
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: m
area: ci
---

## Problem

docs/04-product.md commits to two measurable numbers — loop iterations per
resolved failure, and merged diffs that weakened an oracle. Neither has been
measured. Measurement that arrives after the design is fixed is not measurement,
it is decoration.

## Proposal

Instrument before the framework can influence the numbers:

1. pick two real Rust repositories with agentic development happening
2. record iterations-per-resolved-failure from existing agent logs
3. hand-classify a month of merged test diffs for weakening

Re-measure at v0.3 and report in v1.0, including if the numbers did not move.

## Acceptance criteria

- [ ] baseline captured for at least two codebases, with method written down
- [ ] the classification rubric is published so someone else can dispute it
- [ ] a re-measurement date is scheduled against the v0.3 milestone
