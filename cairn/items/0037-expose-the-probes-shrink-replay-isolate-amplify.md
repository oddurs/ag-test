---
id: 37
title: 'Expose the probes: shrink, replay, isolate, amplify'
type: feature
status: backlog
milestone: v0.4
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: m
area: protocol
---

## Problem

`next_probes` currently hands the agent shell commands to run. Over a protocol
they should be callable methods, and the set should be complete enough that the
agent never needs to invent an investigation of its own.

## Proposal

Four probes, all strictly information-gathering:

- `shrink` — a smaller witness for a fingerprint
- `replay` — re-run one fingerprint at its recorded seed
- `isolate` — re-run one case alone, to test for cross-test interference
- `amplify` — re-run under N seeds, to test for flakiness

Never `try changing X`. The framework has no opinion about the code, and an agent
that receives one from its test runner has been handed a confident guess dressed
as data.

## Acceptance criteria

- [ ] all four are callable and return structured results
- [ ] `next_probes` in the record names methods, not shell strings, when served
- [ ] each probe is bounded in time and cancellable
- [ ] a review confirms no probe suggests a code change
