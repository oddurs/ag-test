//! `agt` — testing for a reader that isn't a person.
//!
//! A conventional test framework is written for a human reading a terminal. It
//! answers one question — *did it pass?* — and when the answer is no it prints a
//! diff and trusts the reader to know what that means.
//!
//! That reader is now usually an agent in a repair loop, and it has three
//! properties no test framework has had to design for. Each one implies a piece
//! of this crate:
//!
//! 1. **It has no memory.** Every run is a fresh read, so nothing tells it
//!    whether this failure is the one it was already working on. Hence
//!    [`Fingerprint`] — a stable failure identity, so *same failure* and *new
//!    failure* are facts rather than inferences. See [`evidence`].
//! 2. **It has no intuition.** It cannot tell a missing fixture from a real
//!    failure, and it cannot shrug at a flake. Hence [`Verdict::Inconclusive`],
//!    minimised witnesses, reproduce commands — and [`Seed`], because a reader
//!    that cannot discount noise needs a suite that does not produce any. See
//!    [`seed`].
//! 3. **It has an incentive to cheat.** It is graded by the suite and can edit
//!    it; deleting the assertion and fixing the bug look identical from outside.
//!    Hence [`Strength`] and [`oracle::Movement`]. See [`oracle`].
//!
//! The full argument, the research behind it, and the open problems are in
//! [`docs/`](https://github.com/oddurs/ag-test/tree/main/docs). This crate is a
//! **skeleton**: the types below fix the vocabulary and the wire format. The
//! runner is not built yet — see `ROADMAP.md`.

#![forbid(unsafe_code)]

pub mod evidence;
pub mod oracle;
pub mod seed;

pub use evidence::{Evidence, Expectation, Fingerprint, Observation, Verdict};
pub use oracle::{Oracle, Strength};
pub use seed::Seed;
