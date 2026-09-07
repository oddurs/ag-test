# Open questions

Where the design is weakest. Kept in the repository because a design document
that only argues for itself is not worth reading, and because an agent working
this codebase should know which parts are load-bearing and which are guesses.

## Is the reader really the problem?

The honest case against the whole thesis: models keep getting better at reading
unstructured output. "The model will just handle it" has been the right bet more
often than not for three years. If a frontier model reliably extracts structure
from `cargo test` stderr, then the *no intuition* consequence buys convenience
rather than capability, and the product shrinks to a nicer reporter.

Two of the three properties do not degrade that way, though:

- **No memory** is a property of the *protocol*, not the model. A better model
  still cannot know whether this failure is the one it saw last iteration unless
  something tells it. It can only guess more plausibly, which is worse.
- **An incentive to cheat** gets *worse* with capability. A more capable agent is
  better at finding the edit that is cheaper than the fix, and that edit is often
  in the test file.

So the emphasis may currently be wrong. If the framework's value turns out to be
entirely in identity and integrity, with structured evidence as merely the
delivery mechanism, the docs and the roadmap should say so — and the reporter
rung of the adoption ladder becomes a lure rather than the point.

## Fingerprint stability is the load-bearing assumption

Everything stateful rests on fingerprints being stable across unrelated edits and
unstable across meaningful ones. Both failure directions are bad, and they are
not symmetric in consequence:

- **too stable** → two different bugs share an identity, the agent believes it
  fixed something it did not, and the error is silent;
- **too unstable** → every edit looks like a new failure, the loop loses its
  history, and the error is at least loud.

Normalising the observation is where this is decided, and normalisation is
domain-specific — timestamps, addresses, allocation-order-dependent output,
`HashMap` iteration order, floating-point formatting. There may be no general
answer, only a per-project normaliser which itself has to be reviewed. That is an
unwelcome amount of configuration for a tool whose pitch is "it just gives you
better signal", and it is the single most likely place for the design to fail
quietly in the field.

Mitigation to try first: ship a default normaliser, and have the runner *detect*
instability — run the same failure twice under the same seed and report a
fingerprint that does not reproduce as a framework-level defect rather than a
test failure.

## Is movement classification decidable?

Classification assumes you can compare two versions of an assertion and say which
proves more. The easy cases are easy: a widened tolerance, a raised timeout, an
added `#[ignore]`.

The hard cases are genuinely hard. Is replacing three example tests with one
property test a strengthening? Usually — but not if the property is trivial. Is
narrowing a generator's range a weakening, or a correction of a generator that
was producing invalid inputs? Mutation score gives an empirical tiebreaker, but
mutation runs are expensive and diff-scoped, so it is not always available.

Hence `Undetermined`, which routes to review. Defaulting to review is the safe
direction. The cost is review fatigue, which is the failure mode that eventually
kills every mandatory-review mechanism, and the mitigation — be aggressive about
auto-classifying `Neutral` — is exactly the direction that risks missing a real
weakening.

## The ledger lives inside the sandbox

The ledger is a file in a repository the agent can write to.

Partial mitigations, none complete: hash the ledger into the commit and verify in
CI where the agent has no credentials; recompute mutation scores in CI rather
than trusting the working tree; fail a ledger change with no corresponding test
change. All of these amount to *keep the referee outside the sandbox*, which is
right, and none of them help a project without CI.

The broader version cannot be solved at all: the verification horizon is real,
tests only prove what is in scope, and a sufficiently capable agent optimising
against any fixed oracle will find the gap. The design goal is to make the gap
visible, not to close it — and a product that oversells this is worse than no
product, because it converts a known risk into an assumed safety.

## Decision replay may be too fragile

Recording decisions and replaying them against live tools is the right seam, but
it assumes a decision is meaningful independent of the tool's response. That
holds for one call. Over a long trajectory it weakens: if the live tool now
returns something different from what was recorded, the *next* recorded decision
was made in a context that no longer exists.

The options are all unsatisfying — `strict` (divergence is `Inconclusive`; safe,
but cassettes rot fast), `until-divergence` (replay then fall back to live
sampling; expensive, and nondeterministic again), or recording a decision
*policy* rather than a trace (much harder, and possibly just "train a small
model"). The architecture picks `strict` by default, which means replay is
honestly a cost optimisation with a determinism ceiling rather than a determinism
guarantee. The docs should keep saying that.

## Cost, and who it is really for

Full deterministic simulation requires owning every entropy source in the
dependency tree, down to `HashMap` seeds. The teams who have done it describe a
large, ongoing investment — and they were building databases, where the payoff is
unambiguous. Diff-scoped mutation scoring adds a second expensive loop.

A framework only worth it for distributed-systems teams with a dedicated testing
budget is much smaller than these documents imply. The mitigation is the adoption
ladder in [04-product.md](04-product.md): each rung has to stand alone, which
constrains the architecture toward loose coupling everywhere. If rung 1 ever
starts requiring rung 2, the product has failed regardless of whether the code
works.

## Will anyone adopt a protocol?

`agt serve` assumes agent harnesses will speak to a test framework as a service
rather than running `cargo test` and reading stdout. That is a bet on other
people's roadmaps. If it does not happen, the NDJSON stream still works — it is
just parsed from a pipe instead of called as a method, losing `delta` and
`propose_case`, which are the two most valuable operations.

Worth building the stream first and the protocol second for exactly this reason,
which is how the roadmap is ordered.

## The measurable claim might not show up

The product doc commits to two numbers: fewer loop iterations per resolved
failure, and fewer merged diffs that weakened an oracle. Both are measurable, and
neither has been measured. If they do not move on a real codebase, the thesis is
wrong and the correct response is to say so here rather than to add features.

Instrumenting this needs a baseline captured before adoption, which means the
measurement work belongs early in the roadmap, not at the end where it would
conveniently arrive after the design is fixed.
