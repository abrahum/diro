use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{error::DiroResult, Dice};

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
        Rule::adice => parse_adice(pair),
        Rule::cdice => parse_cdice(pair),
        Rule::fdice => parse_fdice(pair),
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
    let mut a = 0;
    for pair in pairs {
        match pair.as_rule() {
            Rule::base_dice => parse_base_dice(pair, &mut count, &mut face)?,
            Rule::b => parse_bp(pair, &mut bp, true)?,
            Rule::p => parse_bp(pair, &mut bp, false)?,
            Rule::k => parse_bp(pair, &mut kq, true)?,
            Rule::q => parse_bp(pair, &mut kq, false)?,
            Rule::a => parse_a(pair, &mut a)?,
            _ => unreachable!(),
        }
    }
    Ok(DiroAst::Dice(
        crate::Dice::_dice(count, face, bp, kq, a)?,
        None,
    ))
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

fn parse_a(pair: Pair<Rule>, a: &mut u16) -> DiroResult<()> {
    if let Some(pair) = pair.into_inner().next() {
        *a = pair.as_str().parse()?;
    }
    Ok(())
}

fn parse_adice(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    let pairs = pair.into_inner();
    let mut count = 1;
    let mut face = 10;
    let mut add_line = 11;
    let mut success_line = 8;
    let mut a = false;
    for pair in pairs {
        match pair.as_rule() {
            Rule::ad => a = true,
            Rule::uint => {
                if a {
                    add_line = pair.as_str().parse()?;
                } else {
                    count = pair.as_str().parse()?;
                }
            }
            Rule::k => parse_a(pair, &mut success_line)?,
            Rule::m => parse_a(pair, &mut face)?,
            _ => unreachable!(),
        }
    }
    Ok(DiroAst::Dice(
        Dice::ADice {
            count,
            face,
            add_line,
            success_line,
        },
        None,
    ))
}

fn parse_cdice(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    let pairs = pair.into_inner();
    let mut count = 1;
    let mut face = 10;
    let mut count_line = 11;
    let mut c = false;
    for pair in pairs {
        match pair.as_rule() {
            Rule::c => c = true,
            Rule::uint => {
                if c {
                    count_line = pair.as_str().parse()?;
                } else {
                    count = pair.as_str().parse()?;
                }
            }
            Rule::m => parse_a(pair, &mut face)?,
            _ => unreachable!(),
        }
    }
    Ok(DiroAst::Dice(
        Dice::CDice {
            count,
            face,
            count_line,
        },
        None,
    ))
}

fn parse_fdice(pair: Pair<Rule>) -> DiroResult<DiroAst> {
    let pairs = pair.into_inner();
    let mut count = 1;
    for pair in pairs {
        match pair.as_rule() {
            Rule::uint => count = pair.as_str().parse()?,
            _ => unreachable!(),
        }
    }
    Ok(DiroAst::Dice(Dice::FDice(count), None))
}

#[test]
fn parse_test() {
    let source = "-11 + 2 x ((2 + 2) - 1) / 3";
    let mut ast = parse(source).unwrap();
    println!(
        "{} = {} = {:?}",
        ast.s_expr(),
        ast.detail_expr().unwrap(),
        ast.eval()
    );
    println!("{} = {:?}", ast.expr(), ast.eval());
}

#[test]
fn parse_dice_test() {
    let source = "3d6+1";
    match parse(source) {
        Ok(mut ast) => {
            ast.roll();
            println!(
                "{} = {} = {:?}",
                ast.expr(),
                ast.detail_expr().unwrap(),
                ast.calc()
            )
        }
        Err(e) => println!("{}", e),
    }
}
