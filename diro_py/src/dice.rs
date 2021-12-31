use pyo3::prelude::*;

#[pyclass]
pub(crate) struct Dice(diro::Dice);

#[pymethods]
impl Dice {
    #[new]
    #[args(count = "1", face = "100", bp = "0", kq = "0")]
    fn new(count: u8, face: u16, bp: i8, kq: i8) -> Self {
        Self(diro::Dice::new(count, face, bp, kq))
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
}
