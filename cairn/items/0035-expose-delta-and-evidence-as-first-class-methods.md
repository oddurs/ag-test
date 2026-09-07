---
id: 35
title: Expose delta and evidence as first-class methods
type: feature
status: backlog
milestone: v0.4
depends_on:
- 34
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: protocol
---

## Problem

The run delta is the memory prosthetic — the single computation a memoryless
reader cannot do for itself. Making an agent recover it by diffing two text
outputs reintroduces exactly the parsing step the protocol exists to remove.

## Proposal

`delta` returns fingerprints resolved / persisted / appeared since a prior run,
and is intended to be the first call a well-behaved loop makes after `run`.
`evidence` returns the full record for one fingerprint, so the loop can fetch
detail on demand rather than being handed everything.

Document the reference loop: `run` → `delta` → `evidence` → `shrink` → edit →
`run`.

## Acceptance criteria

- [ ] `delta` accepts an explicit prior-run id, not just "the last one"
- [ ] `evidence` works for any fingerprint in retained state
- [ ] retention policy for prior runs is configurable and documented
- [ ] the reference loop is written down and tested end to end
