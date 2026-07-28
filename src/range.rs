use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass(from_py_object)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Range {
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
impl Range {
    #[new]
    #[must_use]
    pub fn new(start_row: usize, end_row: usize, start_col: usize, end_col: usize) -> Self {
        Range {
            start_row,
            end_row,
            start_col,
            end_col,
        }
    }

    fn __repr__(&self) -> String {
        let a1_notation = if self.start_row == self.end_row && self.start_col == self.end_col {
            format!(
                "{}{}",
                crate::sheet::index_to_col_letters(self.start_col),
                self.start_row + 1
            )
        } else {
            format!(
                "{}{}:{}{}",
                crate::sheet::index_to_col_letters(self.start_col),
                self.start_row + 1,
                crate::sheet::index_to_col_letters(self.end_col),
                self.end_row + 1
            )
        };
        format!(
            "Range({}, cols={}..{}, rows={}..{})",
            a1_notation, self.start_col, self.end_col, self.start_row, self.end_row
        )
    }

    fn __str__(&self) -> String {
        if self.start_row == self.end_row && self.start_col == self.end_col {
            format!(
                "{}{}",
                crate::sheet::index_to_col_letters(self.start_col),
                self.start_row + 1
            )
        } else {
            format!(
                "{}{}:{}{}",
                crate::sheet::index_to_col_letters(self.start_col),
                self.start_row + 1,
                crate::sheet::index_to_col_letters(self.end_col),
                self.end_row + 1
            )
        }
    }

    #[staticmethod]
    #[allow(clippy::missing_errors_doc)]
    pub fn from_a1(a1_str: &str) -> PyResult<Self> {
        fn col_letters_to_index(col_str: &str) -> Option<usize> {
            if col_str.is_empty() {
                return None;
            }
            let mut index: usize = 0;
            for c in col_str.chars() {
                if !c.is_ascii_alphabetic() {
                    return None;
                }
                let val = (c.to_ascii_uppercase() as u8 - b'A') as usize;
                index = index.checked_mul(26)?.checked_add(val + 1)?;
            }
            index.checked_sub(1)
        }

        fn parse_a1_cell(cell_str: &str) -> Option<(usize, usize)> {
            let letters: String = cell_str
                .chars()
                .take_while(char::is_ascii_alphabetic)
                .collect();
            let numbers: String = cell_str.chars().skip(letters.len()).collect();

            if letters.is_empty() || numbers.is_empty() {
                return None;
            }

            let col = col_letters_to_index(&letters)?;
            let row = numbers.parse::<usize>().ok()?.checked_sub(1)?;
            Some((row, col))
        }

        let parts: Vec<&str> = a1_str.split(':').collect();
        if parts.is_empty() || parts.len() > 2 {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Invalid A1 range format: '{a1_str}'"
            )));
        }

        if parts.len() == 1 {
            let cell = parts[0].trim();
            if let Some((row, col)) = parse_a1_cell(cell) {
                Ok(Range {
                    start_row: row,
                    end_row: row,
                    start_col: col,
                    end_col: col,
                })
            } else {
                Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Invalid A1 cell format: '{cell}'"
                )))
            }
        } else {
            let start = parts[0].trim();
            let end = parts[1].trim();
            if let (Some((s_row, s_col)), Some((e_row, e_col))) =
                (parse_a1_cell(start), parse_a1_cell(end))
            {
                let start_row = std::cmp::min(s_row, e_row);
                let end_row = std::cmp::max(s_row, e_row);
                let start_col = std::cmp::min(s_col, e_col);
                let end_col = std::cmp::max(s_col, e_col);

                Ok(Range {
                    start_row,
                    end_row,
                    start_col,
                    end_col,
                })
            } else {
                Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Invalid A1 range format: '{start}:{end}'"
                )))
            }
        }
    }
}

impl Range {
    /// Checks if this range intersects with another range in 2D grid space.
    #[must_use]
    pub fn intersects(&self, other: &Range) -> bool {
        std::cmp::max(self.start_row, other.start_row) <= std::cmp::min(self.end_row, other.end_row)
            && std::cmp::max(self.start_col, other.start_col)
                <= std::cmp::min(self.end_col, other.end_col)
    }
}

