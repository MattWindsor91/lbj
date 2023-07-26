//! Combinator rule representation.
use super::{action::Action, explicit::Node};

/// An ordinal used in a combinator.
#[derive(Copy, Clone, Debug)]
pub struct Ordinal(pub usize);

/// A combinator triple, parameterised over the type of primitive process.
#[derive(Clone, Debug)]
pub struct Triple<P> {
    /// The function from 'on' operator indices to contributed actions.
    pub on_func: OnFunc,
    /// The action occurring in the combinator.
    pub action: Action<OnFuncArg>,
    /// The format of the remaining process.
    pub format: Node<P>,
}

/// A combinator rule.
#[derive(Clone, Debug)]
pub struct Rule<P, Q> {
    /// A list of semantics-defined quantifiers over the action arguments.
    pub quantifiers: Vec<Q>,
    pub triple: Triple<P>,
}

/// A set of combinator rules defining an operator.
#[derive(Clone, Debug)]
pub struct RuleSet<P, Q> {
    /// The set of indices of arguments of this operator that are 'on'.
    pub on: Vec<usize>,
    /// The rules used in this operator's definition.
    pub rules: Vec<Rule<P, Q>>,
}

/// An 'on' function, represented as a list of partial mappings (one for each
/// 'on' argument).
#[derive(Clone, Debug)]
pub struct OnFunc(pub Vec<Option<OnFuncArg>>);

/// An argument in an 'on' function representation.
#[derive(Copy, Clone, Debug)]
pub struct OnFuncArg(pub usize);
