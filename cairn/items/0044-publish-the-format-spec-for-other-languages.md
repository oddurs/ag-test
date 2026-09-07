---
id: 44
title: Publish the format spec for other languages
type: docs
status: backlog
milestone: v1.0
created: 2026-09-07
updated: 2026-09-07
priority: p2
effort: m
area: docs
---

## Problem

The thesis is about the reader, not about Rust. If the argument is right it
applies to every ecosystem, and `agt` should not be the only thing that can speak
it.

## Proposal

A standalone specification of the evidence stream — records, fingerprint
construction, verdict semantics — written so a JS or Python runner could emit it
without reading Rust.

Explicitly a non-goal to *write* those runners (see docs/04-product.md). Someone
else should, after the format has proven itself.

## Acceptance criteria

- [ ] the spec stands alone, with no Rust in it
- [ ] fingerprint construction is specified precisely enough to reimplement
- [ ] the conformance suite is language-neutral
