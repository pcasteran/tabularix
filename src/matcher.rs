use crate::sheet::CellValue;
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[derive(Debug)]
pub enum CellMatchRule {
    Any,
    Empty,
    NonEmpty,
    Exact(String),
    Regex(Py<PyAny>),
}

impl Clone for CellMatchRule {
    fn clone(&self) -> Self {
        match self {
            CellMatchRule::Any => CellMatchRule::Any,
            CellMatchRule::Empty => CellMatchRule::Empty,
            CellMatchRule::NonEmpty => CellMatchRule::NonEmpty,
            CellMatchRule::Exact(s) => CellMatchRule::Exact(s.clone()),
            CellMatchRule::Regex(r) => {
                let mut cloned = None;
                pyo3::Python::initialize();
                pyo3::Python::attach(|py| {
                    cloned = Some(r.clone_ref(py));
                });
                CellMatchRule::Regex(cloned.unwrap())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CellPattern {
    pub rule: CellMatchRule,
    pub min: usize,
    pub max: Option<usize>,
}

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct RowPattern {
    pub cell_patterns: Vec<CellPattern>,
    pub cardinality: String,
}

#[pymethods]
impl RowPattern {
    #[new]
    pub fn new() -> Self {
        RowPattern {
            cell_patterns: Vec::new(),
            cardinality: "1".to_string(),
        }
    }

    pub fn value(mut slf: PyRefMut<'_, Self>, val: String) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::Exact(val),
            min: 1,
            max: Some(1),
        });
        slf
    }

    pub fn regex<'py>(
        mut slf: PyRefMut<'py, Self>,
        py: Python<'py>,
        pattern: &Bound<'py, PyAny>,
    ) -> PyResult<PyRefMut<'py, Self>> {
        let compiled = if pattern.is_instance_of::<pyo3::types::PyString>() {
            let re = py.import("re")?;
            re.call_method1("compile", (pattern,))?.unbind()
        } else {
            pattern.clone().unbind()
        };
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::Regex(compiled),
            min: 1,
            max: Some(1),
        });
        Ok(slf)
    }

    pub fn empty(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::Empty,
            min: 1,
            max: Some(1),
        });
        slf
    }

    pub fn non_empty(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::NonEmpty,
            min: 1,
            max: Some(1),
        });
        slf
    }

    pub fn any(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::Any,
            min: 1,
            max: Some(1),
        });
        slf
    }

    pub fn one_or_more(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        enforce_cell_exclusivity(&mut slf.cell_patterns)?;
        if let Some(last) = slf.cell_patterns.last_mut() {
            last.min = 1;
            last.max = None;
        }
        Ok(slf)
    }

    pub fn zero_or_more(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        enforce_cell_exclusivity(&mut slf.cell_patterns)?;
        if let Some(last) = slf.cell_patterns.last_mut() {
            last.min = 0;
            last.max = None;
        }
        Ok(slf)
    }

    pub fn optional(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        enforce_cell_exclusivity(&mut slf.cell_patterns)?;
        if let Some(last) = slf.cell_patterns.last_mut() {
            last.min = 0;
            last.max = Some(1);
        }
        Ok(slf)
    }

    #[pyo3(signature = (min, max = Some(-1)))]
    pub fn repeat(
        mut slf: PyRefMut<'_, Self>,
        min: usize,
        max: Option<isize>,
    ) -> PyResult<PyRefMut<'_, Self>> {
        enforce_cell_exclusivity(&mut slf.cell_patterns)?;
        let parsed_max = match max {
            Some(-1) => Some(min),
            Some(m) if m >= 0 => Some(m.unsigned_abs()),
            _ => None,
        };
        if let Some(last) = slf.cell_patterns.last_mut() {
            last.min = min;
            last.max = parsed_max;
        }
        Ok(slf)
    }
}

