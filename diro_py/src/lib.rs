use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn parse(source: &str) -> PyResult<Diro> {
    diro::parse(source)
        .map(|ast| Diro(ast))
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyclass]
struct Diro(diro::DiroAst);

#[pymethods]
impl Diro {
    #[new]
    fn new() -> Self {
        Self(diro::DiroAst::Dice(diro::Dice::default()))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.expr())
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
    // rust-analyzer issue: https://github.com/rust-analyzer/rust-analyzer/issues/9606
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
