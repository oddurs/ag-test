---
id: 39
title: Add the simulated isolation level
type: feature
status: backlog
milestone: v0.5
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: xl
area: runner
---

## Problem

Concurrency, timeouts and distributed logic produce exactly the failures that are
hardest for a reader with no intuition — nondeterministic, unreproducible, and
easily mis-attributed to the last edit.

## Proposal

`isolation = "simulated"`: seeded single-threaded scheduler, virtual clock that
advances only on explicit await, mocked I/O. Build on `madsim`/`turmoil` rather
than re-deriving them — owning an entire dependency tree's entropy is the
expensive part of that work and it has been done.

Correctly the last rung. Most teams should never adopt it, and the docs should
say so.

## Acceptance criteria

- [ ] a seeded run of a concurrent test is byte-reproducible
- [ ] the virtual clock makes timeout tests instant rather than slow
- [ ] a known race is caught and reproduced from its seed alone
- [ ] the cost of adoption is documented honestly, including known leaks
