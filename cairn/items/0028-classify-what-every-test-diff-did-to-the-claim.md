---
id: 28
title: Classify what every test diff did to the claim
type: feature
status: backlog
milestone: v0.3
depends_on:
- 26
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: l
area: ledger
---

## Problem

"The test was updated" and "the assertion was deleted" look identical in a green
checkmark. A weakened assertion hides most comfortably inside a diff that also
changes application code, where the reviewer's attention is on the feature and
the test edit reads as incidental cleanup.

## Proposal

Per-case classification on every diff touching tests:

| Movement | Trigger | Gate |
| --- | --- | --- |
| `Strengthened` | strength up, or mutation score up | auto |
| `Neutral` | claim hash stable under refactor | auto |
| `Weakened` | tolerance widened, timeout grown, generator narrowed | review |
| `Silenced` | ignore, skip or quarantine added | review |
| `Removed` | case deleted | review |
| `Undetermined` | classifier cannot decide | review |

`Undetermined` exists because this is not always decidable — replacing three
examples with one property is usually a strengthening, but not if the property is
trivial. Defaulting to review is the safe direction; the cost is review fatigue.

## Acceptance criteria

- [ ] widened tolerances, grown timeouts and added ignores are caught
- [ ] pure refactors classify as `Neutral` with no false positives on a real repo
- [ ] the classifier reports `Undetermined` rather than guessing
- [ ] measured against a hand-labelled corpus of real test diffs
