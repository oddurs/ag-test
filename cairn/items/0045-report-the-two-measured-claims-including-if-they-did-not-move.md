---
id: 45
title: Report the two measured claims, including if they did not move
type: chore
status: backlog
milestone: v1.0
depends_on:
- 15
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: docs
---

## Problem

docs/04-product.md commits to two numbers: fewer loop iterations per resolved
failure, and fewer merged diffs that weakened an oracle. A project that quietly
stops mentioning its own success criteria has answered the question.

## Proposal

Re-measure against the v0.1 baseline and publish, whatever the result. If neither
number moved, say so here and in the README, and treat it as evidence the thesis
is wrong rather than as a prompt to add features.

## Acceptance criteria

- [ ] both numbers re-measured with the same method as the baseline
- [ ] published in the repository, not only in a post
- [ ] a negative result is stated plainly in the README