fn enforce_cell_exclusivity(cell_patterns: &mut [CellPattern]) -> PyResult<()> {
    if let Some(last) = cell_patterns.last_mut() {
        if last.min != 1 || last.max != Some(1) {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Cannot set multiple cardinalities on the same cell pattern",
            ));
        }
    } else {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "No cell pattern defined to apply cardinality to",
        ));
    }
    Ok(())
}

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct RowGroup {
    #[pyo3(get)]
    pub start_row: usize,
    #[pyo3(get)]
    pub end_row: usize,
    #[pyo3(get)]
    pub start_col: usize,
    #[pyo3(get)]
    pub end_col: usize,
}

#[pymethods]
impl RowGroup {
    #[new]
    pub fn new(start_row: usize, end_row: usize, start_col: usize, end_col: usize) -> Self {
        RowGroup {
            start_row,
            end_row,
            start_col,
            end_col,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "<RowGroup rows={}..{}, cols={}..{}>",
            self.start_row, self.end_row, self.start_col, self.end_col
        )
    }
}

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct RowGroupMatcher {
    pub row_patterns: Vec<RowPattern>,
}

#[pymethods]
impl RowGroupMatcher {
    #[new]
    pub fn new() -> Self {
        RowGroupMatcher {
            row_patterns: Vec::new(),
        }
    }

    pub fn row(mut slf: PyRefMut<'_, Self>, pattern: RowPattern) -> PyRefMut<'_, Self> {
        slf.row_patterns.push(pattern);
        slf
    }

    pub fn one_or_more(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        enforce_row_exclusivity(&mut slf.row_patterns)?;
        if let Some(last) = slf.row_patterns.last_mut() {
            last.cardinality = "+".to_string();
        }
        Ok(slf)
    }

    pub fn zero_or_more(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        enforce_row_exclusivity(&mut slf.row_patterns)?;
        if let Some(last) = slf.row_patterns.last_mut() {
            last.cardinality = "*".to_string();
        }
        Ok(slf)
    }

    pub fn optional(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        enforce_row_exclusivity(&mut slf.row_patterns)?;
        if let Some(last) = slf.row_patterns.last_mut() {
            last.cardinality = "?".to_string();
        }
        Ok(slf)
    }

    #[pyo3(signature = (min, max = Some(-1)))]
    pub fn repeat(
        mut slf: PyRefMut<'_, Self>,
        min: usize,
        max: Option<isize>,
    ) -> PyResult<PyRefMut<'_, Self>> {
        enforce_row_exclusivity(&mut slf.row_patterns)?;
        let card = match max {
            Some(-1) => format!("{min}"),
            Some(m) if m >= 0 => format!("{{{min},{}}}", m.unsigned_abs()),
            _ => format!("{{{min},}}"),
        };
        if let Some(last) = slf.row_patterns.last_mut() {
            last.cardinality = card;
        }
        Ok(slf)
    }

    pub fn matches_row_group(
        &self,
        py: Python<'_>,
        rows: Vec<Vec<Bound<'_, PyAny>>>,
    ) -> PyResult<bool> {
        let sheet_data: Vec<Vec<CellValue>> = rows
            .into_iter()
            .map(|r| r.into_iter().map(|c| py_any_to_cell_value(&c)).collect())
            .collect();

        let matched_end = match_row_group(py, &self.row_patterns, &sheet_data, 0, 0)?;
        Ok(matched_end.is_some() && matched_end.unwrap() == sheet_data.len())
    }
}

fn enforce_row_exclusivity(row_patterns: &mut [RowPattern]) -> PyResult<()> {
    if let Some(last) = row_patterns.last_mut() {
        if last.cardinality != "1" {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Cannot set multiple cardinalities on the same row pattern",
            ));
        }
    } else {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "No row pattern defined to apply cardinality to",
        ));
    }
    Ok(())
}

fn py_any_to_cell_value(val: &Bound<'_, PyAny>) -> CellValue {
    if val.is_none() {
        CellValue::Empty
    } else if let Ok(b) = val.cast::<pyo3::types::PyBool>() {
        CellValue::Bool(b.is_true())
    } else if let Ok(i) = val.extract::<i64>() {
        CellValue::Int(i)
    } else if let Ok(f) = val.extract::<f64>() {
        CellValue::Float(f)
    } else if let Ok(s) = val.extract::<String>() {
        CellValue::String(s)
    } else {
        CellValue::Error(val.to_string())
    }
}

