---
id: 17
title: Write the rung-1 quickstart
type: docs
status: backlog
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: s
area: docs
---

## Problem

The adoption ladder in docs/04-product.md only works if the first rung is
genuinely trivial. Documentation is where that promise gets tested.

## Proposal

A quickstart that goes from `cargo add --dev agt` to a first run delta in under
five minutes, on an existing project, with no test rewrites and no config file.

If writing it honestly requires a "first, migrate your tests" step, that is a
design bug in v0.1, not a documentation problem, and it should be filed as one.

## Acceptance criteria

- [ ] fewer than 10 lines of shell, no config file
- [ ] runs against a real third-party crate, not a toy
- [ ] states plainly what rung 1 does *not* give you
