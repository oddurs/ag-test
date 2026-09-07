//! The failure record: what an agent actually consumes.
//!
//! Harness-engineering practice is consistent on this point — an agent should
//! receive a *structured observation* for every outcome, success or failure,
//! rather than a formatted string it has to parse back into meaning. Rust's own
//! ecosystem is moving the same way (the libtest JSON RFC, `cargo nextest`'s
//! machine-readable formats), but those describe *that* a test failed. Evidence
//! describes *how*, and what to do next.

use serde::{Deserialize, Serialize};

use crate::seed::Seed;

/// The outcome of a single test case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "verdict")]
pub enum Verdict {
    /// The oracle held.
    Pass,
    /// The oracle was violated; the record explains how.
    Fail(Box<Evidence>),
    /// The case could not be judged — a missing fixture, an unavailable
    /// dependency. Deliberately distinct from `Fail`: an agent must not "fix" a
    /// broken harness by changing application code.
    Inconclusive {
        /// Why no verdict could be reached.
        reason: String,
    },
    /// Known-failing and excluded from the gate, but still reported.
    Quarantined {
        /// Why the case is quarantined, and what would release it.
        reason: String,
    },
}

/// A structured failure: everything needed to understand, reproduce and repair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    /// Stable identity of the failure across runs and across machines.
    pub fingerprint: Fingerprint,
    /// The test case that failed.
    pub case: String,
    /// What the oracle required.
    pub expected: Expectation,
    /// What actually happened.
    pub observed: Observation,
    /// The seed the run was derived from.
    pub seed: Seed,
    /// A command that reproduces this exact failure.
    pub reproduce: String,
    /// The minimized input, when the case was shrunk to a smaller witness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness: Option<String>,
    /// Ordered, cheapest-first suggestions for narrowing the cause further.
    ///
    /// These are *probes*, not fixes. The framework knows how to get more
    /// information; it does not pretend to know what the code should do.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next_probes: Vec<String>,
}

/// A content-addressed failure identity.
///
/// Fingerprints are what make a repair loop stateful: the same fingerprint
/// across runs means no progress, a new one means the change moved the failure
/// rather than fixing it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Fingerprint(pub String);

/// What the oracle required of the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Expectation {
    /// A concrete value was required.
    Value {
        /// Rendered expected value.
        expected: String,
    },
    /// A named invariant was required to hold for all inputs.
    Property {
        /// Human-readable statement of the invariant.
        statement: String,
    },
    /// The output had to match a stored snapshot.
    Snapshot {
        /// Path to the accepted snapshot.
        path: String,
    },
    /// A rubric was applied by a judge rather than by code.
    ///
    /// Recorded separately because a judged verdict is evidence of a different
    /// quality than an executed one, and reviewers should be able to see which
    /// gates rest on it.
    Judged {
        /// The rubric applied.
        rubric: String,
        /// The judge that applied it.
        judge: String,
    },
}

/// What the system actually did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    /// Rendered observed value or behaviour.
    pub value: String,
    /// The sequence of externally visible actions taken, when recorded.
    ///
    /// For an agent under test the trajectory is often the real subject:
    /// output-only checks miss duplicated tool calls and unsafe intermediate
    /// steps that a correct final answer hides.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trajectory: Vec<String>,
}

impl Evidence {
    /// Render as a single line of newline-delimited JSON.
    ///
    /// One record per line is the whole interface: an agent reads a stream, a
    /// human reads a pretty-printer built over the same stream.
    ///
    /// # Errors
    /// Returns an error if the record cannot be serialized.
    pub fn to_ndjson(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Evidence {
        Evidence {
            fingerprint: Fingerprint("f00d".into()),
            case: "parser::rejects_trailing_comma".into(),
            expected: Expectation::Property {
                statement: "every accepted input round-trips to itself".into(),
            },
            observed: Observation {
                value: "accepted `[1,]`".into(),
                trajectory: vec![],
            },
            seed: Seed(7),
            reproduce: "agt run --seed 0000000000000007 parser::rejects_trailing_comma".into(),
            witness: Some("[1,]".into()),
            next_probes: vec!["agt shrink parser::rejects_trailing_comma".into()],
        }
    }

    #[test]
    fn evidence_round_trips_through_json() {
        let line = sample().to_ndjson().expect("serialize");
        assert!(!line.contains('\n'), "ndjson records must be one line");
        let back: Evidence = serde_json::from_str(&line).expect("deserialize");
        assert_eq!(back, sample());
    }

    #[test]
    fn empty_optional_fields_stay_out_of_the_wire_format() {
        let mut e = sample();
        e.witness = None;
        e.next_probes.clear();
        let line = e.to_ndjson().expect("serialize");
        assert!(!line.contains("witness"));
        assert!(!line.contains("next_probes"));
    }

    #[test]
    fn verdicts_are_externally_tagged_for_cheap_dispatch() {
        let json = serde_json::to_string(&Verdict::Pass).expect("serialize");
        assert_eq!(json, r#"{"verdict":"pass"}"#);
    }
}