/// Parses an optional Python Range specification (`None`, single `Range`, single A1 `str`, or `list[Range | str]`).
///
/// # Errors
/// Returns `PyTypeError` or `PyValueError` if range specification or format is invalid.
pub fn parse_range_spec(obj: Option<Bound<'_, PyAny>>) -> PyResult<Vec<Range>> {
    let Some(bound) = obj else {
        return Ok(Vec::new());
    };

    // Case 1: Single Range object
    if let Ok(r) = bound.extract::<Range>() {
        return Ok(vec![r]);
    }

    // Case 2: Single A1 notation string
    if let Ok(s) = bound.extract::<String>() {
        return Ok(vec![Range::from_a1(&s)?]);
    }

    // Case 3: List / Sequence of Range objects or A1 strings
    if let Ok(sequence) = bound.extract::<Vec<Bound<'_, PyAny>>>() {
        let mut ranges = Vec::with_capacity(sequence.len());
        for elem in sequence {
            if let Ok(r) = elem.extract::<Range>() {
                ranges.push(r);
            } else if let Ok(s) = elem.extract::<String>() {
                ranges.push(Range::from_a1(&s)?);
            } else {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "Elements in range list must be Range objects or A1 strings (e.g. 'A1:C5')",
                ));
            }
        }
        return Ok(ranges);
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected a Range object, an A1 string (e.g. 'A1:C5'), or a list of Range objects/A1 strings",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_from_a1() {
        let r = Range::from_a1("B2").unwrap();
        assert_eq!(r.start_row, 1);
        assert_eq!(r.end_row, 1);
        assert_eq!(r.start_col, 1);
        assert_eq!(r.end_col, 1);

        let r = Range::from_a1("B2:D6").unwrap();
        assert_eq!(r.start_row, 1);
        assert_eq!(r.end_row, 5);
        assert_eq!(r.start_col, 1);
        assert_eq!(r.end_col, 3);

        let r = Range::from_a1("D6:B2").unwrap();
        assert_eq!(r.start_row, 1);
        assert_eq!(r.end_row, 5);
        assert_eq!(r.start_col, 1);
        assert_eq!(r.end_col, 3);

        let r = Range::from_a1(" B2 : D6 ").unwrap();
        assert_eq!(r.start_row, 1);
        assert_eq!(r.end_row, 5);

        let r = Range::from_a1("AA1").unwrap();
        assert_eq!(r.start_col, 26);

        assert!(Range::from_a1("A:B").is_err());
        assert!(Range::from_a1("1:2").is_err());
        assert!(Range::from_a1("A").is_err());
        assert!(Range::from_a1("1").is_err());
        assert!(Range::from_a1("A1:B").is_err());
        assert!(Range::from_a1("A:B2").is_err());
        assert!(Range::from_a1("A0").is_err());
        assert!(Range::from_a1("").is_err());
        assert!(Range::from_a1("A-1").is_err());
        assert!(Range::from_a1("A1:").is_err());
        assert!(Range::from_a1(":A1").is_err());
        assert!(Range::from_a1("A1:B2:C3").is_err());
        assert!(Range::from_a1("A 1").is_err());
    }

    #[test]
    fn test_range_intersects() {
        let r1 = Range::new(0, 4, 0, 2); // A1:C5
        let r2 = Range::new(3, 4, 0, 1); // A4:B5
        let r3 = Range::new(10, 12, 10, 12); // K11:M13

        assert!(r1.intersects(&r2));
        assert!(r2.intersects(&r1));
        assert!(!r1.intersects(&r3));
        assert!(!r3.intersects(&r1));
    }

    #[test]
    fn test_parse_range_spec() {
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            // 1. None
            let ranges = parse_range_spec(None).unwrap();
            assert!(ranges.is_empty());

            // 2. Single Range object
            let r_obj = Range::new(0, 2, 0, 2).into_pyobject(py).unwrap();
            let ranges = parse_range_spec(Some(r_obj.into_any())).unwrap();
            assert_eq!(ranges.len(), 1);
            assert_eq!(ranges[0], Range::new(0, 2, 0, 2));

            // 3. Single A1 string
            let str_obj = pyo3::types::PyString::new(py, "B2:D6")
                .into_pyobject(py)
                .unwrap();
            let ranges = parse_range_spec(Some(str_obj.into_any())).unwrap();
            assert_eq!(ranges.len(), 1);
            assert_eq!(ranges[0], Range::new(1, 5, 1, 3));

            // 4. List of Range and str
            let r1 = Range::new(0, 1, 0, 1).into_pyobject(py).unwrap();
            let s2 = pyo3::types::PyString::new(py, "C3:D4")
                .into_pyobject(py)
                .unwrap();
            let py_list = pyo3::types::PyList::new(py, vec![r1.into_any(), s2.into_any()]).unwrap();
            let ranges = parse_range_spec(Some(py_list.into_any())).unwrap();
            assert_eq!(ranges.len(), 2);
            assert_eq!(ranges[0], Range::new(0, 1, 0, 1));
            assert_eq!(ranges[1], Range::new(2, 3, 2, 3));

            // 5. Invalid element in list -> PyTypeError
            let num_obj = 123i32.into_pyobject(py).unwrap();
            let err_list = pyo3::types::PyList::new(py, vec![num_obj.into_any()]).unwrap();
            assert!(parse_range_spec(Some(err_list.into_any())).is_err());

            // 6. Invalid top-level type -> PyTypeError
            let invalid_type = 456i32.into_pyobject(py).unwrap();
            assert!(parse_range_spec(Some(invalid_type.into_any())).is_err());
        });
    }
}
