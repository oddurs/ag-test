---
id: 12
title: Render the terminal view over the event stream
type: feature
status: backlog
milestone: v0.1
created: 2026-09-07
updated: 2026-09-07
priority: p1
effort: m
area: cli
---

## Problem

If the human view is a separate code path from the machine view, they drift, and
the machine view is always the one that rots.

## Proposal

`agt-cli` consumes the NDJSON stream and renders it. No direct access to runner
internals. The renderer is therefore also the reference implementation of the
format, and anyone can write another one.

Shape (see docs/04-product.md):

    ✗ parser::rejects_trailing_comma          property   3f1a9c2e
        expected  every accepted input round-trips to itself
        observed  accepted `[1,]`
        witness   [1,]                        (shrunk from 412 bytes)
        repro     agt run --seed 4f2a1c9e00b3d551 parser::rejects_trailing_comma

## Acceptance criteria

- [ ] `agt run --format ndjson | agt render` is byte-identical to `agt run`
- [ ] colour degrades correctly under `NO_COLOR` and when not a TTY
- [ ] the delta block appears in the summary
- [ ] no renderer-only fields exist
