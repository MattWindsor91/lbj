//! PEG parser for CSP-M.

use crate::cspm::process::Primitive;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "cspm/parser/cspm.pest"]
pub struct CspMParser;

type Process<'i> = super::Process<&'i str, &'i str>;
type Session<'i> = super::Session<&'i str, &'i str>;

pub fn parse<'i>(input: &'i str) -> Result<Session<'i>> {
    let mut session = Session::default();

    let pair = CspMParser::parse(Rule::cspm, input)
        .map_err(Box::new)?
        .next()
        .unwrap();
    let Rule::cspm = pair.as_rule() else {
        unreachable!();
    };
    parse_cspm(&mut session, pair.into_inner());

    Ok(session)
}

fn parse_cspm<'i>(session: &mut Session<'i>, pairs: Pairs<'i, Rule>) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::def => parse_def(session, pair.into_inner().next().unwrap()),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}

fn parse_def<'i>(session: &mut Session<'i>, pair: Pair<'i, Rule>) {
    match pair.as_rule() {
        Rule::process_def => parse_process_def(session, pair.into_inner()),
        _ => unreachable!(),
    }
}

fn parse_process_def<'i>(session: &mut Session<'i>, pairs: Pairs<'i, Rule>) {
    let mut name: &'i str = "";
    for pair in pairs {
        match pair.as_rule() {
            Rule::process_name => name = pair.as_str(),
            Rule::process_body => {
                _ = session
                    .processes
                    .insert(name, parse_process_body(pair.into_inner().next().unwrap()))
            }
            _ => unreachable!(),
        }
    }
}

fn parse_process_body(pair: Pair<Rule>) -> Process {
    match pair.as_rule() {
        Rule::prim_process => Process::Prim(parse_prim_process(pair.into_inner().next().unwrap())),
        _ => unreachable!(),
    }
}

fn parse_prim_process(pair: Pair<Rule>) -> Primitive {
    match pair.as_rule() {
        Rule::skip => Primitive::Skip,
        Rule::stop => Primitive::Stop,
        _ => unreachable!(),
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Parsing error")]
    Parse(#[from] Box<pest::error::Error<Rule>>),
}
