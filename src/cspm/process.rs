//! The CSP-M process model.

/// A CSP-M event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event<CId> {
    /// Identifier of the base channel of this event.
    pub head: CId,
    /// The tail of this event.
    pub tail: Vec<EventSuffix<CId>>,
}

/// A suffix in a CSP-M event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventSuffix<CId> {
    /// A dotted event suffix.
    Dot(Value<CId>),
    /// An inputted event suffix.
    In(CId),
    /// An outputted event suffix.
    Out(Value<CId>),
}

/// A CSP-M value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value<CId> {
    /// An identifier.
    Id(CId),
}

/// A CSP-M process.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Process<CId, PId> {
    /// A named process.
    Named(PId),
    /// A primitive process.
    Prim(Primitive),
    /// Prefix.
    Prefix(Event<CId>, Box<Process<CId, PId>>),
    /// Binary operation on processes.
    Binary(Binary<CId, PId>),
}

impl<CId: Clone, PId: Copy> Process<CId, PId> {
    /// Maps `f` over all process IDs in a process, expanding them into processes.
    pub fn expand_pid<N>(&self, f: impl Fn(PId) -> Process<CId, N>) -> Process<CId, N> {
        // Wrapper function to avoid infinite reference type construction
        fn go<CId: Clone, P: Copy, N>(
            that: &Process<CId, P>,
            f: &impl Fn(P) -> Process<CId, N>,
        ) -> Process<CId, N> {
            match that {
                Process::Named(pid) => (f)(*pid),
                Process::Prim(p) => Process::Prim(*p),
                Process::Prefix(x, y) => {
                    let x = (*x).clone();
                    let y = Box::new(go(y, f));
                    Process::Prefix(x, y)
                }
                Process::Binary(Binary { op, lhs, rhs }) => {
                    let op = *op;
                    let lhs = Box::new(go(lhs, f));
                    let rhs = Box::new(go(rhs, f));
                    Process::Binary(Binary { op, lhs, rhs })
                }
            }
        }
        go(self, &f)
    }

    /// Maps `f` over all process IDs in a process.
    pub fn map_pid<N>(&self, f: impl Fn(PId) -> N) -> Process<CId, N> {
        self.expand_pid(|p| Process::Named((f)(p)))
    }
}

/// A primitive process.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Primitive {
    /// Deadlock.
    Stop,
    /// Termination.
    Skip,
}

/// A binary operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binary<CId, PId> {
    op: BinaryOp,
    lhs: Box<Process<CId, PId>>,
    rhs: Box<Process<CId, PId>>,
}

/// A binary operator.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BinaryOp {
    /// A choice operator.
    Choice(ChoiceOp),
    /// An interrupt.
    Interrupt,
}

/// A choice operator.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ChoiceOp {
    /// External choice (`[]`).
    External,
    /// Internal choice (`|~|`).
    Internal,
    /// Sliding choice (`|>`).
    Sliding,
}
