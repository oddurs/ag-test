---
id: 8
title: Make the NDJSON event stream the only source of truth
type: feature
status: planned
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: m
area: runner
---

## Problem

In every existing framework, machine-readable output is a bolt-on that loses
fidelity relative to what the human sees. `cargo nextest`'s own docs describe its
JSON as "not full-fidelity". That guarantees the two views drift.

## Proposal

Invert it. One newline-delimited JSON record per event on stdout is the primary
output; the terminal renderer is built *over* that stream. Anything a human can
see, an agent can have — structurally, not by discipline.

`agt-core` stays dependency-light (serde only) so a CI script, an editor plugin
or another language's tooling can consume the format without pulling in a
runtime.

## Acceptance criteria

- [ ] every verdict, run-start and run-end is an NDJSON record
- [ ] `--format ndjson` emits the raw stream; the default renders it
- [ ] no field reaches the terminal that is absent from the stream
- [ ] a golden test asserts the two views carry identical information
