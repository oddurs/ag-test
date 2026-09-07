# Architecture

How the three consequences in [01-concept.md](01-concept.md) get built. This is
design, not documentation — only the vocabulary layer exists in code. What it
feels like to use is [04-product.md](04-product.md); what might not work is
[05-open-questions.md](05-open-questions.md).

## Principles

Four, and they decide most of the arguments further down.

**The stream is the interface.** One newline-delimited JSON record per event on
stdout. The human-readable terminal output is a *renderer over that stream*,
never a parallel code path — so the two can never disagree, and anything the
human sees the agent can also have. This is the inverse of every existing
framework, where machine output is a bolt-on that loses fidelity.

**Each commitment is independently adoptable.** Evidence records without
determinism, determinism without the ledger, the ledger without either. A
framework that only pays off once all three are in place is a framework nobody
finishes adopting. This forces loose coupling everywhere and is the main
constraint on the crate graph.

**The core carries no runtime.** `agt-core` depends on `serde` and nothing else,
so the wire format can be consumed by a CI script, an editor plugin, or another
language's tooling without pulling in an async runtime.

**Nothing invents a fix.** The framework produces evidence and probes. It never
proposes a code change. The moment it does, it is a second, worse agent, and its
output stops being trustworthy input to the first one.

## Shape

```
        your tests
            │
            ▼
   ┌──────────────────┐        ┌───────────────┐
   │   agt-runner     │◀──────▶│   agt-replay  │  decision cassettes
   │  seeded, isolated│        └───────────────┘
   └────────┬─────────┘
            │ NDJSON events
            ├────────────────────────▶  agt-cli render   → terminal (human)
            ├────────────────────────▶  agt serve        → agent protocol
            └────────────────────────▶  *.ndjson         → CI artifact
                     │
                     ▼
            ┌──────────────────┐
            │    agt-ledger    │  strength, movement, mutation scores
            └──────────────────┘
                     │
                     ▼
              review surface (PR)
```

## Crates

| Crate | Responsibility | State |
| --- | --- | --- |
| `agt-core` | `Evidence`, `Verdict`, `Seed`, `Oracle`, `Strength`, `Movement`; the wire format | **exists** |
| `agt-macros` | `#[agt::test]`, `#[agt::property]`, `#[agt::simulation]` | planned |
| `agt-runner` | discovery, scheduling, isolation, entropy control, event stream | planned |
| `agt-replay` | decision-boundary record/replay, cassette storage | planned |
| `agt-ledger` | oracle hashing, movement classification, mutation scoring | planned |
| `agt-cli` | the `agt` binary: `run`, `replay`, `shrink`, `ledger`, `serve` | planned |
| `agt` | facade + prelude; the only crate a user names | partial |

Deliberately not one crate. `agt-core` is what a third party depends on to read
the stream; making them compile a runner to do that would be a tax on the
ecosystem the format is trying to create.

## The evidence record

The load-bearing artifact. Current shape in [`src/evidence.rs`](../src/evidence.rs).

```json
{"verdict":"fail",
 "fingerprint":"3f1a9c2e7d5b8041",
 "case":"parser::rejects_trailing_comma",
 "expected":{"kind":"property","statement":"every accepted input round-trips to itself"},
 "observed":{"value":"accepted `[1,]`","trajectory":[]},
 "seed":"000000000000002a",
 "reproduce":"agt run --seed 000000000000002a parser::rejects_trailing_comma",
 "witness":"[1,]",
 "next_probes":["agt shrink parser::rejects_trailing_comma",
                "agt bisect --since HEAD~20 --fingerprint 3f1a9c2e7d5b8041"]}
```

### Fingerprints

The field that gives a memoryless reader a memory, so its stability is the
single most load-bearing assumption in the design.

Computed as a hash over a **normalised failure identity**:

```
fingerprint = h( case_path ‖ expectation_kind ‖ normalise(observed) ‖ failure_site )
```

What is deliberately *excluded*: line numbers, timestamps, durations, thread
ids, absolute paths, allocation addresses, iteration counts. Including any of
them makes an unrelated edit look like a new bug.

Normalisation is a pipeline of rewriters applied to the observed value before
hashing — numeric literals in addresses, temp paths, UUIDs, and durations
collapse to placeholders. The default set covers the common cases; a project can
add its own, and *that* is the part likely to need per-project configuration,
which [05-open-questions.md](05-open-questions.md) treats as an unresolved risk
rather than a settled design.

Two failure directions, both bad, and the tuning target sits between them:

- **too stable** → two different bugs collide under one identity, and the agent
  believes it fixed something it did not;
- **too unstable** → every edit reads as a new failure, and the loop loses the
  history that made it stateful.

The runner emits a **run delta** alongside the records — fingerprints resolved,
persisted, and appeared — which is the loop's actual progress signal and the
thing `agt serve` exposes first.

### Verdict variants carry the intuition

`Pass`, `Fail`, `Inconclusive`, `Quarantined`. The last two exist so that "red"
is not one undifferentiated bucket:

