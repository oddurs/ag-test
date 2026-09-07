# agt — agentic-first testing

A Rust testing framework built on the premise that **most of a test suite's
output is now read by a machine, and almost none of it is designed for one.**

Conventional frameworks answer one question — *did it pass?* — and when the
answer is no they print a diff and hope the reader knows what to do next. `agt`
assumes the reader is an autonomous agent working a repair loop, and that the
same agent is proposing new tests.

> **Status: concept + skeleton.** The vocabulary and wire format exist and are
> tested. The runner is not built. The thinking is in
> **[docs/](docs/)** — start with [the concept](docs/01-concept.md), then
> [the research it came from](docs/02-landscape.md).

## Three commitments

**1. Failures are evidence, not prose.** Every non-passing outcome emits a
structured record — expected, observed, seed, stable fingerprint, minimized
witness, exact reproduce command, ranked next probes. One NDJSON line each; the
human renderer is built over that stream, not instead of it.

```json
{"verdict":"fail","fingerprint":"3f1a…","case":"parser::rejects_trailing_comma",
 "expected":{"kind":"property","statement":"every accepted input round-trips to itself"},
 "observed":{"value":"accepted `[1,]`"},
 "seed":"000000000000002a",
 "reproduce":"agt run --seed 000000000000002a parser::rejects_trailing_comma",
 "witness":"[1,]"}
```

Fingerprints make the loop stateful: the same one after a change means no
progress; a new one means the change *moved* the failure rather than fixing it.

**2. Determinism is a resource the framework manages.** A flaky suite is noise
injected into an agent's feedback path — it will attribute a random pass to its
last edit and lock in a change that did nothing. Runs are a pure function of one
seed across five entropy sources: execution, randomness, time, I/O, and model
sampling. Sampling is handled at the *decision boundary* — record what the model
chose, replay that, dispatch to the **live** tool — because an HTTP-level
cassette keeps replaying success after you break the tool it was calling.

**3. The oracle defends itself.** When the agent being graded can also edit the
suite, "the bug is fixed" and "the test no longer checks" are the same green
check. So claim strength is tracked (`Smoke < Example < Snapshot < Property <
Proof`), every test diff is classified (**strengthened / neutral / weakened /
silenced / removed**), weakening is surfaced as its own review category, and new
tests are scored by mutation rather than coverage — a test that kills no mutants
earns no credit.

Weakening a test is allowed. Deleting a wrong test is real work. It just cannot
happen quietly.

## What this is not

Not an eval harness — evals measure a model over a benchmark; `agt` tests a
program with an agent as the primary reader. Not autonomous QA — no browser
driving, no self-healing selectors. Not a human-out-of-the-loop pitch — tracking
claim strength exists to concentrate scarce human attention on the small part of
a diff where an agent's incentives and the project's diverge.

## Try it

```bash
cargo test --all
```

Eight tests over the vocabulary layer: seed derivation stability, NDJSON round
trips, wire-format shape, movement classification.

## Developing

This repo develops itself with a parallel agent worktree harness. Every task
gets an isolated worktree, branch, agent process and log:

```bash
./scripts/setup.sh                                     # hooks + git config
./bin/agent doctor                                     # check the environment
./bin/agent auto shrink-fix "Make shrinking deterministic under a fixed seed."
./bin/agent fleet tasks.example.txt -j 4               # many tasks in parallel
```

Nothing merges without passing `AGENT_VERIFY_CMD` (`cargo fmt --check && cargo
clippy -D warnings && cargo test --all`) and a reviewed pull request. Full
reference: [docs/workflow.md](docs/workflow.md).

## License

MIT. See [LICENSE](LICENSE).
