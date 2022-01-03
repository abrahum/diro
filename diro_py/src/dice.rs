use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub(crate) struct Dice(diro::Dice);

#[pymethods]
impl Dice {
    #[new]
    #[args(count = "1", face = "100", bp = "0", kq = "0")]
    fn new(count: u8, face: u16, bp: i8, kq: i8) -> PyResult<Self> {
        match diro::Dice::new(count, face, bp, kq) {
            Ok(dice) => Ok(Dice(dice)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    fn roll(&mut self) -> RollResult {
        RollResult(self.0.roll())
    }

    fn __repr__(&self) -> String {
        format!("{}", self.0.expr())
    }
}

#[pyclass]
pub(crate) struct RollResult(diro::RollResult);

#[pymethods]
impl RollResult {
    fn __call__(&self) -> i32 {
        self.0.result()
    }

    fn detail(&self) -> String {
        self.0.detail()
    }

    fn result(&self) -> (Vec<i32>, Vec<i32>, i32) {
        match &self.0 {
            diro::RollResult::D100 {
                result,
                bp_result,
                bp,
            } => (
                result.iter().map(|i| *i as i32).collect(),
                bp_result.iter().map(|i| *i as i32).collect(),
                if *bp {
                    bp_result.len() as i32
                } else {
                    0 - bp_result.len() as i32
                },
            ),
            diro::RollResult::Other { kq, result } => (
                result.iter().map(|i| *i as i32).collect(),
                vec![],
                *kq as i32,
            ),
        }
    }
}
