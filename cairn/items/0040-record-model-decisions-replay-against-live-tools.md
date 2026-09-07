---
id: 40
title: Record model decisions, replay against live tools
type: feature
status: backlog
milestone: v0.5
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: xl
area: replay
---

## Problem

Model sampling is the fifth entropy source, and the obvious implementation is
wrong. An HTTP-level cassette that recorded a successful run keeps replaying that
success after you break the tool the model was calling — it froze the decision
and the tool execution into one opaque blob. That is the most common way an agent
test suite becomes decorative while staying green.

## Proposal

Record at the **decision boundary**: capture what the model chose (tool name,
arguments, final text), replay that, and dispatch the recorded arguments to the
**live** tool. The expensive nondeterministic part is frozen; the part actually
under test still runs.

Cassettes live in `tests/cassettes/`, one file per case, NDJSON, reviewed like
any other fixture. Keys hash the normalised request context so an unrelated
prompt edit invalidates only the cases it touches.

## Acceptance criteria

- [ ] breaking a tool's implementation makes a replayed test fail
- [ ] cassettes are readable and reviewable in a pull request
- [ ] an unrelated prompt change does not invalidate every cassette
- [ ] recording and replaying are one attribute apart
