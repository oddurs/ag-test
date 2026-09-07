---
id: 21
title: Detect flakes and quarantine them with evidence
type: feature
status: backlog
milestone: v0.2
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: m
area: runner
---

## Problem

A flake is an annoyance to a person and poison to a control loop. Today the
framework leaves the agent to discover instability by confusion, which it does
slowly and often wrongly.

## Proposal

A case that fails and then passes on re-run is promoted to `isolated` (process
per case) and re-checked. If it is still unstable it becomes `Quarantined`, with
the evidence attached.

Quarantine's definition matters and is precise: **the case keeps running and
reporting; only its ability to block a merge changes.** Folding it into skip
loses exactly the information that makes it safe.

## Acceptance criteria

- [ ] flakes are detected by the framework, not by the reader
- [ ] `Quarantined` cases still execute and still report
- [ ] quarantining requires a recorded reason and shows up in the ledger in v0.3
- [ ] a quarantined case that becomes stable again is reported as such