- **`Inconclusive`** — the case could not be judged. Missing fixture, absent
  dependency, cassette miss, harness error. An agent must treat this as *fix the
  harness*, never *edit the code under test*. This is the single highest-value
  variant in the enum and the one most frameworks lack.
- **`Quarantined`** — known-failing, still running, still reported, not blocking
  the gate. Quarantine's precise definition matters: the case keeps executing
  and reporting, only its ability to block a merge changes. Folding it into
  "skip" loses exactly the information that makes it safe.

### Probes, not fixes

`next_probes` are ranked cheapest-first and are strictly information-gathering:
`shrink`, `replay`, `bisect`, `isolate` (re-run alone to test for cross-test
interference), `amplify` (re-run N times under varied seeds to test for
flakiness). Each is a command the agent can execute verbatim.

Never `try changing X`. The framework has no opinion about the code, and an
agent that receives one from its test runner has been handed a confident guess
dressed as data.

## The runner

### Execution model

Custom harness (`harness = false`) rather than a libtest shim, because the
information the record needs — the claim, the generator, the seed scope — has to
be captured at assertion time and libtest has nowhere to put it.

Three isolation levels, per case, declared by attribute:

| Level | Mechanism | Cost | For |
| --- | --- | --- | --- |
| `inline` | same process, same thread | ~µs | pure unit tests |
| `isolated` | process per case | ~ms | anything touching globals, env, or `install`-style state |
| `simulated` | seeded single-threaded runtime, virtual clock, mocked I/O | ~ms + build cost | concurrency, distributed logic, timeouts |

Default is `inline`, promoting to `isolated` automatically for any case that
fails and then passes on re-run — the runner's own flake detector, which files
the case as `Quarantined` with the evidence attached rather than letting it churn
in the loop.

### Scheduling

Cases are ordered by *expected information gain per second*, not by name:
previously failing cases first, then fast ones, then everything else. In a repair
loop the first ten seconds of a run are worth more than the rest, because the
agent is usually waiting to learn whether one specific thing changed. `--fast-fail`
exits after the first fingerprint that differs from the last run's set.

### Seeds

One root seed per run, printed and recorded. Every scope derives its own child
seed by label rather than drawing from a shared stream — see
[`Seed::derive`](../src/seed.rs) — so adding a case does not shift the random
choices made by unrelated cases. A seed that shifts under unrelated edits is not
worth printing.

## Entropy control

Five sources. The first four are the standard deterministic-simulation set and
the plan is to build on `madsim`/`turmoil` rather than re-derive them, because
owning an entire dependency tree's entropy is the expensive part of that work and
it has been done.

| Source | Mechanism | Scope |
| --- | --- | --- |
| Execution | seeded single-threaded scheduler | `simulated` only |
| Randomness | seeded RNG injection; `getrandom` override; fixed `HashMap` seeds | all levels |
| Time | virtual clock; advances only on explicit await | `simulated` only |
| I/O | in-memory transports, service emulators | `simulated` only |
| **Sampling** | **decision-boundary replay** | all levels |

Everything not on this list is a leak, and leaks are what make a suite flaky —
which for this reader is not an annoyance but a corrupted reward. The runner
ships a **meta-test**: run the same seed twice, compare the event streams
byte-for-byte, fail the build if they differ. Determinism that is not
continuously verified decays within weeks.

### Decision-boundary replay

The seam classical simulation testing never needed, and the one place where the
obvious implementation is wrong.

```
record:   context ──▶ model ──▶ decision{tool, args} ──▶ tool ──▶ effects
                        │             │
                        └── recorded ─┘        ← the decision only

replay:   context ──▶ [cassette] ──▶ decision{tool, args} ──▶ tool ──▶ effects
                                                              live
```

Recording the HTTP exchange instead freezes the model's decision *and* the tool's
execution into one opaque blob, so the test keeps passing after you break the
tool the model was calling. That is the most common way an agent test suite
becomes decorative while still being green.

- **Cassettes** live in `tests/cassettes/`, one file per case, NDJSON, reviewed
  like any other fixture.
- **Keys** are a hash of the normalised request context, so an unrelated prompt
  edit invalidates only the cases it touches.
- **A miss is `Inconclusive`, never `Fail`.** A missing recording is a harness
  problem, and reporting it as a failure invites the agent to edit application
  code in response.
- **Divergence policy** is per-case: `strict` (any live-tool mismatch is
  `Inconclusive`), or `until-divergence` (replay to first mismatch, then fall
  back to live sampling and re-record). `strict` is the default because a
  silently-degraded determinism guarantee is worse than a loud one.

## The oracle ledger

The anti-cheating layer, and the piece that has no analogue in existing tools.

### Contents

`.agt/ledger.json`, one entry per case:

```json
{"case":"parser::rejects_trailing_comma",
 "strength":"property",
 "claim_hash":"9a1c…",
 "mutants_killed":14,
 "mutants_total":17,
 "quarantined":false,
 "last_verified":"2026-09-07T16:33:58Z"}
```

`claim_hash` is over the assertion's **normalised AST** — not its source text —
so renaming a local or reformatting is `Neutral`, while widening a comparison is
not.

### Movement classification

On every diff touching tests, per case:

