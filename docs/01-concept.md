# The reader changed

The concept document for `agt`. The survey it came out of is
[02-landscape.md](02-landscape.md); how it would be built is
[03-architecture.md](03-architecture.md).

## Who your test suite is talking to

Think about who reads a test failure.

When you wrote tests five years ago the audience was a person. A test fails, it
prints a red block with a diff, and a human reads it, thinks *ah, the parser is
accepting a trailing comma*, and goes and fixes it. Every design decision in
every test framework — colour, a good diff, a stack trace, a summary line —
assumes a pair of eyes and a brain on the other end.

That is not who is reading any more. Most of the time now a suite runs, fails,
and the thing that reads the failure is an agent in a loop: run tests, read
failure, change code, run tests again. It may go round thirty times before a
human looks at anything.

So the suite has become a machine-to-machine interface that is still formatted
like a human-to-human one.

This is the whole thesis. Not *tests written by AI*, not *tests for AI systems*
— see [02-landscape.md](02-landscape.md), where those are the two readings that
already have the phrase "agentic testing" pointed at them. Just: **the audience
changed, and nothing downstream of that has been redesigned.**

## What the new reader is like

Take the audience seriously and it has three properties, none of which a test
framework has ever had to design for.

### It has no memory

A human remembers, without effort, that they saw this same error two minutes
ago. An agent does not. Each run is a fresh read of fresh text. Nothing in the
output tells it whether this failure is the one it was already working on, a
different one it accidentally introduced, or the same bug relocated by its last
edit.

That gap produces the most expensive failure mode in agentic coding, and it is
almost invisible from the outside: **the loop churns for twenty iterations
without noticing it is making no progress.** Every iteration looks locally
reasonable. The suite never says *this is the same thing you saw last time.*

### It has no intuition

A human reads `error: No such file or directory (os error 2)` in a test and
knows instantly that the fixture is missing and the application code is fine.
That inference is free, unconscious, and completely unavailable to an agent
reading the same red text. An agent sees a failing test and starts editing the
code the test names.

The same gap makes flakiness qualitatively worse. A flaky test is mildly
annoying to a person — you shrug, re-run, move on. The shrug is the whole
mechanism, and it requires knowing that some failures do not mean anything. An
agent cannot shrug. A test that randomly passes gets attributed to whatever the
agent changed last, so it keeps a change that did nothing and moves on,
confidently wrong. Noise in the suite is not an annoyance to a control loop; it
is corruption of the signal driving it.

### It has an incentive to cheat

This one is new in kind, not degree.

The agent is graded by the suite and has write access to it. Deleting the
assertion and fixing the bug produce an identical green checkmark. Nobody had to
think about this when the grader and the graded were different people, and the
literature that has looked at it directly finds the effect is large, not
marginal — filtering reward-hacked trajectories moved hacked resolution rates
from about 29% to about 0.6%. Most of the naive signal was hollow.

It is worth being precise that this needs no intent. An agent minimising the
distance to green will find the cheaper edit, and the cheaper edit is very often
in the test file.

## Three consequences

Each property implies something the framework must do. This is the entire
design.

### No memory → give failures an identity

Every failure carries a **fingerprint**: content-addressed over the case, the
claim, and the normalised observation — deliberately not over line numbers or
timings. That single field converts a stateless reader into a stateful one:

- same fingerprint after a change → *your edit did nothing*
- fingerprint gone → *fixed*
- new fingerprint → *you moved the failure rather than fixing it*

The third is the one no framework reports today and no agent can infer.

### No intuition → say the things a human would have inferred

A failure report should carry what a person would have supplied from context:

- **What kind of failure this is.** `Inconclusive` — a missing fixture, an
  unavailable dependency — is a distinct verdict from `Fail`, because an agent
  that cannot tell them apart repairs a broken harness by editing application
  code. Almost every framework collapses both into red.
