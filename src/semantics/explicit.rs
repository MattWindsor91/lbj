//! The explicit labelled transition system.
//!
//! An explicit LTS, as described in
//! [Gibson-Robinson et al. 2014](https://link.springer.com/chapter/10.1007/978-3-642-54862-8_13),
//! is simply a graph structure with nodes being process descriptions and edges being transitions.

use std::collections::{HashSet, VecDeque};

use petgraph::graph;
use petgraph::graph::NodeIndex;

/// A node of an explicit graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node<P> {
    /// A special process denoting termination.
    Omega,
    /// A process state.
    Process(P),
}

/// An edge in an explicit graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge<E> {
    /// The CSP action.
    pub action: super::action::Action<E>,
}

/// An explicit labelled transition system.
pub type Explicit<P, E> = graph::DiGraph<Node<P>, Edge<E>, Index>;

pub type Index = graph::DefaultIx;

/// Expands an explicit graph by applying a recursive visitor onto each node.
pub struct Expander<'f, P, E> {
    // TODO: make this into an animator, too
    to_visit: VecDeque<NodeIndex<Index>>,
    seen: HashSet<NodeIndex<Index>>,
    graph: Explicit<P, E>,
    visitor: &'f dyn Visitor<P, E>,
}

pub trait Visitor<P, E> {
    fn visit(&self, graph: &mut Explicit<P, E>, node: NodeIndex<Index>);
}

impl<'v, P, E> Expander<'v, P, E> {
    /// Expands the explicit graph by applying the visitor to each node.
    pub fn expand(mut self) -> Explicit<P, E> {
        while let Some(n) = self.to_visit.pop_back() {
            if !self.seen.insert(n) {
                continue;
            }

            self.visitor.visit(&mut self.graph, n);

            let new = self.graph.node_indices().filter(|x| !self.seen.contains(x));

            self.to_visit.extend(new);
        }

        self.graph
    }

    pub fn new(initial: P, visitor: &'v impl Visitor<P, E>) -> Self {
        let mut graph = Explicit::new();
        let initial = graph.add_node(Node { process: initial });

        let mut to_visit = VecDeque::new();
        to_visit.push_back(initial);

        let seen = HashSet::new();

        Self {
            graph,
            to_visit,
            seen,
            visitor,
        }
    }
}
