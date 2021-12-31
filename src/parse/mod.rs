use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::error::{DiroError, DiroResult};

mod ast;
pub use ast::*;

#[derive(Parser)]
#[grammar = "parse/diro.pest"]
struct DiroParser;

pub fn parse(source: &str) -> DiroResult<DiroAst> {
    let pairs = DiroParser::parse(Rule::main, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                return parse_expr(pair);
            }
            _ => {}
        }
    }
    Ok(DiroAst::Dice(crate::Dice::default(), None))
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
        _ => unreachable!(),
    }
}

fn parse_term(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::dice => parse_dice(pair),
        Rule::expr => parse_expr(pair).map(|a| DiroAst::Closed(Box::new(a))),
        Rule::int => Ok(DiroAst::Int(pair.as_str().parse()?)),
        _ => unreachable!(),
    }
}

fn parse_dyadic_expr(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    let mut pairs = pair.into_inner();
    let lpair = pairs.next().unwrap();
    let lhs = match lpair.as_rule() {
        Rule::term => parse_term(lpair)?,
        _ => unreachable!(),
    };
    let verb = parse_verb(pairs.next().unwrap())?;
    let rhs = parse_expr(pairs.next().unwrap())?;
    Ok(DiroAst::dyadic_with_priority(verb, lhs, rhs))
}

fn parse_dice(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    let pairs = pair.into_inner();
    let mut bp = 0;
    let mut kq = 0;
    let mut count = 1;
    let mut face = 100;
    for pair in pairs {
        match pair.as_rule() {
            Rule::base_dice => parse_base_dice(pair, &mut count, &mut face)?,
            Rule::b => parse_bp(pair, &mut bp, true)?,
            Rule::p => parse_bp(pair, &mut bp, false)?,
            Rule::k => parse_bp(pair, &mut kq, true)?,
            Rule::q => parse_bp(pair, &mut kq, false)?,
            _ => unreachable!(),
        }
    }
    if kq.abs() > count as i8 {
        return Err(DiroError::KQTooBig);
    }
    Ok(DiroAst::Dice(crate::Dice::new(count, face, bp, kq), None))
}

fn parse_base_dice(pair: Pair<Rule>, count: &mut u8, face: &mut u16) -> DiroResult<()> {
    let mut d = false;
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::d => d = true,
            Rule::uint => {
                if d {
                    *face = pair.as_str().parse()?;
                } else {
                    *count = pair.as_str().parse()?;
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn parse_bp(pair: Pair<Rule>, bp: &mut i8, b: bool) -> DiroResult<()> {
    if let Some(pair) = pair.into_inner().next() {
        let i: i8 = pair.as_str().parse()?;
        if b {
            *bp += i;
        } else {
            *bp -= i;
        }
    } else {
        if b {
            *bp += 1;
        } else {
            *bp -= 1;
        }
    }
    Ok(())
}

#[test]
fn parse_test() {
    let source = "-11 + 2 x ((2 + 2) - 1) / 3";
    let mut ast = parse(source).unwrap();
    println!("{} = {:?}", ast.s_expr(), ast.eval());
    println!("{} = {:?}", ast.expr(), ast.eval());
}

#[test]
fn parse_dice_test() {
    let source = "k2d6";
    match parse(source) {
        Ok(mut ast) => println!("{} = {:?}", ast.expr(), ast.eval()),
        Err(e) => println!("{}", e),
    }
}
