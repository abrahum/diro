use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

mod dice;

#[pyclass]
struct Diro(diro::DiroAst);

#[pymethods]
impl Diro {
    fn __repr__(&self) -> String {
        self.0.expr()
    }

    fn eval(&mut self) -> PyResult<i32> {
        self.0
            .eval()
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymodule]
#[pyo3(name = "diropy")]
fn diro_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Diro>()?;
    m.add_class::<dice::Dice>()?;
    m.add_class::<dice::RollResult>()?;
    // rust-analyzer issue: https://github.com/rust-analyzer/rust-analyzer/issues/9606
    // m.add_function(wrap_pyfunction!(parse, m)?)?;
    #[pyfn(m)]
    fn parse(source: &str) -> PyResult<Diro> {
        diro::parse(source)
            .map(|ast| Diro(ast))
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    Ok(())
}
