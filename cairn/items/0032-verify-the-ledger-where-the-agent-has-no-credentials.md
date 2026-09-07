---
id: 32
title: Verify the ledger where the agent has no credentials
type: feature
status: backlog
milestone: v0.3
depends_on:
- 26
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: ci
---

## Problem

The ledger is a file in a repository the agent can write to. A referee inside the
sandbox is not a referee.

## Proposal

Move verification to CI:

- commit the ledger hash and re-derive it there
- recompute mutation scores in CI; never trust the working tree's numbers
- fail the build on a ledger change with no corresponding test change

This does not fully solve it, and the docs should keep saying so — a sufficiently
capable agent optimising against any fixed oracle will find the gap. It moves the
referee out of the sandbox, which is the part that is achievable.

## Acceptance criteria

- [ ] a hand-edited ledger fails CI
- [ ] mutation scores are always recomputed, never read from the commit
- [ ] the failure message explains what was tampered with
- [ ] documented honestly, including what it does not protect against
