---
id: 14
title: Detect fingerprint instability as a framework defect
type: chore
status: backlog
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: m
area: runner
---

## Problem

The design rests on fingerprint stability, and the failure mode is silent: if
fingerprints drift, the delta lies, and the loop is worse off than it was with no
delta at all.

## Proposal

Have the runner check itself. Re-run a failing case under the same seed and
compare fingerprints. A fingerprint that does not reproduce is reported as a
**framework-level defect**, not as a test failure, and is loud.

This is the cheap early-warning system for the risk in
docs/05-open-questions.md — better to discover normalisation is
domain-specific here than in the field.

## Acceptance criteria

- [ ] `agt run --verify-fingerprints` re-runs failures and compares
- [ ] instability is reported distinctly from a test failure
- [ ] on by default in CI, off by default locally (cost)
- [ ] the report names which normaliser would have been needed, where derivable
