//! Actions.

use super::super::cspm::Event;

/// Enumeration of things that can be tick, tau, or some type of event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action<E> {
    /// Termination.
    Tick,
    /// Internal action.
    Tau,
    /// An event.
    Ev(E),
}
