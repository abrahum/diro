#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiroAst {
    Int(i32),
    Dice(crate::Dice),
    DyadicOP {
        verb: Verb,
        lhs: Box<DiroAst>,
        rhs: Box<DiroAst>,
    },
    Closed(Box<DiroAst>),
    Empty,
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
    pub fn eval(&mut self) -> i32 {
        match self {
            DiroAst::Int(i) => *i,
            DiroAst::DyadicOP { verb, lhs, rhs } => match verb {
                Verb::Plus => lhs.eval() + rhs.eval(),
                Verb::Minus => lhs.eval() - rhs.eval(),
                Verb::Times => lhs.eval() * rhs.eval(),
                Verb::Divide => lhs.eval() / rhs.eval(),
                Verb::Modulo => lhs.eval() % rhs.eval(),
                Verb::Power => lhs.eval().pow(rhs.eval() as u32),
            },
            DiroAst::Dice(dice) => dice.roll_and_get(),
            DiroAst::Closed(ast) => ast.eval(),
            DiroAst::Empty => 0,
        }
    }

    pub fn expr(&self) -> String {
        match self {
            DiroAst::Int(i) => i.to_string(),
            DiroAst::DyadicOP { verb, lhs, rhs } => {
                format!("({}{}{})", lhs.expr(), verb.expr(), rhs.expr())
            }
            DiroAst::Dice(dice) => dice.expr(),
            DiroAst::Closed(ast) => ast.expr(),
            DiroAst::Empty => "".to_string(),
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
            Verb::Plus => 1,
            Verb::Minus => 1,
            Verb::Times => 2,
            Verb::Divide => 2,
            Verb::Modulo => 2,
            Verb::Power => 3,
        }
    }
}
