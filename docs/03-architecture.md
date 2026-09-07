# Architecture sketch

How the three commitments in [01-concept.md](01-concept.md) would be built.
Design, not documentation — only the vocabulary layer exists in code today.

## Crate layout

```
agt              facade: re-exports, prelude, the attribute macro
agt-core         Evidence, Verdict, Seed, Oracle, Strength, Movement   [exists]
agt-runner       execution, scheduling, entropy control, NDJSON stream
agt-replay       decision-boundary record/replay
agt-ledger       oracle hashing, Movement classification, mutation scoring
agt-cli          `agt` binary; also speaks the agent protocol on stdio
```

Only `agt-core` is written. It is deliberately dependency-light (serde only) so
the wire format can be consumed by anything without pulling in a runtime.

## The stream is the interface

One NDJSON record per line on stdout. Human output is a renderer over the same
stream — never a separate code path, so the two can never disagree.

```json
{"verdict":"fail","fingerprint":"3f1a…","case":"parser::rejects_trailing_comma",
 "expected":{"kind":"property","statement":"every accepted input round-trips to itself"},
 "observed":{"value":"accepted `[1,]`","trajectory":[]},
 "seed":"000000000000002a",
 "reproduce":"agt run --seed 000000000000002a parser::rejects_trailing_comma",
 "witness":"[1,]",
 "next_probes":["agt shrink parser::rejects_trailing_comma","agt bisect --since HEAD~20"]}
```

Four fields carry the weight:

**`fingerprint`** — content-addressed over (case, expectation, normalized
observation), deliberately *not* over line numbers or timing. Stable across
machines and across unrelated edits. This is what lets a loop detect "same
failure, no progress" and "different failure, you moved it" — the distinction an
agent otherwise has to infer from prose.

**`seed`** — see [`Seed::derive`](../src/seed.rs). Seeds split per scope rather
than being shared, so adding a case does not perturb the random choices of
unrelated cases. A seed that shifts under unrelated edits is not worth printing.

**`witness`** — the minimized input. Shrinking runs by default on failure, not
on request, because "smallest failing input" is worth more to a repair loop than
the original input ever was.

**`next_probes`** — ranked cheapest-first, and strictly information-gathering.
The framework knows how to learn more; it does not know what the code should do.
Keeping fixes out of this list is what stops the framework from becoming a
second, worse agent.

## Entropy control

Five sources, from [`seed::Entropy`](../src/seed.rs). The first four are the
standard DST set; the plan is to build on `madsim`/`turmoil` rather than
re-derive them, since owning an entire dependency tree's entropy is the
expensive part of that work.

| Source | Mechanism | Prior art |
| --- | --- | --- |
| Execution | single-threaded seeded scheduler | `madsim`, `turmoil` |
| Randomness | seeded RNG injection; `getrandom` override | `mad-turmoil` |
| Time | virtual clock, advances only on explicit await | `tokio::time::pause` |
| I/O | in-memory transports and service emulators | `turmoil` |
| **Sampling** | **decision-boundary replay** | new |

### Decision-boundary replay

The seam that matters, and the one classical DST never needed. Record what the
model *decided*; replay the decision; dispatch the recorded arguments to the
**live** tool.

```
record:   prompt ──▶ model ──▶ decision{tool, args} ──▶ tool ──▶ effects
                       │              │
                       └── recorded ──┘   (decision only)

replay:   prompt ──▶ [cassette] ──▶ decision{tool, args} ──▶ tool ──▶ effects
                                                              live
```

Recording the HTTP exchange instead would freeze the decision *and* the tool
execution into one blob, and the test would keep passing after the tool broke.
This is the single most common way agent test suites become decorative.

Cassettes are keyed by a hash of the prompt context, stored under
`tests/cassettes/`, and reviewed like any other fixture. A cassette miss in CI
is `Inconclusive`, never `Fail` — a missing recording is a harness problem, and
an agent must not be handed a signal that invites it to edit application code in
response.

## The oracle ledger

The anti-hacking layer. It exists because the agent that runs the suite can also
edit it.

For every case the ledger stores: stable name, declared [`Strength`], a hash
over the assertion's normalized AST, and the mutation score from the last run.

On every diff touching tests it emits a [`Movement`] per case:

| Movement | Trigger | Gate |
| --- | --- | --- |
| `Strengthened` | strength up, or mutation score up | auto |
| `Neutral` | assertion hash stable under refactor | auto |
| `Weakened` | tolerance widened, timeout grown, cases narrowed | **review** |
| `Silenced` | `#[ignore]`, skip, or quarantine added | **review** |
| `Removed` | case deleted | **review** |

Rendered as its own section, separate from the code diff. The reason is specific:
a weakened assertion is hardest to spot inside a diff that also changes
application code, where the reviewer's attention is on the feature and the test
edit reads as cleanup. Splitting the surfaces is the whole mechanism.

Weakening is permitted. Deleting a wrong test is real work. It just cannot
happen quietly.

**Mutation scoring** gates new tests: a newly added case that kills no mutants
earns no credit toward the gate, and is reported as `Smoke` regardless of what
it declares. This is the cheapest defence against the documented LLM failure
mode of emitting properties that are trivially true. Built on `cargo-mutants`,
scoped to the diff so it stays affordable in CI.

## The agent protocol

An agent should not shell out and regex the output of its own test framework.
`agt serve` speaks line-delimited JSON-RPC over stdio, MCP-shaped:

| Method | Returns |
| --- | --- |
| `list_cases` | names, strengths, mutation scores, quarantine state |
| `run` | a stream of `Verdict` records |
| `evidence` | the full record for one fingerprint |
| `shrink` | a smaller witness for a fingerprint |
| `replay` | re-run one fingerprint at its recorded seed |
| `propose_case` | submit a new case; returns its mutation score, unmerged |
| `ledger_diff` | `Movement` per case for the working tree |

`propose_case` is the important one: an agent can find out whether a test it
just wrote is worth anything *before* committing it, and the framework can
decline to count a worthless one. That closes the loop the reward-hacking
literature says is otherwise open.

## Statistical gating

Where a case is genuinely nondeterministic — a judged rubric, a sampled model —
a single run proves nothing. Those cases declare a batch size and a threshold
rather than an equality, and the evidence record carries the observed
distribution, not a single sample. The practitioner consensus figure for
CI-blocking decisions is N ≥ 100.

`Expectation::Judged` is a separate variant precisely so a reviewer can ask
which gates rest on execution and which on an opinion, and so the two can be
counted separately in the strength profile.

## Relationship to existing Rust tooling

`agt` is not a replacement for `cargo test`, and interoperates rather than
competes:

- **libtest / nextest** — `agt` emits `libtest-json` alongside its own stream, so
  existing CI keeps working. RFC 3558 is unfinished and nextest's own docs call
  its JSON "not full-fidelity"; when a first-class NDJSON run format lands, `agt`
  should consume it rather than duplicate it.
- **proptest / quickcheck** — the generation and shrinking engine. `agt` supplies
  the seed discipline and the evidence record around them.
- **insta** — `Expectation::Snapshot`. Snapshot acceptance is exactly the review
  surface the ledger is designed to police: `cargo insta accept` under an agent
  is a `Movement` event, not a formality.
- **cargo-mutants** — the mutation scorer behind `propose_case`.
