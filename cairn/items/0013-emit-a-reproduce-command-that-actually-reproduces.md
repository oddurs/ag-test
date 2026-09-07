---
id: 13
title: Emit a reproduce command that actually reproduces
type: feature
status: backlog
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: s
area: runner
---

## Problem

"Re-run this failure" is currently a hope. Frameworks print the test name and
leave the reader to reconstruct the invocation, which fails as soon as filters,
seeds or environment matter.

## Proposal

Every evidence record carries a `reproduce` string that can be pasted verbatim:

    agt run --seed 4f2a1c9e00b3d551 parser::rejects_trailing_comma

Only honest once runs are deterministic, so v0.1 emits it with whatever seed
discipline exists and v0.2 makes it a promise. Until then the record states its
own confidence rather than overclaiming.

## Acceptance criteria

- [ ] the string is executable as printed, including under filters
- [ ] shell-quoting is correct for case names containing `::` and generics
- [ ] a test asserts that running the string reproduces the same fingerprint
- [ ] where reproduction is not yet guaranteed, the record says so explicitly
