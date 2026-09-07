---
id: 18
title: Derive every random choice from one seed
type: feature
status: backlog
milestone: v0.2
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: runner
---

## Problem

The reader cannot shrug at a flake. A test that randomly passes gets attributed
to whatever the agent changed last, so it keeps a change that did nothing and
moves on, confidently wrong.

## Proposal

One root seed per run, printed and carried in every record. Scopes derive child
seeds **by label** rather than drawing from a shared stream (see
`Seed::derive`), so adding a case does not shift the random choices made by
unrelated cases. A seed that shifts under unrelated edits is not worth printing.

v0.2 scope is the cheap part of determinism: seeded RNG injection, fixed
`HashMap` seeds, `getrandom` interception. Execution order, virtual time and
mocked I/O belong to `simulated` in v0.5 and stay optional.

## Acceptance criteria

- [ ] `agt run --seed <S>` reproduces an identical event stream
- [ ] adding an unrelated case does not change another case's derived seed
- [ ] `HashMap` iteration order is fixed within a run
- [ ] the seed appears in the run header and in every evidence record
