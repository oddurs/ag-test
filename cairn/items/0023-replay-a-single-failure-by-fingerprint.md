---
id: 23
title: Replay a single failure by fingerprint
type: feature
status: backlog
milestone: v0.2
depends_on:
- 18
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: s
area: cli
---

## Problem

The reproduce command re-runs a case. What the loop usually wants is narrower:
re-run *this exact failure* at the seed that produced it, without re-running
anything else.

## Proposal

    agt replay 3f1a9c2e

Looks the fingerprint up in the last run's state, restores the seed and filter,
and runs only that case. Becomes a protocol method in v0.4.

## Acceptance criteria

- [ ] replay reproduces the same fingerprint or reports that it did not
- [ ] a fingerprint not in recent state gives a clear error, not a silent pass
- [ ] works across machines when the seed is supplied explicitly
