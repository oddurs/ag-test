---
id: 27
title: Track claim strength and publish the suite's profile
type: feature
status: backlog
milestone: v0.3
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: ledger
---

## Problem

Not all passing tests prove the same amount, and nothing makes the difference
visible. A suite can gain a hundred tests and lose rigour.

## Proposal

Cases declare strength — `Smoke < Example < Snapshot < Property < Proof` — via
the attribute, or have it inferred from the assertion shape where possible. The
runner reports a profile:

    strength profile:  smoke 12 · example 140 · snapshot 31 · property 30 · proof 1
                       judged 4  (2% of gates rest on an opinion)

Judged gates are counted separately on purpose. A suite whose gates are 40%
judged is a different object from one at 2%, and today nothing makes that
visible.

## Acceptance criteria

- [ ] `#[agt::test(strength = "…")]` declares it; obvious cases are inferred
- [ ] the profile is emitted as a record and rendered in the summary
- [ ] a declared strength contradicted by mutation score is corrected downward, loudly
- [ ] judged gates are counted and shown as a share
