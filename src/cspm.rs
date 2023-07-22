//! LBJ's version of CSP-M.

/// A CSP-M event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event<Id> {
    /// Identifier of the base channel of this event.
    pub head: Id,
    /// The tail of this event.
    pub tail: Vec<EventSuffix<Id>>,
}

/// A suffix in a CSP-M event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventSuffix<Id> {
    /// A dotted event suffix.
    Dot(Value<Id>),
    /// An inputted event suffix.
    In(Id),
    /// An outputted event suffix.
    Out(Value<Id>),
}

/// A CSP-M value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value<Id> {
    /// An identifier.
    Id(Id),
}

/// A CSP-M process.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Process<Id> {
    Prim(Primitive),
    /// Prefix.
    Prefix(Event<Id>, Box<Process<Id>>),
    /// Binary operation on processes.
    Binary(Binary<Id>),
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
    Sliding,
}

/// A CSP explicit graph.
pub type Explicit<Id> = super::semantics::explicit::Explicit<Process<Id>, Event<Id>>;

/// A CSP action.
pub type Action<Id> = super::semantics::action::Action<Event<Id>>;
