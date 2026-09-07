---
id: 38
title: Publish a reference agent loop
type: docs
status: backlog
milestone: v0.4
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: s
area: docs
---

## Problem

A protocol nobody knows how to drive is a protocol nobody drives. The loop that
uses it well is not obvious — in particular, calling `delta` before reading any
evidence is the counter-intuitive step that makes the whole thing pay off.

## Proposal

A worked example: a small agent that fixes a seeded bug using only protocol
calls, with the transcript included, and commentary on why the ordering is what
it is.

## Acceptance criteria

- [ ] runnable against this repository
- [ ] shows delta-first ordering and explains why
- [ ] shows the correct response to `Inconclusive`
- [ ] shows `propose_case` rejecting a decorative test
