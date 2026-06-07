use pyo3::prelude::*;

#[pyfunction]
fn hello_world() -> String {
    "Hello from Tabularix Rust Core!".to_string()
}

#[pymodule]
fn _tabularix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}
