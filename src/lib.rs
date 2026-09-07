//! `agt` — agentic-first testing.
//!
//! A conventional test framework is written for a human reading a terminal.
//! It answers one question — *did it pass?* — and when the answer is no it
//! prints a diff and hopes the reader knows what to do next.
//!
//! `agt` assumes the reader is an autonomous agent working a repair loop, and
//! that the same agent is proposing new tests. That inverts three things:
//!
//! 1. **Failures are evidence, not prose.** Every failing assertion produces an
//!    [`Evidence`] record: what was expected, what was observed, the seed that
//!    produced it, a stable [`Fingerprint`], and the exact command to reproduce
//!    it. See [`evidence`].
//! 2. **Determinism is a resource the suite manages.** A repair loop is only as
//!    good as the signal driving it, so every run is seeded and every failure is
//!    replayable byte-for-byte. See [`seed`].
//! 3. **The oracle defends itself.** When the agent under evaluation also writes
//!    the tests, assertion strength becomes the thing worth guarding. See
//!    [`oracle`].
//!
//! The concept, the research behind it, and the open problems are written up in
//! [`docs/`](https://github.com/oddurs/ag-test/tree/main/docs). This crate is
//! currently a **skeleton**: the types below fix the vocabulary and the wire
//! format. The runner is not built yet.

#![forbid(unsafe_code)]

pub mod evidence;
pub mod oracle;
pub mod seed;

pub use evidence::{Evidence, Expectation, Fingerprint, Observation, Verdict};
pub use oracle::{Oracle, Strength};
pub use seed::Seed;
