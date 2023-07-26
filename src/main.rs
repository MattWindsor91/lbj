use lbj::cspm::combinator::Combinators;
use lbj::cspm::process;
use lbj::semantics::explicit::Expander;
use petgraph::dot::Dot;
use std::collections::HashSet;

fn main() {
    let proc: process::Process<(), ()> = process::Process::Prim(process::Primitive::Skip);
    let visitor = Combinators::new(HashSet::new());
    let expander = Expander::new(proc, &visitor);
    let graph = expander.expand();
    println!("{:?}", Dot::with_config(&graph, &[]));
}
