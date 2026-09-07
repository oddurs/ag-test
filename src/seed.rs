//! Determinism, because the reader cannot shrug.
//!
//! A flaky test is mildly annoying to a person: you shrug, re-run, move on. The
//! shrug is the whole mechanism, and it requires knowing that some failures do
//! not mean anything. An agent cannot shrug — it attributes a random pass to
//! whatever it changed last and keeps a change that did nothing.
//!
//! So noise in the suite is not an annoyance here; it is corruption of the
//! signal driving a control loop. Deterministic simulation testing — the
//! FoundationDB lineage, and in Rust `madsim` and `turmoil` — already makes a
//! run a pure function of one seed by owning execution, entropy, time and I/O.
//! Agentic systems add a fifth source: model sampling.

use serde::{Deserialize, Serialize};

/// The single point every random choice in a run flows from.
///
/// A seed is printed on every run, carried in every [`crate::Evidence`] record,
/// and accepted by the runner, so "reproduce this failure" is a fact rather
/// than a hope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Seed(pub u64);

impl Seed {
    /// Derive a child seed for a sub-scope (a task, a simulated host, a case).
    ///
    /// Splitting rather than sharing means adding a test does not shift the
    /// random choices made by unrelated tests — the property that makes a seed
    /// worth reporting in the first place.
    #[must_use]
    pub fn derive(self, label: &str) -> Seed {
        // FNV-1a over the label, mixed into the parent. Stable across releases
        // and across platforms, which matters more here than distribution.
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in label.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Seed(self.0.rotate_left(17) ^ hash)
    }
}

impl std::fmt::Display for Seed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

/// Sources of nondeterminism a run must own to be replayable.
///
/// Anything not on this list is a leak, and leaks are what make a suite flaky
/// — which in an agentic loop is not an annoyance but a corrupted reward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Entropy {
    /// Task scheduling and thread interleaving.
    Execution,
    /// RNGs, hash seeds, UUIDs.
    Randomness,
    /// Clocks, timeouts, sleeps.
    Time,
    /// Network, filesystem, subprocesses.
    Io,
    /// Model sampling — the source classical DST never had to model.
    Sampling,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_seeds_are_stable_and_distinct() {
        let root = Seed(0x1234_5678_9abc_def0);
        assert_eq!(root.derive("a"), root.derive("a"));
        assert_ne!(root.derive("a"), root.derive("b"));
        assert_ne!(root.derive("a"), root);
    }

    #[test]
    fn seeds_render_as_fixed_width_hex() {
        assert_eq!(Seed(255).to_string(), "00000000000000ff");
    }
}
