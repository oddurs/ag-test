---
id: 7
title: Run the tests a project already has
type: feature
status: planned
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: l
area: runner
---

## Problem

Rung 1 of the adoption ladder has to work on an unmodified codebase. A framework
that needs a migration before it produces any value is a framework nobody
migrates to.

## Proposal

A custom harness (`harness = false`) that discovers and executes existing
`#[test]` functions, plus `#[agt::test]` for cases that want to declare more.
Custom rather than a libtest shim because the information the evidence record
needs — the claim, the generator, the seed scope — has to be captured at
assertion time, and libtest has nowhere to put it.

Default isolation is `inline` (same process, same thread). `isolated`
(process-per-case) lands in v0.2.

## Acceptance criteria

- [ ] `cargo add --dev agt && agt run` executes an existing suite with no source changes
- [ ] plain `#[test]` functions produce `Pass` / `Fail` verdicts
- [ ] panics, assertion failures and timeouts are distinguished
- [ ] per-case overhead at `inline` is under 50 µs (see the performance budget in docs/03-architecture.md)
