---
id: 1
key: v0.1
title: v0.1 — Evidence
type: milestone
status: backlog
created: 2026-09-07
updated: 2026-09-07
priority: p2
due: 2026-11-30
---

## 2026-09-07

**Rung 1 of the adoption ladder: the reporter.** Runs over the tests a project already has, changes nothing about how they are written, and produces the three things a memoryless reader cannot supply for itself — a stable failure identity, a run delta, and a verdict that distinguishes a broken harness from a broken program.

Ships when a Rust project can `cargo add --dev agt`, run `agt run`, and get a delta without editing a single test. If rung 1 requires a migration, nobody reaches rung 2 — see docs/04-product.md.