fn cell_matches_rule(py: Python<'_>, rule: &CellMatchRule, val: &CellValue) -> PyResult<bool> {
    match rule {
        CellMatchRule::Any => Ok(true),
        CellMatchRule::Empty => Ok(matches!(val, CellValue::Empty)),
        CellMatchRule::NonEmpty => Ok(!matches!(val, CellValue::Empty)),
        CellMatchRule::Exact(expected) => {
            let s = val.to_string_for_search();
            Ok(s == *expected)
        }
        CellMatchRule::Regex(py_regex) => {
            let s = val.to_string_for_search();
            let bound_regex = py_regex.bind(py);
            let match_obj = bound_regex.call_method1("search", (s,))?;
            Ok(!match_obj.is_none())
        }
    }
}

fn match_cells(
    py: Python<'_>,
    patterns: &[CellPattern],
    cells: &[CellValue],
    pattern_idx: usize,
    cell_idx: usize,
) -> PyResult<bool> {
    if pattern_idx == patterns.len() {
        return Ok(cell_idx == cells.len());
    }

    let pattern = &patterns[pattern_idx];
    let max_limit = match pattern.max {
        Some(m) => std::cmp::min(m, cells.len() - cell_idx),
        None => cells.len() - cell_idx,
    };

    if max_limit < pattern.min {
        return Ok(false);
    }

    let mut matchable = 0;
    while matchable < max_limit {
        if cell_matches_rule(py, &pattern.rule, &cells[cell_idx + matchable])? {
            matchable += 1;
        } else {
            break;
        }
    }

    if matchable < pattern.min {
        return Ok(false);
    }

    for k in (pattern.min..=matchable).rev() {
        if match_cells(py, patterns, cells, pattern_idx + 1, cell_idx + k)? {
            return Ok(true);
        }
    }

    Ok(false)
}

fn parse_cardinality(card: &str) -> (usize, Option<usize>) {
    match card {
        "1" => (1, Some(1)),
        "+" => (1, None),
        "*" => (0, None),
        "?" => (0, Some(1)),
        other => {
            if other.starts_with('{') && other.ends_with('}') {
                let content = &other[1..other.len() - 1];
                let parts: Vec<&str> = content.split(',').collect();
                if parts.len() == 1 {
                    if let Ok(num) = parts[0].parse::<usize>() {
                        return (num, Some(num));
                    }
                } else if parts.len() == 2 {
                    let min = parts[0].parse::<usize>().unwrap_or(0);
                    let max = if parts[1].is_empty() {
                        None
                    } else {
                        parts[1].parse::<usize>().ok()
                    };
                    return (min, max);
                }
            }
            (1, Some(1))
        }
    }
}

fn matches_row_pattern(py: Python<'_>, pattern: &RowPattern, row: &[CellValue]) -> PyResult<bool> {
    match_cells(py, &pattern.cell_patterns, row, 0, 0)
}

