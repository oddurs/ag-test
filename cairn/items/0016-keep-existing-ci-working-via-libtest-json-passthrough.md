---
id: 16
title: Keep existing CI working via libtest-json passthrough
type: feature
status: backlog
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p2
effort: m
area: cli
---

## Problem

Rung 1 promises that nothing else in the project changes. That is false if
adopting `agt` breaks the CI that consumes test results today.

## Proposal

Emit `libtest-json` alongside the native stream, and JUnit XML, which is what
most CI actually reads. Explicitly a compatibility shim, not the format the
design cares about.

RFC 3558 is unfinished upstream. When a first-class NDJSON run format lands in
libtest, `agt` should consume it rather than maintain a parallel one — worth
revisiting rather than entrenching.

## Acceptance criteria

- [ ] `--message-format libtest-json` matches what existing tooling expects
- [ ] JUnit XML output validates against the schema CI systems accept
- [ ] the shim is documented as a shim, with the upstream tracking link
