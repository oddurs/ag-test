---
id: 36
title: Let an agent score a candidate test before committing it
type: feature
status: backlog
milestone: v0.4
depends_on:
- 34
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: l
area: protocol
---

## Problem

The reward-hacking loop is open at exactly one point: an agent writes a test,
the test passes, the test counts. Nothing sits between "written" and "counted".

## Proposal

`propose_case` accepts a candidate test, runs it, mutation-scores it, and returns
the score **without committing it**. The agent finds out whether the test it just
wrote is worth anything before it enters the suite, and the framework can decline
to count a worthless one.

Structurally this is the most valuable method in the protocol: it changes the
objective from *write a test that passes* to *write a test that kills mutants*.

## Acceptance criteria

- [ ] a candidate is compiled and scored without touching the working tree
- [ ] the response includes killed/total and the strength it would be recorded at
- [ ] a candidate that fails to compile returns a diagnostic, not an error
- [ ] scoring is bounded in time and says so when truncated
