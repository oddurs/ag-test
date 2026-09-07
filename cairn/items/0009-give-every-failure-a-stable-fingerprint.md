---
id: 9
title: Give every failure a stable fingerprint
type: feature
status: planned
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: l
area: core
---

## Problem

The reader has no memory. Nothing in a test failure tells it whether this is the
failure it was already working on, a new one it just introduced, or the same bug
relocated by its last edit. That is the most expensive failure mode in agentic
coding: the loop churns for twenty iterations without noticing it is making no
progress.

## Proposal

    fingerprint = h( case_path ‖ expectation_kind ‖ normalise(observed) ‖ failure_site )

Deliberately excluded: line numbers, timestamps, durations, thread ids, absolute
paths, allocation addresses, iteration counts. Including any of them makes an
unrelated edit look like a new bug.

Normalisation is a pipeline of rewriters applied before hashing — addresses,
temp paths, UUIDs and durations collapse to placeholders.

This is the single most load-bearing assumption in the design. Too stable and two
bugs collide under one identity; too unstable and the loop loses its history.
See docs/05-open-questions.md.

## Acceptance criteria

- [ ] the same failure fingerprints identically across machines and runs
- [ ] reformatting, renaming a local, or editing an unrelated file does not change it
- [ ] a genuinely different failure in the same case gets a different fingerprint
- [ ] fingerprinting a failure costs under 1 ms
