---
id: 11
title: Distinguish a broken harness from a broken program
type: feature
status: planned
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: runner
---

## Problem

A human reads `No such file or directory (os error 2)` inside a test and knows
instantly that the fixture is missing and the application code is fine. That
inference is free, unconscious, and completely unavailable to an agent reading
the same red text. It sees a failing test and starts editing the code the test
names.

Almost every framework collapses "the claim was violated" and "the case could not
be judged" into one undifferentiated red.

## Proposal

`Verdict::Inconclusive { reason }` as a first-class outcome, produced for:
missing fixtures, absent dependencies, harness panics, setup failures, and (from
v0.5) cassette misses.

Rendered distinctly (`⊘`), counted separately, and — importantly — **not
merge-blocking in the same way as a failure**, because the correct response is
to fix the harness, not the code.

This is the highest-value variant in the enum and the one most frameworks lack.

## Acceptance criteria

- [ ] I/O errors in setup produce `Inconclusive`, not `Fail`
- [ ] the reason string names the missing resource
- [ ] the terminal summary counts the four verdicts separately
- [ ] documented for agents: `Inconclusive` means fix the harness, never the code under test
