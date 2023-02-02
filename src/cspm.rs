//! LBJ's version of CSP-M.

/// A CSP-M event.
pub struct Event<Id> {
    /// Identifier of the base channel of this event.
    pub head: Id,
    /// The tail of this event.
    pub tail: Vec<EventSuffix<Id>>
}

/// A suffix in a CSP-M event.
pub enum EventSuffix<Id> {
    /// A dotted event suffix.
    Dot(Value),
    /// An inputted event suffix.
    In(Id),
    /// An outputted event suffix.
    Out(Value)
}

/// A CSP-M process.
pub enum Process<Id> {
    Prim(Primitive),
    /// Prefix.
    Prefix(Event<Id>, Box<Process<Id>>),
    /// Binary operation on processes.
    Binary(Binary<Id>)
}

/// A primitive process.
pub enum Primitive {
    /// Deadlock.
    Stop,
    /// Termination.
    Skip,
}

pub struct Binary<Id> {
    op: BinaryOp,
    lhs: Box<Process<Id>>,
    rhs: Box<Process<Id>>,
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
    Sliding
}