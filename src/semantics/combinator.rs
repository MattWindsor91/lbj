use crate::semantics::action::Action;
use std::collections::HashSet;

/// An ordinal used in a combinator.
pub struct Ordinal(usize);

/// A combinator triple, parameterised over the type of primitive process.
pub struct Triple<P> {
    /// The function from 'on' operator indices to contributed actions.
    pub on_func: OnFunc,
    /// The action occurring in the combinator.
    pub action: Action<OnFuncArg>,
    /// The format of the remaining process.
    pub format: Process<P>,
}

pub type Quantifier<P, E> = fn(&P, HashSet<E>) -> HashSet<E>;

pub struct Rule<P, E> {
    pub quantifiers: Vec<Quantifier<P, E>>,
    pub triple: Triple<P>,
}

pub struct Operator<P, E> {
    pub on_indices: Vec<usize>,
    pub rules: Vec<Rule<P, E>>,
}

pub struct OnFunc(Vec<Option<OnFuncArg>>);

/*
pub fn translate<Id>(state: Node<Id>, universe: HashSet<Id>) -> Explicit<Id> {
    match state.process {
        case Process::Prim(_) => {}
        Process::Prefix(_, _) => {}
        Process::Binary(_) => {}
    }
}

 */

pub struct OnFuncArg(usize);
