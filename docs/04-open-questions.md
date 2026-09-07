# Open questions

Where the concept in [01-concept.md](01-concept.md) is weakest. Kept in the repo
because a design document that only argues for itself is not worth reading.

## Does anyone want this?

The honest case against: agents are getting better at parsing unstructured
output, and "the model will just handle it" has been the correct bet more often
than not for three years running. If frontier models reliably extract structure
from `cargo test` stderr, commitment 1 buys convenience, not capability.

The counter is that commitments 2 and 3 do not degrade the same way.
Reproducibility and oracle integrity are properties of the *system*, and no
amount of model capability supplies them from outside. A better model cannot
un-flake a suite, and a better model is, if anything, *better* at finding the
assertion that is cheaper to delete than to satisfy.

So the framework's real value may be entirely in 2 and 3, with 1 as the delivery
mechanism. If that is true, the emphasis is currently wrong.

## Is strength classification decidable?

`Movement` classification assumes you can look at two versions of an assertion
and say which proves more. Easy cases are easy: a widened tolerance, a raised
timeout, an added `#[ignore]`.

Hard cases are genuinely hard. Is replacing three example tests with one
property test a strengthening? Usually yes, but not if the property is trivially
true. Is narrowing a generator's input range a weakening, or a correction of a
generator that was producing invalid inputs? The mutation score gives an
empirical tiebreaker — did the change kill more mutants or fewer — but mutation
runs are expensive and scoped to the diff, so it is not always available.

Likely outcome: a three-way classifier — `Strengthened` / `Weakened` /
`Undetermined` — where `Undetermined` also routes to human review. Defaulting to
review is the safe direction, and the risk is review fatigue, which is the
failure mode that kills every mandatory-review mechanism eventually.

## Fingerprint stability is the load-bearing assumption

Everything stateful about the repair loop depends on fingerprints being stable
across unrelated edits and unstable across meaningful ones. Both failure
directions are bad:

- **Too stable** and two genuinely different bugs collide under one identity, so
  the agent believes it fixed something it did not.
- **Too unstable** and every edit looks like new-failure-appeared, so the loop
  loses its history and thrashes.

Normalizing the observation is where this is decided, and normalization is
domain-specific — timestamps, addresses, allocation-order-dependent output,
`HashMap` iteration order. There may be no general answer, only a per-project
normalizer that itself has to be reviewed. That would be an unwelcome amount of
configuration for a framework whose pitch is "it just gives you better signal".

## The oracle ledger can be edited

The ledger is a file in the repo. An agent with write access to the repo has
write access to the ledger.

Mitigations, none complete: hash the ledger into the commit and verify in CI
where the agent has no credentials; store mutation scores server-side; require
the ledger diff to be signed by the CI run rather than the working tree. All of
these amount to *keep the referee outside the sandbox*, which is right, and none
of them help a project without CI.

There is a broader version of this problem that the framework cannot solve: the
verification horizon is real, tests only ever prove what is in scope, and a
sufficiently capable agent optimizing against any fixed oracle will find the
gap. The design goal is to make the gap *visible*, not closed.

## Decision replay may be too fragile

Recording decisions and replaying them against live tools is the right seam, but
it assumes the decision is meaningful independent of the tool's response. That
holds for a single tool call. It gets shakier over a long trajectory: if the
live tool now returns something different from what was recorded, the *next*
recorded decision was made in a context that no longer exists.

Options, all unsatisfying: replay until first divergence then fall back to live
sampling (expensive, nondeterministic again); treat divergence as
`Inconclusive` (safe, and cassettes rot fast); or record a decision *policy*
rather than a decision trace (much harder, and possibly just "train a small
model"). The first is probably right, and it means replay is a cost optimization
with a determinism ceiling rather than a determinism guarantee.

## Cost

Full DST requires owning every entropy source in the dependency tree, down to
`HashMap` seeds. The teams who have done this describe it as a large, ongoing
engineering investment, and they were building databases, where the payoff is
unambiguous. Mutation scoring on every proposed test adds a second expensive
loop on top.

A framework that is only worth it for distributed systems teams with a
dedicated testing budget is a much smaller framework than this document
implies. The mitigation is to make each commitment independently adoptable —
evidence records without DST, DST without mutation scoring — and that constrains
the design toward loose coupling everywhere.

## Naming

The crate is `agt` because the repository is `ag-test`. Neither name says
anything. If the concept survives, the name should be chosen after the API
stabilizes, not before.
