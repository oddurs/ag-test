---
id: 26
title: Build the oracle ledger
type: feature
status: backlog
milestone: v0.3
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: l
area: ledger
---

## Problem

Nothing today tracks what a test suite *claims*. Coverage counts lines executed,
which is why "coverage went up" can coexist with "every new test is a smoke
test". Under an agent that can edit the suite, an untracked oracle is an
unguarded one.

## Proposal

`.agt/ledger.json`, one entry per case:

    {"case":"parser::rejects_trailing_comma","strength":"property",
     "claim_hash":"9a1c…","mutants_killed":14,"mutants_total":17,
     "quarantined":false,"last_verified":"2026-09-07T16:33:58Z"}

`claim_hash` is over the assertion's **normalised AST**, not its source text, so
renaming a local or reformatting is neutral while widening a comparison is not.

## Acceptance criteria

- [ ] the ledger is generated, committed, and human-readable in a diff
- [ ] `claim_hash` is stable under formatting and local renames
- [ ] a case missing from the ledger is reported, not silently ignored
- [ ] regenerating it twice produces identical bytes
