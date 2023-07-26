//! Actions.

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

impl<E> Action<E> {
    /// Replaces an event action with the result of `f` over it.
    pub fn expand_event<T>(&self, f: impl Fn(&E) -> Action<T>) -> Action<T> {
        match self {
            Self::Ev(e) => (f)(e),
            Self::Tick => Action::Tick,
            Self::Tau => Action::Tau,
        }
    }

    /// Maps `f` over this action if it is an event.
    pub fn map_event<T>(&self, f: impl Fn(&E) -> T) -> Action<T> {
        self.expand_event(|e| Action::Ev((f)(e)))
    }
}
