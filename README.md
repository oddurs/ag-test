# agt — testing for a reader that isn't a person

Think about who reads a test failure.

Five years ago it was a human. A test fails, prints a red block with a diff, and
someone reads it, thinks *ah, the parser is accepting a trailing comma*, and
fixes it. Every design decision in every test framework — colour, a good diff, a
stack trace — assumes a pair of eyes and a brain on the other end.

That is not who is reading any more. Most of the time now a suite runs, fails,
and the thing that reads the failure is an agent in a loop: run tests, read
failure, change code, run again. Thirty times before a human looks at anything.

**The suite has become a machine-to-machine interface that is still formatted
like a human-to-human one.** `agt` is a Rust test framework that takes the new
reader seriously.

> **Status: concept + skeleton.** The vocabulary and wire format exist and are
> tested; the runner is not built. The thinking is in **[docs/](docs/)** — start
> with [the concept](docs/01-concept.md). The plan is
> [ROADMAP.md](ROADMAP.md).

## The new reader has three properties

**It has no memory.** A human remembers seeing this error two minutes ago. An
agent reads fresh text every run, so nothing tells it whether this failure is the
one it was working on, a new one it just introduced, or the same bug relocated by
its last edit. That produces the most expensive failure mode in agentic coding:
the loop churns for twenty iterations without noticing it is making no progress.

→ So every failure carries a **fingerprint**, and every run reports a **delta**:

```
since last run:  1 appeared · 0 resolved · 0 persisted
                 ↳ new failure introduced by the current change
```

**It has no intuition.** A human reads `No such file or directory` inside a test
and instantly knows the fixture is missing and the code is fine. An agent sees
red and starts editing the code the test names. The same gap makes flakiness
qualitatively worse — a person shrugs at a flake, and the shrug requires knowing
some failures mean nothing. An agent cannot shrug; it attributes the random pass
to whatever it changed last and keeps a change that did nothing.

→ So a failure says what a person would have inferred: `Inconclusive` as a
verdict distinct from `Fail`, the minimised witness, a reproduce command that
actually reproduces, and ranked next probes — and the run is deterministic,
because the reader cannot discount noise.

**It has an incentive to cheat.** The agent is graded by the suite and can edit
it. Deleting the assertion and fixing the bug produce an identical green
checkmark. Nobody had to think about this when the grader and the graded were
different people. Filtering reward-hacked trajectories has been measured to move
hacked resolution rates from ~29% to ~0.6% — most of the naive signal was hollow.

→ So the oracle defends itself: cases declare what they prove (`Smoke < Example <
Snapshot < Property < Proof`), every test diff is classified (**strengthened /
neutral / weakened / silenced / removed**) into its own review surface, and new
tests are scored by mutation rather than coverage — a case that kills no mutants
earns no credit.

Weakening a test is allowed. Deleting a wrong test is real work. It just cannot
happen quietly.

## What it looks like

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

  213 passed · 1 failed · 1 inconclusive · 2.4s
```

Same events as NDJSON on `--format ndjson`, or as method calls over `agt serve`.
The terminal output is a renderer over the stream, never a separate path, so
anything a human can see an agent can have.

## What this is not

Not an eval harness — evals measure a model against a benchmark; `agt` tests a
program whose reader happens to be one. Not autonomous QA — no browser driving,
no self-healing selectors. Not human-out-of-the-loop — tracking claim strength
exists to concentrate review attention, not remove it. And not a new way to write
tests: the assertions are the ones you already write.

## Try it

```bash
cargo test --all
```

Eight tests over the vocabulary layer: seed derivation stability, NDJSON round
trips, wire-format shape, movement classification.

## Developing

This repo develops itself with a parallel agent worktree harness — every task
gets an isolated worktree, branch, agent process and log:

```bash
./scripts/setup.sh                                     # hooks + git config
./bin/agent doctor
./bin/agent auto shrink-fix "Make shrinking deterministic under a fixed seed."
./bin/agent fleet tasks.example.txt -j 4
```

Roadmap and issues live in the repo as Markdown under
[`cairn/items`](cairn/items) — `cairn next` to see what is ready. Nothing merges
without passing `AGENT_VERIFY_CMD` and a reviewed pull request. Full reference:
[docs/workflow.md](docs/workflow.md).

## License

MIT. See [LICENSE](LICENSE).
