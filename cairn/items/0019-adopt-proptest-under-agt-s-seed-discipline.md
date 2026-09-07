---
id: 19
title: Adopt proptest under agt's seed discipline
type: feature
status: backlog
milestone: v0.2
created: 2026-09-07
updated: 2026-09-07
priority: p0
effort: l
area: macros
---

## Problem

Property-based testing is the recommended circuit-breaker against a model
writing tests that share the blind spots of the code it just wrote — invariants
are easier to state correctly than exhaustive input/output pairs. But the
generation engine is a solved problem and rewriting it would be a waste.

## Proposal

`#[agt::property(cases = N)]` wrapping `proptest`, supplying the seed discipline
and the evidence record around it rather than a new engine.

Known limitation to document rather than hide: LLM-generated properties are
frequently trivial (asserting the return has the right type) or outright wrong.
Property tests relocate the oracle problem, they do not solve it — which is what
mutation scoring in v0.3 is for.

## Acceptance criteria

- [ ] `#[agt::property]` runs proptest strategies with a derived child seed
- [ ] a failing property produces an evidence record with `Expectation::Property`
- [ ] the invariant statement, not the macro expansion, is what appears in `expected`
- [ ] existing `proptest!` blocks keep working untouched
