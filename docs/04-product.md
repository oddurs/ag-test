# Product

What `agt` actually is to use. The reasoning is in
[01-concept.md](01-concept.md); the internals are in
[03-architecture.md](03-architecture.md).

## Who it is for

**Primary: a Rust team whose code is substantially written by agents**, where
someone has already noticed that the loop churns, that a green run stopped
meaning much, and that review has quietly become the bottleneck. They do not
want a new way to write tests. They want the tests they already have to produce
a better signal.

**Secondary: a Rust team with a flaky, slow suite and no agents at all.** Almost
everything here pays off for humans too — reproduce commands that work, minimised
witnesses, quarantine that keeps reporting, a review surface for weakened
assertions. This matters commercially: a tool that only works once you have
adopted agentic development has a much smaller market than one that is merely
*better* under agents.

**Not for**: teams wanting autonomous QA, browser test generation, or model
evaluation. Those are different products and
[01-concept.md](01-concept.md#what-this-is-not) says so plainly.

## The three surfaces

### 1. The library — what you write

Ordinary Rust tests. The framework's opinions arrive through attributes, not
through a new assertion vocabulary.

```rust
#[agt::test]
fn rejects_trailing_comma() {
    assert!(parse("[1,]").is_err());
}

#[agt::property(cases = 1000)]
fn round_trips(input: Json) {
    prop_assert_eq!(parse(&render(&input))?, input);
}

#[agt::test(isolation = "simulated", strength = "property")]
async fn retries_survive_partition(sim: Sim) { … }

#[agt::judged(rubric = "explanations/clarity.md", batch = 100, threshold = 0.9)]
fn explanation_is_clear(out: Explanation) -> Score { … }
```

The whole surface is: declare what a case proves, declare how much isolation it
needs. Everything else — seeds, fingerprints, shrinking, records — is derived.

### 2. The CLI — what a human sees

```console
$ agt run
  running 214 cases · seed 4f2a1c9e00b3d551

  ✗ parser::rejects_trailing_comma          property   3f1a9c2e
      expected  every accepted input round-trips to itself
      observed  accepted `[1,]`
      witness   [1,]                        (shrunk from 412 bytes)
      repro     agt run --seed 4f2a1c9e00b3d551 parser::rejects_trailing_comma

  ⊘ store::reads_after_write                inconclusive
      fixture `fixtures/store.db` not found — harness problem, not a failure

  ⚑ net::retries_backoff                    quarantined  (flaky since #204)

  213 passed · 1 failed · 1 inconclusive · 1 quarantined · 2.4s

  since last run:  1 appeared · 0 resolved · 0 persisted
                   ↳ new failure introduced by the current change
```

That last block is the part no other runner prints, and the part a human turns
out to want as much as an agent does.

```console
$ agt ledger --diff
  3 cases changed

  ✓ parser::round_trips              strengthened   example → property
  ~ parser::helpers                  neutral        refactor, claim unchanged
  ⚠ store::handles_conflict          WEAKENED       assert_eq → assert!(…is_ok())
      mutation score 12/14 → 3/14
      this is the only case covering conflict resolution

  1 change needs review before merge.
```

### 3. The protocol — what an agent sees

`agt serve` over stdio, MCP-shaped, so it drops into an existing tool loop as a
server rather than a subprocess to be screen-scraped. Full method list in
[03-architecture.md](03-architecture.md#the-agent-protocol).

The design goal is that a well-behaved loop never needs to parse a string:
`run` → `delta` → `evidence` → `shrink` → edit → `run`.

## The adoption ladder

Each rung stands alone and is worth something on its own. This is a hard product
constraint, not a nicety — a tool that only pays off at rung four is a tool
nobody reaches rung four with.

**Rung 1 — the reporter.** `agt run` over your existing `#[test]` functions. No
code changes. You get evidence records, fingerprints, the run delta,
`Inconclusive` classification, and NDJSON output for CI. Nothing else in your
project changes. *Value: the loop stops churning invisibly.*

**Rung 2 — seeds and shrinking.** Adopt `#[agt::property]` for the cases where
you already use `proptest`, and turn on the determinism meta-test. *Value:
reproduce commands become promises, and flakes get caught by the framework
rather than by the agent's confusion.*

**Rung 3 — the ledger.** Turn on strength declarations and movement
classification in CI. *Value: weakened assertions surface as their own review
category. This is the rung where an agent-heavy team feels the largest
difference, and the one requiring the most trust.*

**Rung 4 — simulation.** `isolation = "simulated"` for concurrency and
distributed logic. *Value: the classical deterministic-simulation payoff. Also
the most expensive rung, and correctly the last.*

Most teams should stop at 2 or 3. Saying so in the documentation is worth more
than pretending everyone needs 4.

## What day one looks like

```console
$ cargo add --dev agt
$ agt run                    # works immediately over existing #[test] fns
$ agt run --format ndjson > run.ndjson
$ agt serve                  # point your agent at it
```

No config file, no rewrite, no new assertion macros. If rung 1 requires a
migration, nobody gets to rung 2.

## What day thirty looks like

The suite has a strength profile that someone has looked at. Two or three cases
are quarantined with evidence attached instead of deleted. CI posts a movement
summary on every PR, and the team has developed an actual opinion about which
weakenings are fine. The agent's loop terminates faster because it stops
re-attempting resolved failures.

The measurable claim, and the one to be honest about if it fails to appear:
**fewer loop iterations per resolved failure, and fewer merged diffs that
weakened an oracle.** If neither number moves, the thesis is wrong.

## Positioning

| | Answers | `agt` adds |
| --- | --- | --- |
| `cargo test` | did it pass | identity, structure, intuition, integrity |
| `cargo nextest` | did it pass, fast, with JUnit | evidence over results; a protocol |
| `proptest` | does the invariant hold | seed discipline, records, strength tracking |
| `insta` | did the output change | acceptance as a reviewable movement |
| `cargo-mutants` | are the tests any good | the score as a live gate on new tests |
| eval frameworks | is the model any good | different question entirely |

`agt` is not competing with any of these. It composes four of them and adds the
layer none of them has: **a stateful, structured, tamper-evident interface for a
reader that is not a person.**

## Non-goals

Worth writing down, because each is a plausible-sounding direction that would
dilute the thesis:

- **No test generation.** The framework must never be the thing writing the
  tests it grades.
- **No fix suggestions.** Probes only. See
  [03-architecture.md](03-architecture.md#principles).
- **No language beyond Rust for now.** The wire format is portable on purpose and
  a JS or Python runner emitting it would be welcome — but from someone else,
  after the format has proven itself.
- **No hosted service in v1.** The ledger wants a trusted verifier, and "so we
  built a SaaS" is the obvious wrong turn. CI is a trusted verifier most teams
  already have.
- **No dashboard.** The stream is the product. Somebody else's dashboard can
  consume it.

## Naming

The crate is `agt` because the repository is `ag-test`. Neither name says
anything. A real name should be chosen once the API has stabilised, not before —
tracked as a milestone item rather than left to drift.
