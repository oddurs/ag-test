---
id: 42
title: Gate nondeterministic cases statistically
type: feature
status: backlog
milestone: v0.5
created: 2026-09-07
updated: 2026-09-07
priority: p2
effort: l
area: core
---

## Problem

Where a case is genuinely nondeterministic — a judged rubric, a sampled model — a
single run proves nothing, and an equality assertion over one sample is theatre.

## Proposal

`#[agt::judged(rubric = "…", batch = 100, threshold = 0.9)]`. The case declares a
batch size and a threshold rather than an equality, and the evidence record
carries the observed distribution rather than one sample. N ≥ 100 is the figure
the practitioner literature converges on for merge-blocking decisions.

`Expectation::Judged` stays a separate variant so a reviewer can always ask which
gates rest on execution and which on an opinion.

## Acceptance criteria

- [ ] batch execution with a threshold, not an equality
- [ ] the record carries the distribution, not a single sample
- [ ] judged gates are counted separately in the strength profile
- [ ] the cost of a batch is visible before it is run
