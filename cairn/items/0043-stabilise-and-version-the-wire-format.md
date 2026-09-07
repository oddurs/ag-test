---
id: 43
title: Stabilise and version the wire format
type: chore
status: backlog
milestone: v1.0
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: core
---

## Problem

The format is the product. Other people's tooling — CI, editors, other languages'
runners — can only depend on it if it stops moving, and a format that breaks
quietly is worse than one that never stabilised.

## Proposal

Version the stream explicitly, commit to compatibility rules, and publish a
schema. Additive changes bump a minor; anything else is a major and is refused by
a reader that does not know it.

## Acceptance criteria

- [ ] every record carries a format version
- [ ] a reader refuses an unknown major rather than misreading it
- [ ] compatibility policy is written down before 1.0, not after
- [ ] a conformance suite exists that a third-party emitter can run
