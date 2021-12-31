use crate::error::{DiroError, DiroResult};
use rand::Rng;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dice {
    D100(i8),
    Other { face: u16, count: u8, kq: i8 },
}

#[derive(Debug)]
pub enum RollResult {
    D100 {
        bp: bool,
        result: [u8; 2],
        bp_result: Vec<u8>,
    },
    Other {
        kq: i8,
        result: Vec<i32>,
    },
}

impl RollResult {
    pub fn result(&self) -> DiroResult<i32> {
        match self {
            Self::D100 {
                result,
                bp_result,
                bp,
            } => {
                let [mut high, low] = result;
                if !bp_result.is_empty() {
                    if *bp {
                        let min = bp_result.iter().min().unwrap();
                        if min < &high {
                            high = *min;
                        }
                    } else {
                        let max = bp_result.iter().max().unwrap();
                        if max > &high {
                            high = *max;
                        }
                    }
                }
                if high == 0 && *low == 0 {
                    Ok(100)
                } else {
                    Ok((high * 10 + low) as i32)
                }
            }
            Self::Other { kq, result } => match kq {
                0 => Ok(result.iter().sum()),
                1..=i8::MAX => {
                    if result.len() < *kq as usize {
                        return Err(DiroError::KQTooBig);
                    }
                    let mut temp = result.clone();
                    temp.sort_unstable();
                    Ok(temp.iter().rev().take(*kq as usize).sum())
                }
                i8::MIN..=-1 => {
                    if result.len() < kq.abs() as usize {
                        return Err(DiroError::KQTooBig);
                    }
                    let mut temp = result.clone();
                    temp.sort_unstable();
                    Ok(temp.iter().take(kq.abs() as usize).sum())
                }
            },
        }
    }
}

impl Default for Dice {
    fn default() -> Self {
        Dice::D100(0)
    }
}

impl Dice {
    pub fn roll(&mut self) -> RollResult {
        fn range_9(rng: &mut impl Rng) -> u8 {
            rng.gen_range(0..9)
        }

        let mut rng = rand::thread_rng();
        match self {
            Self::Other { face, count, kq } => {
                let mut result = Vec::new();
                for _ in 0..*count {
                    let t = rng.gen_range(1..=*face) as i32;
                    result.push(t);
                }
                RollResult::Other { kq: *kq, result }
            }
            Self::D100(bp) => {
                let result = [range_9(&mut rng), range_9(&mut rng)];
                let mut bp_result = Vec::new();
                for _ in 0..bp.abs() as usize {
                    bp_result.push(range_9(&mut rng));
                }
                RollResult::D100 {
                    bp: *bp > 0,
                    result,
                    bp_result,
                }
            }
        }
    }

    pub fn expr(&self) -> String {
        match self {
            Self::Other {
                face, count, kq, ..
            } => {
                let mut s = format!("{}D{}", count, face);
                if *kq > 0 {
                    s.push_str(&format!("K{}", kq));
                } else if *kq < 0 {
                    s.push_str(&format!("Q{}", kq.abs()));
                }
                s
            }
            Self::D100(bp) => {
                let mut s = "D100".to_string();
                if *bp > 0 {
                    s.push('B');
                    s.push_str(&bp.to_string());
                } else if *bp < 0 {
                    s.push('P');
                    s.push_str(&bp.abs().to_string());
                }
                s
            }
        }
    }
}

#[test]
fn dice_test() {
    let mut dices = [
        Dice::D100(5),
        Dice::Other {
            face: 6,
            count: 4,
            kq: -2,
        },
    ];
    for dice in dices.iter_mut() {
        let r = dice.roll();
        println!("{}={:?}", dice.expr(), r.result());
        println!("{:?}", r);
    }
}
