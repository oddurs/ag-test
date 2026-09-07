---
id: 4
key: v0.4
title: v0.4 — Protocol
type: milestone
status: backlog
created: 2026-09-07
updated: 2026-09-07
priority: p2
due: 2027-07-31
---

## 2026-09-07

**The agent stops screen-scraping its own test framework.** Everything in v0.1–v0.3 is reachable through a stdio JSON-RPC service instead of a pipe.

Two methods carry this milestone. `delta` is the memory prosthetic — the first call a well-behaved loop makes after a run. `propose_case` converts *write a test that passes* into *write a test that kills mutants*, which is a materially harder thing to game.

Deliberately after the stream, not before: if agent harnesses never adopt a protocol, the NDJSON stream still works. See docs/05-open-questions.md.
