---
id: 20
title: Shrink on failure by default
type: feature
status: backlog
milestone: v0.2
depends_on:
- 19
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: runner
---

## Problem

The minimised input is worth more to a repair loop than the original input ever
was, and yet shrinking is conventionally something the reader asks for after the
fact. A reader with no memory does not know to ask.

## Proposal

Shrink automatically when a case fails, and put the result in the record's
`witness` field with the original size noted:

    witness   [1,]                        (shrunk from 412 bytes)

Bounded by a time budget so a pathological shrink cannot stall the loop; on
timeout the record carries the best witness found and says it is not minimal.

## Acceptance criteria

- [ ] failing property cases shrink without being asked
- [ ] `witness` records both the minimised value and the original size
- [ ] shrinking is deterministic under a fixed seed
- [ ] the budget is configurable and a truncated shrink is labelled as such
