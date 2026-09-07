---
id: 2
key: v0.2
title: v0.2 — Determinism
type: milestone
status: backlog
created: 2026-09-07
updated: 2026-09-07
priority: p2
due: 2027-02-28
---

## 2026-09-07

**Rung 2: seeds and shrinking.** Turns the reproduce command from an aspiration into a promise, and makes the framework rather than the agent responsible for noticing flakes.

The reader cannot shrug at a flaky test, so noise here is not an annoyance but corruption of the signal driving a control loop. Scope is deliberately the cheap four-fifths of determinism: seeded randomness, per-scope seed derivation, shrinking, flake detection. Full simulation is v0.5 and stays optional forever.
