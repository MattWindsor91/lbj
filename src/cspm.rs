//! LBJ's version of CSP-M.

use process::{Event, Process};
use std::collections::HashMap;

pub mod combinator;
pub mod parser;
pub mod process;

/// A CSP explicit graph.
pub type Explicit<CId, PId> = super::semantics::explicit::Explicit<Process<CId, PId>, Event<CId>>;

/// A CSP action.
pub type Action<CId> = super::semantics::action::Action<Event<CId>>;

/// A CSP-M session.
#[derive(Clone, Debug, Default)]
pub struct Session<CId, PId> {
    /// Table of processes.
    pub processes: HashMap<PId, Process<CId, PId>>,
}
