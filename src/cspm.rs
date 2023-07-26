//! LBJ's version of CSP-M.

use process::{Event, Process};

pub mod combinator;
pub mod process;

/// A CSP explicit graph.
pub type Explicit<CId, PId> = super::semantics::explicit::Explicit<Process<CId, PId>, Event<CId>>;

/// A CSP action.
pub type Action<CId> = super::semantics::action::Action<Event<CId>>;
