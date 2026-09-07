---
id: 33
title: Write down when weakening a test is legitimate
type: docs
status: backlog
milestone: v0.3
created: 2026-09-07
updated: 2026-09-07
priority: p2
effort: s
area: docs
---

## Problem

If the tooling implies that weakening is always wrong, reviewers will start
rubber-stamping the warnings, and a mandatory-review mechanism that gets
rubber-stamped is worse than no mechanism.

## Proposal

Document the legitimate cases explicitly: the test encoded a behaviour that was
never required; the tolerance was wrong for the domain; the generator produced
invalid inputs; the timeout was tuned to one machine. With the corresponding
illegitimate versions of each, which look nearly identical in a diff.

## Acceptance criteria

- [ ] paired legitimate/illegitimate examples, from real diffs
- [ ] guidance on what a reviewer should ask for in each case
- [ ] linked directly from the CI review comment