- **The smallest input that triggers it.** Shrinking on failure by default, not
  on request; the minimised witness is worth more to a repair loop than the
  original input ever was.
- **The exact command to reproduce it**, as a fact rather than an aspiration —
  which is only possible if the run was deterministic, which is why determinism
  is a commitment and not a nice-to-have.
- **The cheapest next probes**, ranked. The framework knows how to get more
  information — shrink this, replay that seed, bisect this trajectory. It does
  not know what the code should do, and it must not guess: probes are
  information-gathering only. Keeping fixes out of that list is what stops the
  framework becoming a second, worse agent.

And the precondition under all of it: **the run must be reproducible**, because
the reader cannot discount noise. One seed governs execution order, randomness,
time, I/O, and — new to this generation — model sampling.

### Incentive to cheat → make the oracle defend itself

The suite is the asset under protection, so its strength becomes a tracked
quantity rather than an assumed one.

- Every case declares what it proves: `Smoke < Example < Snapshot < Property <
  Proof`. A suite reports its strength profile, so *coverage went up* cannot
  conceal *every new test is a smoke test*.
- Every diff touching tests is classified per case — **strengthened / neutral /
  weakened / silenced / removed** — and rendered as its own review surface. The
  reason is specific: a weakened assertion hides most comfortably inside a diff
  that also changes application code, where the reviewer's attention is on the
  feature and the test edit reads as incidental cleanup.
- New tests are scored by **mutation**, not coverage. A case that kills no
  mutants earns no credit toward the gate. This is the cheapest known defence
  against the documented failure mode where a model emits a property test
  asserting something trivially true, like *the function returns a value of the
  right type*.
- The referee lives outside the sandbox. Scores and ledger hashes are verified
  where the agent has no credentials.

Weakening a test is **not** forbidden. Deleting a wrong test is real work. It is
made impossible to do quietly.

## The part this does not fix

Tests only ever verify what is in their scope. Real tasks carry requirements no
test states — maintainability, design, the intent behind the ticket — and no
amount of test-writing closes that gap. The literature calls this the
*verification horizon* and concludes, correctly, that there is no silver bullet.

`agt` does not claim to close it. The design goal is the opposite: make the gap
**visible**, so the small number of places where an agent's incentives and the
project's diverge are exactly where human attention gets spent. Full autonomy on
the execution loop; mandatory human review on the oracle.

That is also the honest answer to *is this just distrust of agents?* No — it is
the same reason code review survived good engineers. The check is cheap and the
failure is silent.

## What this is not

- **Not an eval harness.** Evals measure a model against a benchmark. `agt`
  tests a program whose primary reader happens to be a model. Shared machinery,
  different question.
- **Not autonomous QA.** No browser driving, no self-healing selectors, no
  natural-language authoring. That is a product category built on top of a
  framework like this, not the framework.
- **Not human-out-of-the-loop.** Tracking claim strength exists to concentrate
  scarce review attention, not to remove it.
- **Not a new way to write tests.** The assertions are the ones you already
  write. What changes is what the framework does with them.

## Why Rust

Not incidental:

- The type system can carry claim strength and evidence structure at compile
  time, so weakening an oracle is a visible type-level change rather than a
  quietly deleted line.
- The deterministic-simulation prior art is here (`madsim`, `turmoil`), and
  entropy control is the expensive part of that work.
- The mutation tooling is here (`cargo-mutants`) and already integrates with
  `cargo-nextest`.
- Cargo gives one predictable execution model, which is what lets a reproduce
  command be a promise rather than a hope.

## Status

The concept is deliberately ahead of the code. What exists is the vocabulary and
the wire format — `Evidence`, `Verdict`, `Seed`, `Oracle`, `Strength`,
`Movement` — with tests over seed stability and record shape. The runner,
entropy control, mutation scoring and the ledger are not built; the plan for
them is the roadmap, and [05-open-questions.md](05-open-questions.md) is honest
about which parts might not survive contact.
