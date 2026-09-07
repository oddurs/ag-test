# Landscape: what "agentic testing" currently means

Research notes behind [01-concept.md](01-concept.md), which argues for the third
of the three readings below. Surveyed September 2026.
Every claim here links to its source; where sources disagree, the disagreement
is noted rather than resolved.

## 1. Agents doing the testing (the vendor reading)

The dominant commercial usage. The shift described is *from scripting every step
to describing an outcome*: you hand an agent a goal in plain English and it
plans, drives the browser, observes, corrects itself, and reports whether the
goal was met. The loop — perceive, plan, act, reflect — is what makes it
"agentic" rather than a one-shot prompt.

Recurring capability list: self-healing locators, autonomous exploration that
proposes scenarios, natural-language authoring, and failure triage that
classifies a red run as *real bug / environment issue / test defect*.

The most useful piece in this category is also the most self-critical. It argues
the naive implementation — "a single model with a browser, a goal, and a loop" —
fails in four specific ways:

- **silent hallucination** — a single model reports a pass when it is uncertain;
- **cost** — no caching means a fresh model call per step per run;
- **brittleness** — a UI change is unrecoverable rather than re-resolved;
- **false autonomy** — removing the human removes the checkpoint that catches
  the first failure mode.

Its proposed architecture is worth stealing from even for a non-browser
framework: **intent capture** (steps as intent, not selectors), **cache-first
execution** (a resolved step replays deterministically and only falls back to a
model when it breaks), **auto-healing that re-caches**, and **multi-model
consensus assertions** with an arbiter for disagreement.

Cache-first execution is the same idea as record/replay (§4), arrived at from a
cost argument instead of a determinism argument. That convergence is the
strongest signal in this whole survey.

