use std::fmt::Display;

use crate::{
    error::{DiroError, DiroResult},
    Dice, RollResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiroAst {
    Int(i32),
    Dice(Dice, Option<RollResult>),
    DyadicOP {
        verb: Verb,
        lhs: Box<DiroAst>,
        rhs: Box<DiroAst>,
    },
    Closed(Box<DiroAst>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verb {
    Plus,   // +
    Minus,  // -
    Times,  // x
    Divide, // /
    Modulo, // %
    Power,  // ^
}

impl DiroAst {
    pub fn eval(&mut self) -> DiroResult<i32> {
        self.roll();
        self.calulate()
    }

    pub fn roll(&mut self) {
        match self {
            DiroAst::Dice(dice, result) => *result = Some(dice.roll()),
            DiroAst::Closed(ast) => ast.roll(),
            DiroAst::DyadicOP { lhs, rhs, .. } => {
                lhs.roll();
                rhs.roll();
            }
            _ => {}
        }
    }

    pub fn calulate(&self) -> DiroResult<i32> {
        match self {
            DiroAst::Int(i) => Ok(*i),
            DiroAst::DyadicOP { verb, lhs, rhs } => match verb {
                Verb::Plus => Ok(lhs.calulate()? + rhs.calulate()?),
                Verb::Minus => Ok(lhs.calulate()? - rhs.calulate()?),
                Verb::Times => Ok(lhs.calulate()? * rhs.calulate()?),
                Verb::Divide => {
                    let r = rhs.calulate()?;
                    if r == 0 {
                        return Err(DiroError::InvalidResult("Division by zero".to_owned()));
                    }
                    Ok(lhs.calulate()? / r)
                }
                Verb::Modulo => Ok(lhs.calulate()? % rhs.calulate()?),
                Verb::Power => Ok(lhs.calulate()?.pow(rhs.calulate()? as u32)),
            },
            DiroAst::Dice(_, result) => {
                if let Some(r) = result {
                    Ok(r.result())
                } else {
                    Err(DiroError::DiceNotRolled)
                }
            }
            DiroAst::Closed(ast) => Ok(ast.calulate()?),
        }
    }

    pub fn expr(&self) -> String {
        self.expr_with_priority(1)
    }

    fn expr_with_priority(&self, priority: u8) -> String {
        match self {
            DiroAst::Int(i) => i.to_string(),
            DiroAst::DyadicOP { verb, lhs, rhs } => format!(
                "{}{}{}",
                lhs.expr_with_priority(verb.priority()),
                verb,
                rhs.expr_with_priority(verb.priority())
            ),
            DiroAst::Dice(dice, ..) => dice.expr(),
            DiroAst::Closed(ast) => {
                if let Self::DyadicOP { verb, .. } = ast.as_ref() {
                    if verb.priority() < priority {
                        format!("({})", ast.expr_with_priority(priority))
                    } else {
                        ast.expr_with_priority(priority)
                    }
                } else {
                    ast.expr()
                }
            }
        }
    }

    pub fn s_expr(&self) -> String {
        match self {
            DiroAst::Int(i) => i.to_string(),
            DiroAst::DyadicOP { verb, lhs, rhs } => {
                format!("({} {} {})", verb, lhs.s_expr(), rhs.s_expr())
            }
            DiroAst::Dice(dice, ..) => dice.expr(),
            DiroAst::Closed(ast) => ast.s_expr(),
        }
    }

    pub fn dyadic_with_priority(verb: Verb, lhs: DiroAst, rhs: DiroAst) -> DiroAst {
        if let DiroAst::DyadicOP {
            verb: sub_verb,
            lhs: sub_lhs,
            rhs: sub_rhs,
        } = rhs.clone()
        {
            if verb.priority() > sub_verb.priority() {
                return DiroAst::DyadicOP {
                    verb: sub_verb,
                    lhs: Box::new(DiroAst::DyadicOP {
                        verb,
                        lhs: Box::new(lhs),
                        rhs: sub_lhs,
                    }),
                    rhs: sub_rhs,
                };
            }
        }
        DiroAst::DyadicOP {
            verb,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

impl Verb {
    pub fn expr(&self) -> String {
        match self {
            Verb::Plus => "+".to_string(),
            Verb::Minus => "-".to_string(),
            Verb::Times => "*".to_string(),
            Verb::Divide => "/".to_string(),
            Verb::Modulo => "%".to_string(),
            Verb::Power => "^".to_string(),
        }
    }

    fn priority(&self) -> u8 {
        match self {
            Verb::Plus | Verb::Minus => 1,
            Verb::Times | Verb::Divide | Verb::Modulo => 2,
            Verb::Power => 3,
        }
    }
}

impl Display for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr())
    }
}
