mod matcher;
mod sheet;
mod svg;
mod table;

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
    m.add_class::<matcher::RangePattern1D>()?;
    m.add_class::<matcher::RangeMatcher>()?;
    m.add_class::<matcher::Range>()?;
    m.add_class::<table::Table>()?;
    Ok(())
}
