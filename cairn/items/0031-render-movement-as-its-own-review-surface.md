---
id: 31
title: Render movement as its own review surface
type: feature
status: backlog
milestone: v0.3
depends_on:
- 28
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: m
area: ci
---

## Problem

Splitting the review surface is the entire mechanism. If movement information
lands inline in the code diff, it inherits exactly the inattention that lets a
weakened assertion through today.

## Proposal

A CI job posting a separate summary:

    ⚠ store::handles_conflict          WEAKENED       assert_eq → assert!(…is_ok())
        mutation score 12/14 → 3/14
        this is the only case covering conflict resolution

The last line matters: a weakening is much more serious when the case is the only
one covering that path, and the ledger knows which those are.

## Acceptance criteria

- [ ] the summary is a separate comment, not inline annotations
- [ ] auto-classified movements are collapsed; review-needed ones are not
- [ ] coverage-uniqueness is noted where the ledger can determine it
- [ ] silence when nothing needs review — no noise on ordinary PRs
