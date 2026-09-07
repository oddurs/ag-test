---
id: 46
title: Choose a real name
type: chore
status: backlog
milestone: v1.0
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: s
area: docs
---

## Problem

The crate is `agt` because the repository is `ag-test`. Neither says anything,
and a name chosen under deadline pressure at 1.0 will be worse than one chosen
deliberately now that the shape is known.

## Proposal

Pick a name after the API stabilises but before 1.0, when what the thing does is
settled. Rename the crate, the repository and the binary in one change, with
redirects.

Constraint: the name should point at the *reader*, not at AI. Anything with
"agent" in it will read as dated within two years.

## Acceptance criteria

- [ ] name is available on crates.io and as a GitHub org or repo
- [ ] rename lands in one commit with redirects in place
- [ ] docs, README and roadmap updated together
