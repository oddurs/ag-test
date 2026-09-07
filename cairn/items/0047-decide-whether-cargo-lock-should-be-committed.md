---
id: 47
title: Decide whether Cargo.lock should be committed
type: chore
status: backlog
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p2
effort: s
area: harness
---

## Problem

`Cargo.lock` is gitignored, which is the library convention. But `agt` ships a
binary (`agt-cli`), and for binaries the convention is the opposite — a committed
lock file is what makes a CI failure reproducible.

Given that this project's entire thesis is reproducibility, the inconsistency is
worth resolving deliberately rather than by default.

## Acceptance criteria

- [ ] decision made and recorded here with its reasoning
- [ ] `.gitignore` matches the decision
- [ ] CI verifies the lock file is current if it is committed