pub fn match_row_group(
    py: Python<'_>,
    patterns: &[RowPattern],
    sheet_data: &[Vec<CellValue>],
    pattern_idx: usize,
    sheet_row_idx: usize,
) -> PyResult<Option<usize>> {
    if pattern_idx == patterns.len() {
        return Ok(Some(sheet_row_idx));
    }

    let pattern = &patterns[pattern_idx];
    let (min_rows, max_rows) = parse_cardinality(&pattern.cardinality);

    let max_limit = match max_rows {
        Some(m) => std::cmp::min(m, sheet_data.len() - sheet_row_idx),
        None => sheet_data.len() - sheet_row_idx,
    };

    if max_limit < min_rows {
        return Ok(None);
    }

    let mut matchable = 0;
    while matchable < max_limit {
        if matches_row_pattern(py, pattern, &sheet_data[sheet_row_idx + matchable])? {
            matchable += 1;
        } else {
            break;
        }
    }

    if matchable < min_rows {
        return Ok(None);
    }

    for k in (min_rows..=matchable).rev() {
        if let Some(end_idx) =
            match_row_group(py, patterns, sheet_data, pattern_idx + 1, sheet_row_idx + k)?
        {
            return Ok(Some(end_idx));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cardinality() {
        assert_eq!(parse_cardinality("1"), (1, Some(1)));
        assert_eq!(parse_cardinality("+"), (1, None));
        assert_eq!(parse_cardinality("*"), (0, None));
        assert_eq!(parse_cardinality("?"), (0, Some(1)));
        assert_eq!(parse_cardinality("{3}"), (3, Some(3)));
        assert_eq!(parse_cardinality("{2,5}"), (2, Some(5)));
        assert_eq!(parse_cardinality("{4,}"), (4, None));
    }

    #[test]
    fn test_backtracking_matcher_exact_and_non_empty() {
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            let pattern = pyo3::Bound::new(py, RowPattern::new()).unwrap();
            {
                let mut p = pattern.borrow_mut();
                p = RowPattern::value(p, "Date".to_string());
                p = RowPattern::non_empty(p);
                p = RowPattern::one_or_more(p).unwrap();
                let _ = RowPattern::value(p, "Total".to_string());
            }

            let pattern_ref = pattern.borrow();

            // Match success
            let row1 = vec![
                CellValue::String("Date".to_string()),
                CellValue::String("Description".to_string()),
                CellValue::Int(123),
                CellValue::String("Total".to_string()),
            ];
            assert!(matches_row_pattern(py, &pattern_ref, &row1).unwrap());

            // Match failure (no non-empty cells)
            let row2 = vec![
                CellValue::String("Date".to_string()),
                CellValue::String("Total".to_string()),
            ];
            assert!(!matches_row_pattern(py, &pattern_ref, &row2).unwrap());

            // Match failure (wrong value at end)
            let row3 = vec![
                CellValue::String("Date".to_string()),
                CellValue::String("Description".to_string()),
                CellValue::Int(123),
            ];
            assert!(!matches_row_pattern(py, &pattern_ref, &row3).unwrap());
        });
    }

    #[test]
    fn test_backtracking_matcher_regex() {
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            let pattern = pyo3::Bound::new(py, RowPattern::new()).unwrap();
            {
                let mut p = pattern.borrow_mut();
                let re = py.import("re").unwrap();
                let re_pattern = re.call_method1("compile", ("^Q[1-4]$",)).unwrap();

                p = RowPattern::regex(p, py, &re_pattern).unwrap();
                p = RowPattern::repeat(p, 2, Some(2)).unwrap();
                p = RowPattern::empty(p);
                let _ = RowPattern::zero_or_more(p).unwrap();
            }

            let pattern_ref = pattern.borrow();

            // Match success: exactly 2 Q1-4, followed by 0 empty
            let row1 = vec![
                CellValue::String("Q1".to_string()),
                CellValue::String("Q2".to_string()),
            ];
            assert!(matches_row_pattern(py, &pattern_ref, &row1).unwrap());

            // Match success: exactly 2 Q1-4, followed by 2 empty
            let row2 = vec![
                CellValue::String("Q1".to_string()),
                CellValue::String("Q2".to_string()),
                CellValue::Empty,
                CellValue::Empty,
            ];
            assert!(matches_row_pattern(py, &pattern_ref, &row2).unwrap());

            // Match failure: only 1 Q1-4
            let row3 = vec![CellValue::String("Q1".to_string()), CellValue::Empty];
            assert!(!matches_row_pattern(py, &pattern_ref, &row3).unwrap());
        });
    }

    #[test]
    fn test_exclusivity_rule() {
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            let pattern = pyo3::Bound::new(py, RowPattern::new()).unwrap();
            let mut p = pattern.borrow_mut();
            p = RowPattern::empty(p);
            p = RowPattern::optional(p).unwrap();

            // Try to set cardinality again
            let res = RowPattern::one_or_more(p);
            assert!(res.is_err());
            let err_msg = res.err().unwrap().to_string();
            assert!(err_msg.contains("Cannot set multiple cardinalities"));
        });
    }
}
