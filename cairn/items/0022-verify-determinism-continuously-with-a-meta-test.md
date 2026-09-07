---
id: 22
title: Verify determinism continuously with a meta-test
type: feature
status: backlog
milestone: v0.2
depends_on:
- 18
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: ci
---

## Problem

Determinism that is not continuously verified decays within weeks. Every team
that has built a deterministic simulator says the same thing, and they were
motivated database engineers.

## Proposal

A meta-test in `agt`'s own CI: run the suite twice under the same seed, compare
the event streams byte-for-byte, fail the build on any difference. Ship it as a
public helper so downstream projects can run the same check on their suites.

## Acceptance criteria

- [ ] same seed produces byte-identical NDJSON across two runs
- [ ] the check runs on every PR to this repository
- [ ] a divergence report names the first differing record
- [ ] exposed as `agt verify-determinism` for downstream use
