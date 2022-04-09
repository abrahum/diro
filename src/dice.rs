use rand::Rng;
use std::fmt::Debug;

use crate::error::{DiroError, DiroResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dice {
    D100 {
        count: u8,
        bp: i8,
    },
    Dice {
        count: u8,
        face: u16,
        kq: i8,
    },
    ADice {
        count: u8,
        face: u16,
        add_line: u16,
        success_line: u16,
    },
    CDice {
        count: u8,
        face: u16,
        count_line: u16,
    },
    FDice(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RollResult {
    D100(Vec<([u8; 2], bool, Vec<u8>)>),
    Dice {
        kq: i8,
        result: Vec<i32>,
    },
    ADice {
        result: Vec<Vec<i32>>,
        add_line: u16,
        success_line: u16,
    },
    CDice {
        result: Vec<Vec<i32>>,
        count_line: u16,
    },
    FDice(Vec<i8>),
}

impl RollResult {
    pub fn detail(&self) -> String {
        match self {
            RollResult::D100(v) => v
                .iter()
                .map(|r| {
                    let mut s = String::new();
                    s.push_str(r.0[0].to_string().as_str());
                    s.push_str(r.0[1].to_string().as_str());
                    for i in r.2.iter() {
                        if r.1 {
                            s.push_str("B");
                        } else {
                            s.push_str("P");
                        }
                        s.push_str(i.to_string().as_str());
                    }
                    s
                })
                .collect::<Vec<_>>()
                .join("+"),
            RollResult::Dice { result, .. } => result
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join("+"),
            RollResult::ADice { result, .. } => result
                .iter()
                .enumerate()
                .map(|(round, r)| {
                    let mut s = String::new();
                    s.push_str(format!("[{}]:", round + 1).as_str());
                    s.push_str(
                        r.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                            .as_str(),
                    );
                    s
                })
                .collect::<Vec<_>>()
                .join(" "),
            RollResult::CDice { result, .. } => result
                .iter()
                .enumerate()
                .map(|(round, r)| {
                    let mut s = String::new();
                    s.push_str(format!("[{}]:", round + 1).as_str());
                    s.push_str(
                        r.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                            .as_str(),
                    );
                    s
                })
                .collect::<Vec<_>>()
                .join(" "),
            RollResult::FDice(f) => f
                .iter()
                .map(|x| {
                    if x > &0 {
                        "+"
                    } else if x == &0 {
                        "0"
                    } else {
                        "-"
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
        }
    }

    pub fn result(&self) -> i32 {
        match self {
            Self::D100(v) => v
                .iter()
                .map(|r| {
                    let mut h = r.0[0];
                    for j in r.2.iter() {
                        if r.1 && j < &h {
                            h = *j;
                        } else if !r.1 && j > &h {
                            h = *j;
                        }
                    }
                    (h * 10 + r.0[1]) as i32
                })
                .sum(),
            Self::Dice { kq, result } => match kq {
                0 => result.iter().sum(),
                1..=i8::MAX => {
                    let mut temp = result.clone();
                    temp.sort_unstable();
                    temp.iter().rev().take(*kq as usize).sum()
                }
                i8::MIN..=-1 => {
                    let mut temp = result.clone();
                    temp.sort_unstable();
                    temp.iter().take(kq.abs() as usize).sum()
                }
            },
            Self::ADice {
                result,
                success_line,
                ..
            } => {
                let mut count = 0;
                for r in result {
                    for i in r {
                        if *i >= *success_line as i32 {
                            count += 1;
                        }
                    }
                }
                count
            }
            Self::CDice { result, count_line } => {
                let mut count = 0;
                for r in result {
                    for i in r {
                        if *i >= *count_line as i32 {
                            count += i;
                        }
                    }
                }
                count += result[result.len() - 1].iter().max().unwrap();
                count
            }
            Self::FDice(f) => f.iter().sum::<i8>() as i32,
        }
    }
}

impl Default for Dice {
    fn default() -> Self {
        Dice::D100 { count: 1, bp: 0 }
    }
}

impl Dice {
    pub fn d100(count: u8, bp: i8) -> DiroResult<Self> {
        Self::_dice(count, 100, bp, 0, 0)
    }

    pub fn dice(count: u8, face: u16, kq: i8) -> DiroResult<Self> {
        Self::_dice(count, face, 0, kq, 0)
    }

    pub fn _dice(count: u8, face: u16, bp: i8, kq: i8, a: u16) -> DiroResult<Self> {
        if count == 0 {
            Err(DiroError::NoDice)
        } else if a != 0 {
            Ok(Dice::ADice {
                count,
                face,
                add_line: face + 1,
                success_line: a,
            })
        } else if bp != 0 || face == 100 {
            Ok(Dice::D100 { count, bp })
        } else if kq.abs() as u8 <= count {
            Ok(Dice::Dice { count, face, kq })
        } else {
            Err(DiroError::KQTooBig)
        }
    }

    pub fn adice(count: u8, face: u16, success_line: u16, add_line: u16) -> DiroResult<Self> {
        if count == 0 {
            Err(DiroError::NoDice)
        } else {
            Ok(Dice::ADice {
                count,
                face,
                add_line,
                success_line,
            })
        }
    }

    pub fn cdice(count: u8, face: u16, count_line: u16) -> DiroResult<Self> {
        if count == 0 {
            Err(DiroError::NoDice)
        } else {
            Ok(Dice::CDice {
                count,
                face,
                count_line,
            })
        }
    }

    pub fn fdice(count: u8) -> DiroResult<Self> {
        if count == 0 {
            Err(DiroError::NoDice)
        } else {
            Ok(Dice::FDice(count))
        }
    }

    pub fn roll(&self) -> RollResult {
        fn range_9(rng: &mut impl Rng) -> u8 {
            rng.gen_range(0..9)
        }

        let mut rng = rand::thread_rng();
        match self {
            Self::D100 { count, bp } => {
                let mut r = vec![];
                for _ in 0..*count as usize {
                    let result = [range_9(&mut rng), range_9(&mut rng) + 1];
                    let mut bp_result = Vec::new();
                    for _ in 0..bp.abs() as usize {
                        bp_result.push(range_9(&mut rng));
                    }
                    r.push((result, bp > &0, bp_result));
                }
                RollResult::D100(r)
            }
            Self::Dice { count, face, kq } => {
                let mut result = Vec::new();
                for _ in 0..*count {
                    let t = rng.gen_range(1..=*face) as i32;
                    result.push(t);
                }
                RollResult::Dice { kq: *kq, result }
            }
            Self::ADice {
                count,
                face,
                add_line,
                success_line,
            } => {
                let mut result = vec![];
                let mut add = *count;

                loop {
                    let mut next = 0;
                    let mut v = vec![];
                    for _ in 0..add as usize {
                        let t = rng.gen_range(1..=*face) as i32;
                        v.push(t);
                        if t >= *add_line as i32 {
                            next += 1;
                        }
                    }
                    result.push(v);
                    add = next;
                    if add == 0 {
                        break;
                    }
                }

                RollResult::ADice {
                    result,
                    add_line: *add_line,
                    success_line: *success_line,
                }
            }
            Self::CDice {
                count,
                face,
                count_line,
            } => {
                let mut result = vec![];
                let mut add = *count;

                loop {
                    let mut next = 0;
                    let mut v = vec![];
                    for _ in 0..add as usize {
                        let t = rng.gen_range(1..=*face) as i32;
                        v.push(t);
                        if t >= *count_line as i32 {
                            next += 1;
                        }
                    }
                    result.push(v);
                    add = next;
                    if add == 0 {
                        break;
                    }
                }

                RollResult::CDice {
                    result,
                    count_line: *count_line,
                }
            }
            Self::FDice(count) => {
                let mut result = vec![];
                for _ in 0..*count {
                    result.push(rng.gen_range(-1..=1));
                }
                RollResult::FDice(result)
            }
        }
    }

    pub fn expr(&self) -> String {
        match self {
            Self::D100 { count, bp } => {
                let mut s = format!(
                    "{}D100",
                    if count > &1 {
                        count.to_string()
                    } else {
                        "".to_string()
                    }
                );
                if *bp > 0 {
                    s.push('B');
                    s.push_str(&bp.to_string());
                } else if *bp < 0 {
                    s.push('P');
                    s.push_str(&bp.abs().to_string());
                }
                s
            }
            Self::Dice { face, count, kq } => {
                let mut s = format!("{}D{}", count, face);
                if *kq > 0 {
                    s.push_str(&format!("K{}", kq));
                } else if *kq < 0 {
                    s.push_str(&format!("Q{}", kq.abs()));
                }
                s
            }
            Self::ADice {
                face,
                count,
                add_line,
                success_line,
            } => {
                let mut s = String::default();
                s.push_str(&format!("{}A{}", count, add_line));
                if success_line != &8 {
                    s.push_str(&format!("K{}", success_line));
                }
                if face != &10 {
                    s.push_str(&format!("D{}", face));
                }
                s
            }
            Self::CDice {
                count,
                face,
                count_line,
            } => format!("{}C{}M{}", count, count_line, face),
            Self::FDice(f) => format!("{}F3", f),
        }
    }
}

#[test]
fn dice_test() {
    let dices = [Dice::d100(1, 0).unwrap(), Dice::dice(4, 6, -2).unwrap()];
    for dice in dices.iter() {
        let r = dice.roll();
        println!("{}={}={:?}", dice.expr(), r.detail(), r.result());
        println!("{:?}", r);
    }
}
