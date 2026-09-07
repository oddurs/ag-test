---
id: 24
title: Add process-per-case isolation
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

Cases that touch process globals, environment variables, or `install`-style
singletons interfere with each other, and cross-test interference is exactly the
kind of failure a reader with no intuition mis-attributes.

## Proposal

`#[agt::test(isolation = "isolated")]` runs the case in its own process.
Automatically promoted for any case implicated in flake detection. Costs
milliseconds rather than microseconds, so it stays opt-in.

Pairs with the `isolate` probe: re-run one case alone to test whether the failure
is really about that case.

## Acceptance criteria

- [ ] an isolated case cannot observe another case's global state
- [ ] crashes and timeouts in a child are reported as verdicts, not runner failures
- [ ] overhead stays around a millisecond
- [ ] `agt run --isolate <CASE>` works as a one-off probe
