---
id: 10
title: 'Report the run delta: resolved, persisted, appeared'
type: feature
status: planned
milestone: v0.1
depends_on:
- 9
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: runner
---

## Problem

A fingerprint on its own is a fact the reader still has to diff by hand. The
useful thing is the comparison between this run and the last one, which is
exactly the computation a memoryless reader cannot perform.

## Proposal

The runner persists the previous run's fingerprint set and emits a delta
alongside the records:

    since last run:  1 appeared · 0 resolved · 0 persisted
                     ↳ new failure introduced by the current change

Three states, and the third is the one no framework reports today:

- **resolved** — it is fixed
- **persisted** — your edit did nothing
- **appeared** — you moved the failure rather than fixing it

This is the loop's actual progress signal, and the first thing `agt serve`
exposes in v0.4.

## Acceptance criteria

- [ ] delta is emitted as its own record and rendered in the terminal summary
- [ ] state persists in `.agt/` across runs and survives a filter change
- [ ] a run with `--filter` reports a delta scoped to what actually ran, and says so
- [ ] humans reviewing the output find it useful too — this is not an agent-only feature
