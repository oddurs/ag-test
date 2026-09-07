---
id: 25
title: Document what determinism does and does not cover
type: docs
status: backlog
milestone: v0.2
created: 2026-09-07
updated: 2026-09-07
priority: p2
effort: s
area: docs
---

## Problem

Determinism is the easiest thing in this design to oversell, and an overclaimed
guarantee is worse than none: it converts a known risk into an assumed safety.

## Proposal

State plainly which entropy sources are controlled at each isolation level, what
`inline` does not control, that decision replay is a cost optimisation with a
determinism ceiling rather than a guarantee, and that full simulation is v0.5 and
optional forever.

## Acceptance criteria

- [ ] a table of the five entropy sources against the three isolation levels
- [ ] the known leaks are named rather than omitted
- [ ] linked from the quickstart, not buried in the architecture doc
