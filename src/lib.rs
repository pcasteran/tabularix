mod matcher;
mod sheet;

use pyo3::prelude::*;

#[pyfunction]
fn load_workbook(path: &str) -> PyResult<sheet::Workbook> {
    sheet::load_workbook_impl(path)
}

#[pymodule]
fn _tabularix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_workbook, m)?)?;
    m.add_class::<sheet::Sheet>()?;
    m.add_class::<sheet::Workbook>()?;
    m.add_class::<matcher::RowPattern>()?;
    m.add_class::<matcher::RangeMatcher>()?;
    m.add_class::<matcher::Range>()?;
    Ok(())
}
