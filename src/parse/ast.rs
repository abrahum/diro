use std::fmt::Display;

use crate::{
    error::{DiroError, DiroResult},
    Dice,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiroAst {
    Int(i32),
    Dice(Dice),
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
        match self {
            DiroAst::Int(i) => Ok(*i),
            DiroAst::DyadicOP { verb, lhs, rhs } => match verb {
                Verb::Plus => Ok(lhs.eval()? + rhs.eval()?),
                Verb::Minus => Ok(lhs.eval()? - rhs.eval()?),
                Verb::Times => Ok(lhs.eval()? * rhs.eval()?),
                Verb::Divide => {
                    let r = rhs.eval()?;
                    if r == 0 {
                        return Err(DiroError::InvalidResult("Division by zero".to_owned()));
                    }
                    Ok(lhs.eval()? / r)
                }
                Verb::Modulo => Ok(lhs.eval()? % rhs.eval()?),
                Verb::Power => Ok(lhs.eval()?.pow(rhs.eval()? as u32)),
            },
            DiroAst::Dice(dice) => dice.roll().result(),
            DiroAst::Closed(ast) => Ok(ast.eval()?),
        }
    }

    pub fn expr(&self) -> String {
        self.expr_with_verb(&Verb::Plus)
    }

    fn expr_with_verb(&self, s_verb: &Verb) -> String {
        match self {
            DiroAst::Int(i) => i.to_string(),
            DiroAst::DyadicOP { verb, lhs, rhs } => format!(
                "{}{}{}",
                lhs.expr_with_verb(verb),
                verb,
                rhs.expr_with_verb(verb)
            ),
            DiroAst::Dice(dice) => dice.expr(),
            DiroAst::Closed(ast) => {
                if let Self::DyadicOP { verb, .. } = ast.as_ref() {
                    if verb.priority() < s_verb.priority() {
                        format!("({})", ast.expr_with_verb(s_verb))
                    } else {
                        ast.expr_with_verb(s_verb)
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
            DiroAst::Dice(dice) => dice.expr(),
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
