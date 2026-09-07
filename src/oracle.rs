//! Guarding the oracle.
//!
//! Test-based rewards are the most scalable signal we have for coding agents,
//! and they are also the easiest to game: an agent that can edit the suite can
//! delete the assertion instead of satisfying it. The two failures look
//! identical from the outside — both are a green check.
//!
//! So `agt` treats the oracle as the asset under protection. Every case reports
//! how strong its claim is, and every change to a case is classified as
//! strengthening, neutral, or weakening. Weakening is not forbidden; it is made
//! impossible to do quietly.

use serde::{Deserialize, Serialize};

use crate::evidence::Verdict;

/// How much a passing case actually proves.
///
/// Ordering is meaningful: `Smoke < Example < Snapshot < Property < Proof`.
/// A suite's strength profile is a first-class metric, so "coverage went up"
/// cannot hide "every new test is a smoke test".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Strength {
    /// It ran without panicking.
    Smoke,
    /// One input maps to one expected output.
    Example,
    /// Output matches a previously accepted rendering.
    Snapshot,
    /// An invariant holds over generated inputs.
    Property,
    /// Exhaustively or symbolically established.
    Proof,
}

/// How a change to a case moved its claim.
///
/// Emitted per-case on every diff that touches the suite, so a reviewer reads
/// the delta in what the tests *claim* rather than the delta in test lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Movement {
    /// The claim got stronger.
    Strengthened,
    /// Refactor with no change in what is proven.
    Neutral,
    /// The claim got weaker: looser assertion, wider tolerance, longer timeout.
    Weakened,
    /// The case stopped running: skipped, ignored, quarantined.
    Silenced,
    /// The case is gone.
    Removed,
}

impl Movement {
    /// Whether a change of this kind must be surfaced for human review.
    ///
    /// Deleting a wrong test is legitimate. Deleting an inconvenient one is
    /// not, and only a human looking at the diff can tell the two apart.
    #[must_use]
    pub fn needs_review(self) -> bool {
        matches!(
            self,
            Movement::Weakened | Movement::Silenced | Movement::Removed
        )
    }
}

/// A judgeable claim about the system under test.
pub trait Oracle {
    /// The input the claim is evaluated against.
    type Input;

    /// A stable name, used for fingerprints and for the suite ledger.
    fn name(&self) -> &str;

    /// How much this oracle proves when it passes.
    fn strength(&self) -> Strength;

    /// Evaluate the claim.
    fn judge(&self, input: &Self::Input) -> Verdict;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strength_is_ordered_from_weakest_to_strongest() {
        assert!(Strength::Smoke < Strength::Example);
        assert!(Strength::Property < Strength::Proof);
    }

    #[test]
    fn only_loosening_movements_demand_review() {
        assert!(!Movement::Strengthened.needs_review());
        assert!(!Movement::Neutral.needs_review());
        for m in [Movement::Weakened, Movement::Silenced, Movement::Removed] {
            assert!(m.needs_review(), "{m:?} must be reviewed");
        }
    }

    struct NonEmpty;
    impl Oracle for NonEmpty {
        type Input = String;
        fn name(&self) -> &str {
            "non_empty"
        }
        fn strength(&self) -> Strength {
            Strength::Property
        }
        fn judge(&self, input: &String) -> Verdict {
            if input.is_empty() {
                Verdict::Inconclusive {
                    reason: "no input supplied".into(),
                }
            } else {
                Verdict::Pass
            }
        }
    }

    #[test]
    fn an_oracle_can_decline_to_judge() {
        assert_eq!(NonEmpty.judge(&"x".to_string()), Verdict::Pass);
        assert!(matches!(
            NonEmpty.judge(&String::new()),
            Verdict::Inconclusive { .. }
        ));
    }
}