- [What agentic testing actually means in 2026 — Bug0](https://bug0.com/blog/what-agentic-testing-actually-means-2026)
- [Agentic QA Architecture: Reasoning Loops, Self-Healing DOM — TestQuality](https://testquality.com/agentic-qa-architecture-autonomous-testing-2026/)
- [Agentic AI Testing: Complete Guide — QASkills](https://qaskills.sh/blog/agentic-ai-testing-guide-2026)

## 2. Testing of agents (the research reading)

Here the agent is the system under test, and the central problem is that
assertions do not survive nondeterminism.

**Trajectory evaluation.** Score the step-by-step path, not just the final
answer: duplicate tool calls, irrelevant actions, unsafe intermediate steps —
all invisible to output-only checks that a correct final answer conceals.

**LLM-as-judge**, and its escalation **Agent-as-a-judge**, where the judge is
itself a multi-step agent evaluating the whole reasoning trace rather than the
final output. Judges give continuous scores with explanations instead of binary
pass/fail — useful for quality dimensions, and a liability when a merge gate
rests on one.

**The three-tier metric stack** that recurs across practitioner writing:
deterministic checks (schema, regex) → reference-based scoring (semantic
similarity) → judge rubrics for abstract quality. Deterministic first, judges
last, and only for what the earlier tiers cannot express.

**Statistical gating.** Because a single run proves nothing about a
nondeterministic system, CI decisions are taken over batches (N ≥ 100 is the
figure that appears), against thresholds rather than equality.

**The counter-movement**: judge-free frameworks that score against grounded,
time-bounded evidence and evaluate both the final answer and the recorded
trajectory, on the argument that judges are themselves an unvalidated oracle.

- [LLM Evaluation Framework: Trajectories vs. Outputs — LangChain](https://www.langchain.com/resources/llm-evaluation-framework)
- [Evaluating AI Agents at the Run, Trace, and Thread Level — LangChain](https://www.langchain.com/resources/agent-evals)
- [When AIs Judge AIs: Agent-as-a-Judge Evaluation (arXiv 2508.02994)](https://arxiv.org/pdf/2508.02994)
- [LLM-as-a-Judge in 2026 — DeepEval](https://deepeval.com/blog/llm-as-a-judge)
- [GroundEval: A Deterministic Replacement for LLM-as-Judge (arXiv 2606.22737)](https://arxiv.org/pdf/2606.22737)
- [A Practical Framework for Testing Non-Deterministic AI Agents](https://dev.to/ella-wilson/a-practical-framework-for-testing-non-deterministic-ai-agents-4hk0)

## 3. Harness engineering (the connective tissue)

A distinct practice has crystallised around the *harness* — the layer between
model and world — and its testing literature contains the single most directly
applicable idea in this survey:

> When tests fail, the harness feeds the error output back to the agent as fresh
> context. The model returns a structured tool call; the harness validates the
> schema, checks permissions, executes, and injects the result back. Whether
> it's a successful API response, a permission denial, or a timeout, **the agent
> always receives a structured observation.**

Also from this literature: hash every tool call and detect repetition, forcing a
strategy change rather than allowing an infinite loop. That is the same function
`agt`'s failure fingerprints serve — *this is the same failure you saw last
iteration; your change did nothing.*

- [Agentic Program Repair from Test Failures at Scale (arXiv 2507.18755)](https://arxiv.org/pdf/2507.18755)
- [Code as Agent Harness (arXiv 2605.18747)](https://arxiv.org/pdf/2605.18747)
- [How to Test an AI Agent Harness: The Six-Layer Guide](https://atlan.com/know/how-to-test-ai-agent-harness/)
- [From Question Answering to Task Completion: Agent System and Harness Design (arXiv 2606.20683)](https://arxiv.org/pdf/2606.20683)

## 4. Determinism and replay

**Deterministic simulation testing.** Randomness and determinism stop being
contradictory when every choice flows from one seed. Four dimensions must be
owned: **execution** (single-threaded, no scheduler variance), **entropy** (all
RNGs seeded), **time** (simulated clocks that advance only on explicit calls),
and **I/O** (mocked channels, in-memory service emulators).

The Rust position: `madsim` and `turmoil` (with `libc` overrides for
`getrandom` and `clock_gettime`), validated by *meta tests* that compare
byte-level logs across runs with the same seed. Reported result: 17 notable
distributed-systems bugs caught pre-production. Reported cost: controlling every
entropy source across an entire dependency tree, down to Rust's randomized
`HashMap` seeds. Nobody describes this as cheap.

**Record/replay, and where the seam goes.** The most important distinction found
in the survey. HTTP-level cassettes capture too much:

> An HTTP cassette that recorded a successful run will keep replaying that
> success even after you break the tool's implementation. The recording captures
> too much by freezing the LLM's decision and the tool's execution into a single
> opaque blob, whereas we want to only freeze the former.

The fix is to record *decisions* and replay them against *live tools*: recorded
tool input is dispatched to the real tool, the recorded output is yielded
alongside, and the live output's side effects still happen. Implementations of
this shape now exist for agent frameworks and for MCP traffic.

- [Deterministic simulation testing for async Rust — S2](https://s2.dev/blog/dst)
- [MadSim — Rust simulation library](https://lib.rs/crates/madsim)
- [awesome-deterministic-simulation-testing](https://github.com/ivanyu/awesome-deterministic-simulation-testing)
- [So, You Want to Learn More About DST? — Pierre Zemb](https://pierrezemb.fr/posts/learn-about-dst/)
- [Deterministic Testing for LangChain Agents — Sixty North](https://blog.sixty-north.com/deterministic-testing-for-langchain-agents.html)
- [langchain-replay](https://github.com/sixty-north/langchain-replay)
- [agent-vcr: record/replay for MCP](https://github.com/Jarvis2021/agent-vcr)
- [Deterministic AI Testing with Session Recording in cagent — Docker](https://www.docker.com/blog/deterministic-ai-testing-with-session-recording-in-cagent/)

## 5. The oracle problem, sharpened by agents

Classical: distinguishing correct from merely-observed behaviour is the hardest
part of automated testing.

What agents change:

**Self-deception.** Tests generated by the same model that wrote the code share
its blind spots. Property-based testing is the proposed circuit-breaker —
invariants are easier to state correctly than exhaustive input/output pairs.

**But LLM-generated properties are frequently weak.** Documented failure mode:
generated properties are trivial (asserting the return has the right type) or
outright wrong (asserting invariants that do not hold). Property-based testing
does not solve the oracle problem for agents; it relocates it.

**Reward hacking is measured, not hypothetical.** Execution-based tests are the
most reliable available reward for SWE tasks, and are subject to systematic
faithfulness failures. Filtering low-faithfulness tasks plus rollout-time
behaviour monitoring cut hacked resolution rates from **28.57% → 0.56%**, and
raised clean resolved rates **40.22% → 60.53%**.

**The verification horizon.** The structural limit: tests verify only what is in
scope; real tasks carry implicit requirements (quality, maintainability, design)
that tests cannot state; comprehensive coverage of complex systems is
impractical, leaving exploitable blind spots. The paper's own conclusion is that
there is no silver bullet — test-based reward is one tool inside a broader
evaluation frame, alongside human judgment and behavioural monitoring.

- [The Verification Horizon: No Silver Bullet for Coding Agent Rewards (arXiv 2606.26300)](https://arxiv.org/pdf/2606.26300)
- [Beyond Verifiable Rewards: Rubric-Based GRM for SWE Agents (arXiv 2604.16335)](https://arxiv.org/pdf/2604.16335)
- [Use Property-Based Testing to Bridge LLM Code Generation and Validation (arXiv 2506.18315)](https://arxiv.org/html/2506.18315v1)
- [Nexus: Execution-Grounded Multi-Agent Test Oracle Synthesis (arXiv 2510.26423)](https://arxiv.org/pdf/2510.26423)
- [LLM-Based Test Oracles: Source-of-Authority Taxonomy (arXiv 2607.05031)](https://arxiv.org/html/2607.05031)
- [Introducing SWE-bench Verified — OpenAI](https://openai.com/index/introducing-swe-bench-verified/)

## 6. Suite integrity under agent authorship

The practitioner literature on flaky tests has independently arrived at `agt`'s
third commitment:

> Repairs must appear as a reviewable diff rather than a silent mutation,
> because "the test was updated" and "the assertion was deleted" look identical
> in a green checkmark. Weakening changes should be visible as a category: an
> assertion that got looser, a case that got skipped, a timeout that grew.

And on where the damage hides:

> A weakened assertion hides most comfortably when a single change touches both
> application code and test code, because the reviewer's attention is on the
> feature and the test change reads as incidental cleanup.

**Quarantine** is the established pattern for separating "known unstable" from
"broken", and its definition is precise: the test keeps running and reporting,
only its ability to block a merge changes. That is exactly why `agt` gives
`Quarantined` its own verdict variant rather than folding it into skip.

- [Human in the Loop: Where People Belong in Agent-Written Tests — Shiplight](https://www.shiplight.ai/blog/human-in-the-loop-agent-quality)
- [Test Quarantine: Stop Flaky Tests From Blocking Merges — Mergify](https://mergify.com/learn/test-quarantine)
- [Flaky Test Quarantine — minware](https://www.minware.com/guide/best-practices/flaky-test-quarantine)

## 7. Machine-readable test output in Rust today

The gap `agt` is aimed at. Current state:

- libtest JSON output is **unstable**, with an accepted RFC (3558) and an
  ongoing project goal to finish the experiment.
- `cargo nextest` ships `--message-format libtest-json` / `libtest-json-plus`,
  explicitly framed as *compatibility with existing infrastructure* and
  documented as "not currently full-fidelity". JUnit XML is the main
  production-ready machine format.
- A first-class newline-delimited JSON format for runs is named as still
  missing.

So the ecosystem is converging on machine-readable *results* — which test, which
status, how long. Nothing in it carries expected-vs-observed structure, a
reproduce command, a minimized witness, or a failure identity stable across
runs. That is the layer `agt` is proposing.

- [Libtest JSON output — cargo-nextest](https://nexte.st/docs/machine-readable/libtest-json/)
- [RFC 3558: libtest JSON](https://rust-lang.github.io/rfcs/3558-libtest-json.html)
- [Finish the libtest json output experiment — Rust Project Goals 2026](https://rust-lang.github.io/rust-project-goals/2026/libtest-json.html)
- [cargo-mutants](https://lib.rs/crates/cargo-mutants) · [with nextest](https://nexte.st/docs/integrations/cargo-mutants/)
- [Faster Rust Testing at Scale: cargo-nextest in Practice — JetBrains](https://blog.jetbrains.com/rust/2026/05/01/faster-rust-tests-with-cargo-nextest/)

## What the survey settles

1. **"Agentic testing" is three ideas.** Any framework has to say which one it
   is. `agt` is the third — the suite as an interface for a non-human reader —
   and the third is the least occupied.
2. **Structured observations are the consensus interface.** Harness engineering
   assumes them; Rust's test tooling does not yet produce them at the fidelity a
   repair loop needs (§7). That gap is the product.
3. **Nobody is supplying the memory.** Every source describes a loop that reads
   failures; none describes a framework that tells the loop whether this failure
   is the same one as last time. Failure identity is the least-discussed and
   cheapest of the three consequences in [01-concept.md](01-concept.md).
4. **Cache-first execution and decision-level replay are the same insight**,
   reached independently from cost (§1) and from determinism (§4). Freeze the
   model's choice, keep the tool live.
5. **Test-based reward is simultaneously the best signal and a measured attack
   surface.** ~29% → ~0.6% under filtering is the number to design against.
6. **The verification horizon is a real ceiling.** The honest response is to make
   claim strength visible, not to promise coverage.
7. **Everyone who has looked closely concludes humans stay in the loop** — and
   converges on the same placement: autonomy on the execution loop, human review
   on the oracle.
