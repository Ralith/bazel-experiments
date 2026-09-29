use pyo3::{Bound, PyResult, pymodule, types::PyModule};

#[pymodule]
fn _native(_: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