| Movement | Trigger | Gate |
| --- | --- | --- |
| `Strengthened` | strength up, or mutation score up | auto |
| `Neutral` | claim hash stable under refactor | auto |
| `Weakened` | tolerance widened, timeout grown, generator narrowed, assertion loosened | **review** |
| `Silenced` | `#[ignore]`, skip, or quarantine added | **review** |
| `Removed` | case deleted | **review** |
| `Undetermined` | classifier cannot decide | **review** |

Rendered as its own section of the PR, separate from the code diff, because a
weakened assertion is hardest to see inside a diff that also changes application
code. Splitting the surfaces is the entire mechanism.

`Undetermined` exists because the classification is not always decidable —
replacing three examples with one property is usually a strengthening but not if
the property is trivial; narrowing a generator may be a weakening or a
correction. Defaulting to review is the safe direction; the cost is review
fatigue, which is what kills mandatory-review mechanisms.

### Mutation scoring

The empirical tiebreaker, and the gate on new tests. Built on `cargo-mutants`,
scoped to the diff so it stays affordable:

1. generate mutants in the code the changed tests cover;
2. run only those tests against them;
3. a new case that kills zero mutants earns no credit and is recorded as `Smoke`
   regardless of what it declares.

This is the cheapest defence against a model emitting a property that is
trivially true. It also gives `Undetermined` a way to resolve itself: did the
change kill more mutants, or fewer?

### Trust boundary

The ledger is a file in a repository the agent can write to. So verification
happens where the agent has no credentials:

- the ledger hash is committed and re-derived in CI;
- mutation scores are recomputed in CI, never trusted from the working tree;
- a ledger change without a corresponding test change fails the build.

This does not fully solve it — see
[05-open-questions.md](05-open-questions.md) — but it moves the referee out of
the sandbox, which is the part that matters.

## The agent protocol

An agent should not shell out and regex the output of its own test framework.
`agt serve` speaks line-delimited JSON-RPC over stdio, MCP-shaped, so it drops
into an existing tool loop as a server rather than a subprocess.

| Method | Returns |
| --- | --- |
| `list_cases` | names, strengths, mutation scores, quarantine state |
| `run` | a stream of verdict records; accepts a filter and a seed |
| `delta` | fingerprints resolved / persisted / appeared since a prior run |
| `evidence` | the full record for one fingerprint |
| `shrink` | a smaller witness for a fingerprint |
| `replay` | re-run one fingerprint at its recorded seed |
| `isolate` | re-run one case alone, to test for cross-test interference |
| `amplify` | re-run under N seeds, to test for flakiness |
| `propose_case` | submit a candidate test; returns its mutation score, unmerged |
| `ledger_diff` | movement per case for the working tree |

Two are doing unusual work.

**`delta`** is the memory prosthetic. It is the first call a well-behaved loop
makes after a run, and the answer — *nothing resolved, one appeared* — is the
thing an agent cannot compute for itself from a text diff.

**`propose_case`** closes the loop the reward-hacking literature says is
otherwise open: the agent can find out whether a test it just wrote is worth
anything *before* committing it, and the framework can decline to count a
worthless one. Structurally, it converts "write a test that passes" into "write a
test that kills mutants", which is a different and much harder thing to game.

## Statistical gating

Where a case is genuinely nondeterministic — a judged rubric, a sampled model —
a single run proves nothing. Those cases declare a batch size and a threshold
rather than an equality, and the evidence record carries the observed
distribution rather than one sample. N ≥ 100 is the figure the practitioner
literature converges on for merge-blocking decisions.

`Expectation::Judged` is a separate variant so a reviewer can always ask which
gates rest on execution and which on an opinion, and so the strength profile can
count them separately. A suite whose gates are 40% judged is a different object
from one at 2%, and today nothing makes that visible.

## Interop

`agt` is not a replacement for `cargo test`, and the adoption path in
[04-product.md](04-product.md) depends on that being true.

- **libtest / nextest** — emits `libtest-json` alongside its own stream, so
  existing CI keeps working unchanged. RFC 3558 is unfinished and nextest's own
  docs call its JSON "not full-fidelity"; when a first-class NDJSON run format
  lands upstream, `agt` should consume it rather than duplicate it.
- **proptest / quickcheck** — the generation and shrinking engine. `agt` supplies
  the seed discipline and the record around them, and does not write another one.
- **insta** — `Expectation::Snapshot`. Snapshot acceptance is precisely the
  surface the ledger polices: `cargo insta accept` under an agent is a movement
  event, not a formality.
- **cargo-mutants** — the scorer behind `propose_case` and the ledger.
- **madsim / turmoil** — the `simulated` isolation level.

## Performance budget

The framework is in a loop that runs many times a minute, so overhead is a
correctness concern, not a nicety. Targets:

- evidence construction on the pass path: **zero** — records are built only for
  non-pass verdicts;
- per-case overhead at `inline`: < 50 µs;
- fingerprinting a failure: < 1 ms;
- ledger classification on a 10-case diff: < 2 s;
- diff-scoped mutation run: < 60 s, which is the number that decides whether
  `propose_case` is usable interactively or only in CI.
