---
id: 41
title: Define cassette miss and divergence policy
type: feature
status: backlog
milestone: v0.5
depends_on:
- 40
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: replay
---

## Problem

Replay assumes a decision is meaningful independent of the tool's response. That
holds for one call. Over a long trajectory it weakens: if the live tool now
returns something different, the *next* recorded decision was made in a context
that no longer exists.

## Proposal

- **A miss is `Inconclusive`, never `Fail`.** A missing recording is a harness
  problem, and reporting it as a failure invites the agent to edit application
  code in response.
- **Divergence is per-case policy**: `strict` (any mismatch is `Inconclusive`) or
  `until-divergence` (replay to first mismatch, then live-sample and re-record).
- **`strict` is the default**, because a silently degraded determinism guarantee
  is worse than a loud one.

This makes replay honestly a cost optimisation with a determinism ceiling rather
than a determinism guarantee, and the docs should keep saying that.

## Acceptance criteria

- [ ] a cassette miss produces `Inconclusive` with the missing key named
- [ ] both policies are implemented and selectable per case
- [ ] `until-divergence` re-records and reports that it did
- [ ] the ceiling is documented where users will actually read it
