mod sheet;

use pyo3::prelude::*;

#[pyfunction]
fn load_workbook(path: &str) -> PyResult<sheet::Workbook> {
    sheet::load_workbook_impl(path)
}

#[pyfunction]
fn index_to_a1(row: isize, col: isize) -> PyResult<String> {
    sheet::index_to_a1(row, col)
}

#[pyfunction]
fn a1_to_index(a1: &str) -> PyResult<(usize, usize)> {
    sheet::a1_to_index(a1)
}

#[pymodule]
fn _tabularix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_workbook, m)?)?;
    m.add_function(wrap_pyfunction!(index_to_a1, m)?)?;
    m.add_function(wrap_pyfunction!(a1_to_index, m)?)?;
    m.add_class::<sheet::Sheet>()?;
    m.add_class::<sheet::Workbook>()?;
    Ok(())
}
