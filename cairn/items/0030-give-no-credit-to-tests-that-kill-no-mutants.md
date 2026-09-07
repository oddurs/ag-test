---
id: 30
title: Give no credit to tests that kill no mutants
type: feature
status: backlog
milestone: v0.3
depends_on:
- 29
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: m
area: ledger
---

## Problem

If a new test counts simply by existing and passing, an agent optimising toward
green will write tests that pass rather than tests that check. That is the
cheapest possible reward hack and it is currently free.

## Proposal

A newly added case that kills zero mutants earns no credit toward the gate and is
recorded as `Smoke` regardless of what it declares. Structurally this converts
"write a test that passes" into "write a test that kills mutants", which is a
materially harder thing to game.

Not a hard rejection — some genuinely valuable tests are unmutatable — but it is
never counted as strengthening, and it is visible.

## Acceptance criteria

- [ ] zero-kill new cases are reported as `Smoke` with the reason attached
- [ ] the case still runs and still reports
- [ ] a legitimate unmutatable case can be annotated, and the annotation is a reviewed movement
