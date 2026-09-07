---
id: 29
title: Score tests by mutation, scoped to the diff
type: feature
status: backlog
milestone: v0.3
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: l
area: ledger
---

## Problem

Coverage cannot tell a real test from a decorative one, and the documented LLM
failure mode is precisely a decorative test: a property asserting that the
function returns a value of the right type.

## Proposal

Build on `cargo-mutants`, scoped to the diff so it stays affordable:

1. generate mutants in the code the changed tests cover
2. run only those tests against them
3. record killed/total per case in the ledger

Also the empirical tiebreaker that lets `Undetermined` resolve itself: did the
change kill more mutants, or fewer?

Target is under 60 seconds, which is the number that decides whether this is
usable interactively or only in CI.

## Acceptance criteria

- [ ] diff-scoped run completes in under 60 s on a mid-size crate
- [ ] scores land in the ledger and appear in movement classification
- [ ] uses the nextest integration where the project already uses nextest
- [ ] a timeout degrades to "unscored", never to a false pass
