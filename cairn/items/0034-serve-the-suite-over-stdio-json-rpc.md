---
id: 34
title: Serve the suite over stdio JSON-RPC
type: feature
status: backlog
milestone: v0.4
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: l
area: protocol
---

## Problem

An agent should not shell out and regex the output of its own test framework.
Every loop in production today does exactly that, and every one of them
re-implements the same fragile parser.

## Proposal

`agt serve` speaking line-delimited JSON-RPC over stdio, MCP-shaped, so it drops
into an existing tool loop as a server rather than a subprocess to be
screen-scraped.

This is a bet on other people's roadmaps, which is why it comes after the stream
rather than before: if harnesses never adopt a protocol, the NDJSON stream still
works. See docs/05-open-questions.md.

## Acceptance criteria

- [ ] `list_cases`, `run` (streaming) and `evidence` work over stdio
- [ ] MCP-compatible enough to register as a tool server without a shim
- [ ] a crashed or hung run surfaces as a protocol error, never a hang
- [ ] concurrent requests during a long run are handled or explicitly refused
