# Agentic-first testing

> The concept document for `agt`. Written after a survey of what the phrase is
> being used to mean in 2026 — see [02-landscape.md](02-landscape.md) for the
> sources.

## The problem with the phrase

"Agentic testing" is currently used for three different things, and they are
routinely conflated:

| Reading | Who is the agent? | What is new |
| --- | --- | --- |
| **Agents *doing* the testing** | The agent is the QA engineer | Natural-language intent instead of scripts; self-healing selectors; autonomous exploration |
| **Testing *of* agents** | The agent is the system under test | Trajectory evaluation, LLM-as-judge, statistical gates over nondeterministic output |
| **Testing *for* agents** | The agent is the *consumer of the test suite* | The suite is an interface an autonomous repair loop reads and writes |

The first reading dominates the vendor literature and is mostly about browser
automation. The second dominates the research literature and is mostly about
evals. Both are real, and neither is a *testing framework* — the first is a
product category, the second is a measurement discipline.

The third reading is the one that changes what a test framework should be, and
it is the one `agt` takes.

## The thesis

**Most of a test suite's output is now read by a machine, and almost none of it
is designed for one.**

An agent working a repair loop does this, several times a minute:

1. run the suite,
2. parse human-formatted stderr back into structured meaning,
3. guess which failure is the cause and which are downstream,
4. change something,
5. re-run and hope the signal was stable.

Steps 2, 3 and 5 are pure loss. Step 2 is lossy string parsing of information
the framework had in structured form and threw away. Step 3 is a ranking
problem the framework is better positioned to answer than the agent is. Step 5
is a coin flip whenever the suite is nondeterministic.

Agentic-first testing means designing the framework so those three steps stop
existing. Concretely, three commitments:

### 1. Failures are evidence, not prose

Every non-passing outcome produces a structured record — expected, observed,
seed, stable fingerprint, minimized witness, the exact command to reproduce,
and a ranked list of cheapest-next-probes. One NDJSON record per line; the
human-readable renderer is built *over* that stream, not instead of it.

This is not a formatting preference. Three things follow from it that cannot be
retrofitted onto text output:

- **Fingerprints make the loop stateful.** Same fingerprint after a change means
  no progress. A *new* fingerprint means the change moved the failure rather
  than fixing it — the single most common way an agent convinces itself it is
  making progress while going in circles.
- **`Inconclusive` stops a whole class of damage.** A missing fixture is not a
  failing assertion, and an agent that cannot tell them apart will "fix" a
  broken harness by editing application code. Most frameworks collapse both into
  "red".
- **Probes are separable from fixes.** The framework knows how to get more
  information (shrink this input, re-run with this seed, bisect this trajectory).
  It does not know what the code should do. Emitting only the former is what
  keeps the framework honest.

The Rust ecosystem is already moving toward machine-readable *results* — the
libtest JSON RFC, `cargo nextest`'s `--message-format`. Those answer *which*
test failed. Evidence answers *how it failed and what to try next*, which is the
part the agent is currently reconstructing by hand.

### 2. Determinism is a resource the framework manages

A repair loop is a control system, and a flaky suite is noise injected directly
into its feedback path. For a human, flakiness is an annoyance to be worked
around. For an agent it is a corrupted reward signal, and the damage compounds:
the agent attributes a random pass to its last edit and locks in a change that
did nothing.

The deterministic-simulation-testing lineage — FoundationDB, and in Rust
`madsim` and `turmoil` — already solved the hard version of this for distributed
systems, by making an entire run a pure function of one seed. That requires
owning four sources of entropy: execution order, randomness, time, and I/O.

Agentic systems add a fifth: **model sampling**. And it needs different
treatment than the other four, because the naive approach — record HTTP traffic
with a VCR-style cassette and replay it — freezes too much. An HTTP cassette
that recorded a successful run keeps replaying that success after you break the
tool the model was calling. The recording captured the model's decision *and*
the tool's execution as one opaque blob.

The right seam is the **decision boundary**: record what the model chose (tool
name, arguments, final text), replay that, and dispatch the recorded arguments
to the *live* tool. The expensive nondeterministic part is frozen; the part you
are actually testing still runs.

### 3. The oracle is the asset, and it defends itself

This is the commitment that makes the other two safe, and it comes straight out
of the reinforcement-learning-for-code literature.

Execution-based test signal is the most scalable reward we have for coding
agents. It is also the easiest to game, and the failure is invisible: an agent
that can edit the suite can delete the assertion instead of satisfying it. From
the outside, "the bug is fixed" and "the test no longer checks" are the same
green check. Published mitigation work on filtering reward-hacked trajectories
moved hacked resolution rates from ~29% to ~0.6% — the effect size tells you how
much of the naive signal was hollow.

There is a deeper limit underneath, sometimes called the *verification horizon*:
tests only verify what is in their scope, real tasks carry requirements no test
states, and no amount of test-writing closes that gap. You cannot fix this. You
can refuse to hide it.

So `agt` makes the suite's *claim strength* a tracked, first-class quantity:

- Every case declares its strength: `Smoke < Example < Snapshot < Property < Proof`.
  A suite reports its strength profile, so "coverage went up" cannot conceal
  "every new test is a smoke test."
- Every diff touching the suite is classified per-case as **strengthened /
  neutral / weakened / silenced / removed**. A weakened assertion hides most
  comfortably inside a diff that also changes application code, where it reads
  as incidental cleanup — so weakening is surfaced as its own review category,
  never as an ordinary line change.
- Weakening is *not forbidden*. Deleting a wrong test is legitimate work.
  It is made impossible to do quietly.
- New tests are scored by mutation, not by coverage. A test that kills no
  mutants earns no credit, which is the cheapest available defence against the
  known LLM failure mode of generating property tests that assert something
  trivially true (`the function returns a value of the right type`).
- Judged verdicts (LLM-as-judge against a rubric) are recorded as a distinct
  expectation kind, so a reviewer can always see which gates rest on execution
  and which rest on an opinion.

## What this is not

- **Not an eval harness.** Evals measure a model over a benchmark. `agt` tests a
  program, with an agent as the primary reader. The two share machinery
  (trajectories, judges, statistical gates) and answer different questions.
- **Not autonomous QA.** No browser driving, no self-healing selectors, no
  natural-language test authoring. Those are a product category built on top of
  a framework like this one, not the framework.
- **Not a human-out-of-the-loop pitch.** The opposite: the point of tracking
  claim strength is to concentrate scarce human attention on the ~2% of a diff
  where an agent's incentives and the project's diverge. Full autonomy on the
  execution loop, mandatory human review on the oracle.

## Why Rust

Not incidental:

- The type system can carry claim strength and evidence structure at compile
  time, so a weakened oracle is a visible type-level change rather than a
  deleted line in a string.
- The DST prior art is here (`madsim`, `turmoil`), and the entropy-control work
  is the expensive part.
- The mutation-testing tooling is here (`cargo-mutants`) and already integrates
  with `cargo-nextest`.
- Cargo gives one predictable execution model, which is what makes a
  reproduce-command a reliable promise rather than an aspiration.

## Status

The concept is ahead of the code, deliberately. The crate currently fixes the
vocabulary and the wire format — `Evidence`, `Verdict`, `Seed`, `Oracle`,
`Strength`, `Movement`. The runner, the entropy control, the mutation scoring
and the ledger are not built. [04-open-questions.md](04-open-questions.md) is
honest about which of these are hard and which might not survive contact.
