use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::error::{DiroError, DiroResult};

mod ast;
use ast::*;

#[derive(Parser)]
#[grammar = "parse/diro.pest"]
struct DiroParser;

pub(crate) fn parse(source: &str) -> DiroResult<DiroAst> {
    let pairs = DiroParser::parse(Rule::main, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                return parse_expr(pair);
            }
            _ => {}
        }
    }
    Ok(DiroAst::Empty)
}

fn parse_expr(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    match pair.as_rule() {
        Rule::expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::dyadic_expr => parse_dyadic_expr(pair),
        Rule::term => parse_term(pair),
        _ => unreachable!(),
    }
}

fn parse_verb(pair: Pair<Rule>) -> DiroResult<Verb> {
    match pair.as_str() {
        "+" => Ok(Verb::Plus),
        "-" => Ok(Verb::Minus),
        "x" => Ok(Verb::Times),
        "X" => Ok(Verb::Times),
        "*" => Ok(Verb::Times),
        "/" => Ok(Verb::Divide),
        "%" => Ok(Verb::Modulo),
        "^" => Ok(Verb::Power),
        _ => Err(DiroError::UnknownVerb(pair.as_str().to_owned())),
    }
}

fn parse_term(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::expr => parse_expr(pair).map(|a| DiroAst::Closed(Box::new(a))),
        Rule::int => Ok(DiroAst::Int(pair.as_str().parse()?)),
        _ => unreachable!(),
    }
}

fn parse_dyadic_expr(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    let mut pair = pair.into_inner();
    let lpair = pair.next().unwrap();
    let lhs = match lpair.as_rule() {
        Rule::term => parse_term(lpair)?,
        _ => unreachable!(),
    };
    let verb = parse_verb(pair.next().unwrap())?;
    let rhs = parse_expr(pair.next().unwrap())?;
    Ok(DiroAst::dyadic_with_priority(verb, lhs, rhs))
}

#[test]
fn parse_test() {
    let source = "11 + 2 / (2 + 2 - 1)";
    let ast = parse(source).unwrap();
    println!("{} = {}", ast.expr(), ast.eval());
}
