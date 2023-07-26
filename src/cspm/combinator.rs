//! Combinatorial semantics for CSP-M.

use std::{collections::HashSet, marker::PhantomData};

use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::visit::{IntoNodeReferences, NodeRef};

use crate::{
    cspm::process::Primitive,
    semantics::{
        action::Action,
        combinator::{self, Ordinal},
        explicit::{Edge, Index, Node, Visitor},
    },
};

use super::{
    process::{Event, Process},
    Explicit,
};

/// A CSP quantifier.
pub enum Quantifier {
    Universe,
}

impl Quantifier {
    /// Expands a quantifier into a vector of all possible satisfying events.
    pub fn expand<CId: Clone>(&self, universe: &HashSet<Event<CId>>) -> Vec<Event<CId>> {
        match self {
            Self::Universe => universe.clone().into_iter().collect_vec(),
        }
    }
}

/// A visitor for constructing CSP LTSes by applying combinator rules.
#[derive(Default)]
pub struct Combinators<CId, PId> {
    _p_phantom: Box<PhantomData<PId>>,

    /// The universe set, which must be bounded for an explicit graph.
    universe: HashSet<Event<CId>>,
}

pub type Triple<Id> = combinator::Triple<Process<Id, Ordinal>>;
pub type Rule<Id> = combinator::Rule<Process<Id, Ordinal>, Quantifier>;
pub type RuleSet<Id> = combinator::RuleSet<Process<Id, Ordinal>, Quantifier>;

struct ProcessExpander<'g, CId, PId> {
    graph: &'g mut Explicit<CId, PId>,
    node_id: NodeIndex<Index>,
    parent: &'g Combinators<CId, PId>,
}

impl<'g, CId: Clone, PId: Clone> ProcessExpander<'g, CId, PId> {
    fn visit(&mut self) {
        // Don't bother visiting an omega node.
        if let Node::Process(p) = &self.graph[self.node_id] {
            self.visit_process(p.clone());
        }
    }

    fn visit_process(&mut self, proc: Process<CId, PId>) {
        match proc {
            Process::Named(pid) => {}
            Process::Prim(p) => self.visit_prim(p),
            Process::Prefix(_, _) => {}
            Process::Binary(_) => {}
        }
    }

    fn visit_prim(&mut self, prim: Primitive) {
        let def: RuleSet<CId> = prim_rules(prim);
        self.apply_ruleset(&[], def);
    }

    fn apply_ruleset(&mut self, args: &[Process<CId, PId>], rules: RuleSet<CId>) {
        /* First, deduce the list of processes that are 'on' by mapping them
        against the 'on' indices in the rule set. */
        let on_processes: Vec<&Process<CId, PId>> =
            rules.on.iter().copied().map(|i| &args[i]).collect();

        for rule in rules.rules {
            self.apply_rule(args, &on_processes, rule);
        }
    }

    fn apply_rule(
        &mut self,
        args: &[Process<CId, PId>],
        on: &[&Process<CId, PId>],
        rule: Rule<CId>,
    ) {
        let assignments = self.expand_quants(&rule.quantifiers);

        for assignment in assignments {
            self.apply_triple(args, on, &assignment, &rule.triple)
        }
    }

    fn expand_quants(&self, quantifiers: &[Quantifier]) -> Vec<Vec<Event<CId>>> {
        if quantifiers.is_empty() {
            return vec![vec![]];
        }

        /* TODO: this will explode if the universe or quantifiers are anything
        other than tiny and simple. */
        quantifiers
            .iter()
            .map(|q| q.expand(&self.parent.universe))
            .multi_cartesian_product()
            .collect()
    }

    fn apply_triple(
        &mut self,
        args: &[Process<CId, PId>],
        on: &[&Process<CId, PId>],
        assignment: &[Event<CId>],
        triple: &Triple<CId>,
    ) {
        if !events_accepted(on, assignment) {
            return;
        }

        let dest = self.resolve_dest(args, &triple.format);
        let action = triple.action.map_event(|i| assignment[i.0].clone());

        self.graph.add_edge(self.node_id, dest, Edge { action });
    }

    fn resolve_dest(
        &mut self,
        args: &[Process<CId, PId>],
        format: &Node<Process<CId, Ordinal>>,
    ) -> NodeIndex<Index> {
        match format {
            Node::Omega => self.resolve_omega(),
            Node::Process(p) => self.resolve_process_dest(args, p),
        }
    }

    /// Finds an existing omega in the graph and returns an ID for it, if one exists.
    /// Otherwise, creates an omega and returns an ID for that.
    fn resolve_omega(&mut self) -> NodeIndex<Index> {
        if let Some(o) = self
            .graph
            .node_references()
            .find(|n| matches!(n.weight(), Node::Omega))
        {
            o.id()
        } else {
            self.graph.add_node(Node::Omega)
        }
    }

    fn resolve_process_dest(
        &mut self,
        args: &[Process<CId, PId>],
        format: &Process<CId, Ordinal>,
    ) -> NodeIndex<Index> {
        // TODO: handle recursion appropriately instead of always constructing a new process
        let proc = format.expand_pid(|i: Ordinal| args[i.0].clone());
        self.graph.add_node(Node::Process(proc))
    }
}

/// Checks that a triple is proposing events that all of the 'on' processes
/// can accept.
fn events_accepted<CId, PId>(on: &[&Process<CId, PId>], assignment: &[Event<CId>]) -> bool {
    // TODO: perform this acceptance check when generating the assignment?
    // TODO: make these two arrays equally long by construction
    return on
        .iter()
        .zip_eq(assignment.iter())
        .all(|(o, a)| event_accepted(o, a));
}

/// Checks that a triple is proposing an event that its 'on' process
/// can accept.
fn event_accepted<CId, PId>(on: &Process<CId, PId>, assignment: &Event<CId>) -> bool {
    false
}

fn prim_rules<Id>(prim: Primitive) -> RuleSet<Id> {
    match prim {
        Primitive::Stop => combinator::RuleSet {
            on: vec![],
            rules: vec![],
        },
        Primitive::Skip => combinator::RuleSet {
            on: vec![],
            rules: vec![combinator::Rule {
                quantifiers: vec![],
                triple: combinator::Triple {
                    on_func: combinator::OnFunc(vec![]),
                    action: Action::Tick,
                    format: Node::Omega,
                },
            }],
        },
    }
}

impl<CId, PId> Combinators<CId, PId> {
    /// Constructs a new combinator-based visitor with the given universe set.
    pub fn new(universe: HashSet<Event<CId>>) -> Self {
        Self {
            _p_phantom: Box::default(),
            universe,
        }
    }
}

impl<CId: Clone, PId: Clone> Visitor<Process<CId, PId>, Event<CId>> for Combinators<CId, PId> {
    fn visit(&self, graph: &mut Explicit<CId, PId>, node_id: NodeIndex<Index>) {
        ProcessExpander {
            graph,
            node_id,
            parent: self,
        }
        .visit();
    }
}
