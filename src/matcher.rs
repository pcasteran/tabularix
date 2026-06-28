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
    Group(RangePattern1D),
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
            CellMatchRule::Group(g) => CellMatchRule::Group(g.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CellPattern {
    pub rule: CellMatchRule,
    pub min: usize,
    pub max: Option<usize>,
    pub greedy: bool,
}

impl CellPattern {
    pub fn width_bounds(&self) -> (usize, Option<usize>) {
        if let CellMatchRule::Group(sub_group) = &self.rule {
            let (sub_min, sub_max) = sub_group.width_bounds();
            let min_w = self.min * sub_min;
            let max_w = match (self.max, sub_max) {
                (Some(max_reps), Some(max_sub_w)) => Some(max_reps * max_sub_w),
                _ => None,
            };
            (min_w, max_w)
        } else {
            let min_w = self.min;
            let max_w = self.max;
            (min_w, max_w)
        }
    }
}

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct RangePattern1D {
    pub cell_patterns: Vec<CellPattern>,
    #[pyo3(get, set)]
    pub cardinality: String,
    #[pyo3(get, set)]
    pub greedy: bool,
}

impl RangePattern1D {
    pub fn width_bounds(&self) -> (usize, Option<usize>) {
        let mut min_w = 0;
        let mut max_w = Some(0);
        for cp in &self.cell_patterns {
            let (cp_min, cp_max) = cp.width_bounds();
            min_w += cp_min;
            if let (Some(total), Some(max_val)) = (max_w, cp_max) {
                max_w = Some(total + max_val);
            } else {
                max_w = None;
            }
        }
        (min_w, max_w)
    }
}

#[pymethods]
impl RangePattern1D {
    #[new]
    pub fn new() -> Self {
        RangePattern1D {
            cell_patterns: Vec::new(),
            cardinality: "1".to_string(),
            greedy: true,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "RangePattern1D(cardinality='{}', greedy={}, cell_patterns={:?})",
            self.cardinality, self.greedy, self.cell_patterns
        )
    }

    pub fn value(mut slf: PyRefMut<'_, Self>, val: String) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::Exact(val),
            min: 1,
            max: Some(1),
            greedy: true,
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
            greedy: true,
        });
        Ok(slf)
    }

    pub fn empty(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::Empty,
            min: 1,
            max: Some(1),
            greedy: true,
        });
        slf
    }

    pub fn non_empty(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::NonEmpty,
            min: 1,
            max: Some(1),
            greedy: true,
        });
        slf
    }

    pub fn any(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::Any,
            min: 1,
            max: Some(1),
            greedy: true,
        });
        slf
    }

    pub fn group(mut slf: PyRefMut<'_, Self>, pattern: RangePattern1D) -> PyRefMut<'_, Self> {
        slf.cell_patterns.push(CellPattern {
            rule: CellMatchRule::Group(pattern),
            min: 1,
            max: Some(1),
            greedy: true,
        });
        slf
    }

    #[pyo3(signature = (greedy = true))]
    pub fn one_or_more(mut slf: PyRefMut<'_, Self>, greedy: bool) -> PyResult<PyRefMut<'_, Self>> {
        enforce_cell_exclusivity(&mut slf.cell_patterns)?;
        if let Some(last) = slf.cell_patterns.last_mut() {
            last.min = 1;
            last.max = None;
            last.greedy = greedy;
        }
        Ok(slf)
    }

    #[pyo3(signature = (greedy = true))]
    pub fn zero_or_more(mut slf: PyRefMut<'_, Self>, greedy: bool) -> PyResult<PyRefMut<'_, Self>> {
        enforce_cell_exclusivity(&mut slf.cell_patterns)?;
        if let Some(last) = slf.cell_patterns.last_mut() {
            last.min = 0;
            last.max = None;
            last.greedy = greedy;
        }
        Ok(slf)
    }

    #[pyo3(signature = (greedy = true))]
    pub fn optional(mut slf: PyRefMut<'_, Self>, greedy: bool) -> PyResult<PyRefMut<'_, Self>> {
        enforce_cell_exclusivity(&mut slf.cell_patterns)?;
        if let Some(last) = slf.cell_patterns.last_mut() {
            last.min = 0;
            last.max = Some(1);
            last.greedy = greedy;
        }
        Ok(slf)
    }

    #[pyo3(signature = (min, max = Some(-1), greedy = true))]
    pub fn repeat(
        mut slf: PyRefMut<'_, Self>,
        min: usize,
        max: Option<isize>,
        greedy: bool,
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
            last.greedy = greedy;
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

    #[staticmethod]
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

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct RangeMatcher {
    #[pyo3(get)]
    pub row_patterns: Vec<RangePattern1D>,
    #[pyo3(get)]
    pub outer_direction: String,
    #[pyo3(get)]
    pub inner_direction: String,
}

#[pymethods]
impl RangeMatcher {
    #[new]
    pub fn new(
        row_patterns: Vec<RangePattern1D>,
        outer_direction: String,
        inner_direction: String,
    ) -> PyResult<Self> {
        let outer_is_vert = matches!(outer_direction.as_str(), "TB" | "BT");
        let inner_is_horiz = matches!(inner_direction.as_str(), "LR" | "RL");
        let outer_is_horiz = matches!(outer_direction.as_str(), "LR" | "RL");
        let inner_is_vert = matches!(inner_direction.as_str(), "TB" | "BT");

        if !((outer_is_vert && inner_is_horiz) || (outer_is_horiz && inner_is_vert)) {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Directions must be orthogonal. Got outer_direction='{outer_direction}', inner_direction='{inner_direction}'"
            )));
        }

        Ok(RangeMatcher {
            row_patterns,
            outer_direction,
            inner_direction,
        })
    }

    pub fn matches_range(
        &self,
        py: Python<'_>,
        rows: Vec<Vec<Bound<'_, PyAny>>>,
    ) -> PyResult<bool> {
        let sheet_data: Vec<Vec<CellValue>> = rows
            .into_iter()
            .map(|r| r.into_iter().map(|c| py_any_to_cell_value(&c)).collect())
            .collect();

        // Orient the input grid to canonical direction
        let canon_data = orient_grid(&sheet_data, &self.outer_direction, &self.inner_direction);

        let cols_count = if canon_data.is_empty() {
            0
        } else {
            canon_data[0].len()
        };
        let col_end = if cols_count > 0 { cols_count - 1 } else { 0 };
        let matched_end = match_range(py, &self.row_patterns, &canon_data, 0, col_end, 0, 0)?;
        Ok(matched_end.is_some() && matched_end.unwrap() == canon_data.len())
    }
}

pub fn orient_grid(
    grid: &[Vec<CellValue>],
    outer_dir: &str,
    inner_dir: &str,
) -> Vec<Vec<CellValue>> {
    let needs_transpose = matches!(outer_dir, "LR" | "RL");

    let mut working = if needs_transpose {
        let r_count = grid.len();
        let c_count = if r_count > 0 { grid[0].len() } else { 0 };
        let mut transposed = vec![vec![CellValue::Empty; r_count]; c_count];
        for r in 0..r_count {
            for c in 0..c_count {
                transposed[c][r] = grid[r][c].clone();
            }
        }
        transposed
    } else {
        grid.to_vec()
    };

    let reverse_rows = if needs_transpose {
        outer_dir == "RL"
    } else {
        outer_dir == "BT"
    };

    if reverse_rows {
        working.reverse();
    }

    let reverse_cols = if needs_transpose {
        inner_dir == "BT"
    } else {
        inner_dir == "RL"
    };

    if reverse_cols {
        for row in &mut working {
            row.reverse();
        }
    }

    working
}

#[allow(clippy::too_many_arguments)]
pub fn map_canonical_range_back(
    canon_start_row: usize,
    canon_end_row: usize,
    canon_start_col: usize,
    canon_end_col: usize,
    orig_rows: usize,
    orig_cols: usize,
    outer_dir: &str,
    inner_dir: &str,
) -> (usize, usize, usize, usize) {
    let needs_transpose = matches!(outer_dir, "LR" | "RL");

    let reverse_rows = if needs_transpose {
        outer_dir == "RL"
    } else {
        outer_dir == "BT"
    };

    let reverse_cols = if needs_transpose {
        inner_dir == "BT"
    } else {
        inner_dir == "RL"
    };

    let canon_rows_limit = if needs_transpose {
        orig_cols
    } else {
        orig_rows
    };
    let canon_cols_limit = if needs_transpose {
        orig_rows
    } else {
        orig_cols
    };

    let (t_start_row, t_end_row) = if reverse_rows {
        (
            canon_rows_limit - 1 - canon_end_row,
            canon_rows_limit - 1 - canon_start_row,
        )
    } else {
        (canon_start_row, canon_end_row)
    };

    let (t_start_col, t_end_col) = if reverse_cols {
        (
            canon_cols_limit - 1 - canon_end_col,
            canon_cols_limit - 1 - canon_start_col,
        )
    } else {
        (canon_start_col, canon_end_col)
    };

    if needs_transpose {
        let start_row = t_start_col;
        let end_row = t_end_col;
        let start_col = t_start_row;
        let end_col = t_end_row;
        (start_row, end_row, start_col, end_col)
    } else {
        (t_start_row, t_end_row, t_start_col, t_end_col)
    }
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
        if let (Ok(year), Ok(month), Ok(day)) = (
            val.getattr("year").and_then(|a| a.extract::<i32>()),
            val.getattr("month").and_then(|a| a.extract::<u32>()),
            val.getattr("day").and_then(|a| a.extract::<u32>()),
        ) {
            if let Some(naive_date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                if let (Ok(hour), Ok(minute), Ok(second), Ok(microsecond)) = (
                    val.getattr("hour").and_then(|a| a.extract::<u32>()),
                    val.getattr("minute").and_then(|a| a.extract::<u32>()),
                    val.getattr("second").and_then(|a| a.extract::<u32>()),
                    val.getattr("microsecond").and_then(|a| a.extract::<u32>()),
                ) {
                    if let Some(naive_dt) =
                        naive_date.and_hms_micro_opt(hour, minute, second, microsecond)
                    {
                        return CellValue::DateTime(naive_dt);
                    }
                }
                return CellValue::Date(naive_date);
            }
        }
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
        CellMatchRule::Group(_) => Ok(false),
    }
}

struct GroupMatchCtx<'a, 'py> {
    py: Python<'py>,
    cells: &'a [CellValue],
}

fn match_group_reps(
    ctx: &GroupMatchCtx<'_, '_>,
    sub_patterns: &[CellPattern],
    outer_patterns: &[CellPattern],
    outer_pattern_idx: usize,
    cell_idx: usize,
    current_reps: usize,
) -> PyResult<bool> {
    let pattern = &outer_patterns[outer_pattern_idx];
    let min_reps = pattern.min;
    let max_reps = pattern.max;

    if pattern.greedy {
        let can_match_more = match max_reps {
            Some(max) => current_reps < max,
            None => true,
        };

        if can_match_more {
            let mut valid_ends = Vec::new();
            find_group_match_ends(ctx, sub_patterns, cell_idx, 0, &mut valid_ends)?;
            for next_cell_idx in valid_ends {
                if match_group_reps(
                    ctx,
                    sub_patterns,
                    outer_patterns,
                    outer_pattern_idx,
                    next_cell_idx,
                    current_reps + 1,
                )? {
                    return Ok(true);
                }
            }
        }

        if current_reps >= min_reps
            && match_cells(ctx, outer_patterns, outer_pattern_idx + 1, cell_idx)?
        {
            return Ok(true);
        }
    } else {
        if current_reps >= min_reps
            && match_cells(ctx, outer_patterns, outer_pattern_idx + 1, cell_idx)?
        {
            return Ok(true);
        }

        let can_match_more = match max_reps {
            Some(max) => current_reps < max,
            None => true,
        };

        if can_match_more {
            let mut valid_ends = Vec::new();
            find_group_match_ends(ctx, sub_patterns, cell_idx, 0, &mut valid_ends)?;
            for next_cell_idx in valid_ends {
                if match_group_reps(
                    ctx,
                    sub_patterns,
                    outer_patterns,
                    outer_pattern_idx,
                    next_cell_idx,
                    current_reps + 1,
                )? {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

fn find_group_match_ends(
    ctx: &GroupMatchCtx<'_, '_>,
    patterns: &[CellPattern],
    current_cell_idx: usize,
    pattern_idx: usize,
    results: &mut Vec<usize>,
) -> PyResult<()> {
    if pattern_idx == patterns.len() {
        results.push(current_cell_idx);
        return Ok(());
    }

    let pattern = &patterns[pattern_idx];

    if let CellMatchRule::Group(ref sub_group) = pattern.rule {
        let mut sub_ends = Vec::new();
        find_group_reps_ends(
            ctx,
            &sub_group.cell_patterns,
            pattern,
            current_cell_idx,
            0,
            &mut sub_ends,
        )?;
        for end in sub_ends {
            find_group_match_ends(ctx, patterns, end, pattern_idx + 1, results)?;
        }
    } else {
        let max_limit = match pattern.max {
            Some(m) => std::cmp::min(m, ctx.cells.len() - current_cell_idx),
            None => ctx.cells.len() - current_cell_idx,
        };

        if max_limit < pattern.min {
            return Ok(());
        }

        let mut matchable = 0;
        while matchable < max_limit {
            if cell_matches_rule(
                ctx.py,
                &pattern.rule,
                &ctx.cells[current_cell_idx + matchable],
            )? {
                matchable += 1;
            } else {
                break;
            }
        }

        if matchable < pattern.min {
            return Ok(());
        }

        if pattern.greedy {
            for k in (pattern.min..=matchable).rev() {
                find_group_match_ends(
                    ctx,
                    patterns,
                    current_cell_idx + k,
                    pattern_idx + 1,
                    results,
                )?;
            }
        } else {
            for k in pattern.min..=matchable {
                find_group_match_ends(
                    ctx,
                    patterns,
                    current_cell_idx + k,
                    pattern_idx + 1,
                    results,
                )?;
            }
        }
    }

    Ok(())
}

fn find_group_reps_ends(
    ctx: &GroupMatchCtx<'_, '_>,
    sub_patterns: &[CellPattern],
    outer_pattern: &CellPattern,
    current_cell_idx: usize,
    current_reps: usize,
    results: &mut Vec<usize>,
) -> PyResult<()> {
    let min_reps = outer_pattern.min;
    let max_reps = outer_pattern.max;

    if outer_pattern.greedy {
        let can_match_more = match max_reps {
            Some(max) => current_reps < max,
            None => true,
        };

        if can_match_more {
            let mut sub_ends = Vec::new();
            find_group_match_ends(ctx, sub_patterns, current_cell_idx, 0, &mut sub_ends)?;
            for end in sub_ends {
                find_group_reps_ends(
                    ctx,
                    sub_patterns,
                    outer_pattern,
                    end,
                    current_reps + 1,
                    results,
                )?;
            }
        }

        if current_reps >= min_reps {
            results.push(current_cell_idx);
        }
    } else {
        if current_reps >= min_reps {
            results.push(current_cell_idx);
        }

        let can_match_more = match max_reps {
            Some(max) => current_reps < max,
            None => true,
        };

        if can_match_more {
            let mut sub_ends = Vec::new();
            find_group_match_ends(ctx, sub_patterns, current_cell_idx, 0, &mut sub_ends)?;
            for end in sub_ends {
                find_group_reps_ends(
                    ctx,
                    sub_patterns,
                    outer_pattern,
                    end,
                    current_reps + 1,
                    results,
                )?;
            }
        }
    }

    Ok(())
}

fn match_cells(
    ctx: &GroupMatchCtx<'_, '_>,
    patterns: &[CellPattern],
    pattern_idx: usize,
    cell_idx: usize,
) -> PyResult<bool> {
    if pattern_idx == patterns.len() {
        return Ok(cell_idx == ctx.cells.len());
    }

    let pattern = &patterns[pattern_idx];

    if let CellMatchRule::Group(ref sub_group) = pattern.rule {
        return match_group_reps(
            ctx,
            &sub_group.cell_patterns,
            patterns,
            pattern_idx,
            cell_idx,
            0,
        );
    }

    let max_limit = match pattern.max {
        Some(m) => std::cmp::min(m, ctx.cells.len() - cell_idx),
        None => ctx.cells.len() - cell_idx,
    };

    if max_limit < pattern.min {
        return Ok(false);
    }

    let mut matchable = 0;
    while matchable < max_limit {
        if cell_matches_rule(ctx.py, &pattern.rule, &ctx.cells[cell_idx + matchable])? {
            matchable += 1;
        } else {
            break;
        }
    }

    if matchable < pattern.min {
        return Ok(false);
    }

    if pattern.greedy {
        for k in (pattern.min..=matchable).rev() {
            if match_cells(ctx, patterns, pattern_idx + 1, cell_idx + k)? {
                return Ok(true);
            }
        }
    } else {
        for k in pattern.min..=matchable {
            if match_cells(ctx, patterns, pattern_idx + 1, cell_idx + k)? {
                return Ok(true);
            }
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
        s if s.starts_match("{") && s.ends_match("}") => {
            let inner = &s[1..s.len() - 1];
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 1 {
                let val = parts[0].parse::<usize>().unwrap_or(0);
                (val, Some(val))
            } else {
                let min = parts[0].parse::<usize>().unwrap_or(0);
                let max = if parts[1].is_empty() {
                    None
                } else {
                    Some(parts[1].parse::<usize>().unwrap_or(0))
                };
                (min, max)
            }
        }
        _ => (1, Some(1)),
    }
}

trait StringHelpers {
    fn starts_match(&self, s: &str) -> bool;
    fn ends_match(&self, s: &str) -> bool;
}
impl StringHelpers for str {
    fn starts_match(&self, s: &str) -> bool {
        self.starts_with(s)
    }
    fn ends_match(&self, s: &str) -> bool {
        self.ends_with(s)
    }
}

fn matches_row_pattern(
    py: Python<'_>,
    pattern: &RangePattern1D,
    row: &[CellValue],
    col_start: usize,
    col_end: usize,
) -> PyResult<bool> {
    if col_start > col_end || col_end >= row.len() {
        return Ok(false);
    }
    let ctx = GroupMatchCtx {
        py,
        cells: &row[col_start..=col_end],
    };
    match_cells(&ctx, &pattern.cell_patterns, 0, 0)
}

pub fn match_range(
    py: Python<'_>,
    patterns: &[RangePattern1D],
    sheet_data: &[Vec<CellValue>],
    col_start: usize,
    col_end: usize,
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
        if matches_row_pattern(
            py,
            pattern,
            &sheet_data[sheet_row_idx + matchable],
            col_start,
            col_end,
        )? {
            matchable += 1;
        } else {
            break;
        }
    }

    if matchable < min_rows {
        return Ok(None);
    }

    if pattern.greedy {
        for k in (min_rows..=matchable).rev() {
            if let Some(end_idx) = match_range(
                py,
                patterns,
                sheet_data,
                col_start,
                col_end,
                pattern_idx + 1,
                sheet_row_idx + k,
            )? {
                return Ok(Some(end_idx));
            }
        }
    } else {
        for k in min_rows..=matchable {
            if let Some(end_idx) = match_range(
                py,
                patterns,
                sheet_data,
                col_start,
                col_end,
                pattern_idx + 1,
                sheet_row_idx + k,
            )? {
                return Ok(Some(end_idx));
            }
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
            let pattern = pyo3::Bound::new(py, RangePattern1D::new()).unwrap();
            {
                let mut p = pattern.borrow_mut();
                p = RangePattern1D::value(p, "Date".to_string());
                p = RangePattern1D::non_empty(p);
                p = RangePattern1D::one_or_more(p, true).unwrap();
                let _ = RangePattern1D::value(p, "Total".to_string());
            }

            let pattern_ref = pattern.borrow();

            let row1 = vec![
                CellValue::String("Date".to_string()),
                CellValue::String("Description".to_string()),
                CellValue::Int(123),
                CellValue::String("Total".to_string()),
            ];
            assert!(matches_row_pattern(py, &pattern_ref, &row1, 0, row1.len() - 1).unwrap());

            let row2 = vec![
                CellValue::String("Date".to_string()),
                CellValue::String("Total".to_string()),
            ];
            assert!(!matches_row_pattern(py, &pattern_ref, &row2, 0, row2.len() - 1).unwrap());

            let row3 = vec![
                CellValue::String("Date".to_string()),
                CellValue::String("Description".to_string()),
                CellValue::Int(123),
            ];
            assert!(!matches_row_pattern(py, &pattern_ref, &row3, 0, row3.len() - 1).unwrap());
        });
    }

    #[test]
    fn test_backtracking_matcher_regex() {
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            let pattern = pyo3::Bound::new(py, RangePattern1D::new()).unwrap();
            {
                let mut p = pattern.borrow_mut();
                let re = py.import("re").unwrap();
                let re_pattern = re.call_method1("compile", ("^Q[1-4]$",)).unwrap();

                p = RangePattern1D::regex(p, py, &re_pattern).unwrap();
                p = RangePattern1D::repeat(p, 2, Some(2), true).unwrap();
                p = RangePattern1D::empty(p);
                let _ = RangePattern1D::zero_or_more(p, true).unwrap();
            }

            let pattern_ref = pattern.borrow();

            let row1 = vec![
                CellValue::String("Q1".to_string()),
                CellValue::String("Q2".to_string()),
            ];
            assert!(matches_row_pattern(py, &pattern_ref, &row1, 0, row1.len() - 1).unwrap());

            let row2 = vec![
                CellValue::String("Q1".to_string()),
                CellValue::String("Q2".to_string()),
                CellValue::Empty,
                CellValue::Empty,
            ];
            assert!(matches_row_pattern(py, &pattern_ref, &row2, 0, row2.len() - 1).unwrap());

            let row3 = vec![CellValue::String("Q1".to_string()), CellValue::Empty];
            assert!(!matches_row_pattern(py, &pattern_ref, &row3, 0, row3.len() - 1).unwrap());
        });
    }

    #[test]
    fn test_exclusivity_rule() {
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            let pattern = pyo3::Bound::new(py, RangePattern1D::new()).unwrap();
            let mut p = pattern.borrow_mut();
            p = RangePattern1D::empty(p);
            p = RangePattern1D::optional(p, true).unwrap();

            let res = RangePattern1D::one_or_more(p, true);
            assert!(res.is_err());
            let err_msg = res.err().unwrap().to_string();
            assert!(err_msg.contains("Cannot set multiple cardinalities"));
        });
    }

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
        assert_eq!(r.start_col, 1);
        assert_eq!(r.end_col, 3);

        let r = Range::from_a1("AA1").unwrap();
        assert_eq!(r.start_col, 26);
        assert_eq!(r.start_row, 0);

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
    fn test_row_pattern_width_bounds() {
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            let pattern = pyo3::Bound::new(py, RangePattern1D::new()).unwrap();
            {
                let mut p = pattern.borrow_mut();
                p = RangePattern1D::value(p, "Header".to_string());
                p = RangePattern1D::non_empty(p);
                p = RangePattern1D::one_or_more(p, true).unwrap();
                p = RangePattern1D::any(p);
                let _ = RangePattern1D::optional(p, true).unwrap();
            }
            let pattern_ref = pattern.borrow();
            let (min, max) = pattern_ref.width_bounds();
            assert_eq!(min, 2);
            assert_eq!(max, None);
        });

        pyo3::Python::attach(|py| {
            let pattern = pyo3::Bound::new(py, RangePattern1D::new()).unwrap();
            {
                let mut p = pattern.borrow_mut();
                p = RangePattern1D::value(p, "Header".to_string());
                let p = RangePattern1D::non_empty(p);
                let _ = RangePattern1D::repeat(p, 2, Some(4), true).unwrap();
            }
            let pattern_ref = pattern.borrow();
            let (min, max) = pattern_ref.width_bounds();
            assert_eq!(min, 3);
            assert_eq!(max, Some(5));
        });
    }

    #[test]
    fn test_group_pattern_matching() {
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            let sub_group = pyo3::Bound::new(py, RangePattern1D::new()).unwrap();
            {
                let mut sg = sub_group.borrow_mut();
                sg = RangePattern1D::value(sg, "Expected".to_string());
                let _ = RangePattern1D::value(sg, "Actual".to_string());
            }

            let pattern = pyo3::Bound::new(py, RangePattern1D::new()).unwrap();
            {
                let mut p = pattern.borrow_mut();
                p = RangePattern1D::value(p, "Product".to_string());
                p = RangePattern1D::group(p, sub_group.borrow().clone());
                let _ = RangePattern1D::one_or_more(p, true).unwrap();
            }

            let pattern_ref = pattern.borrow();

            let row1 = vec![
                CellValue::String("Product".to_string()),
                CellValue::String("Expected".to_string()),
                CellValue::String("Actual".to_string()),
                CellValue::String("Expected".to_string()),
                CellValue::String("Actual".to_string()),
            ];
            assert!(matches_row_pattern(py, &pattern_ref, &row1, 0, row1.len() - 1).unwrap());

            let row2 = vec![
                CellValue::String("Product".to_string()),
                CellValue::String("Expected".to_string()),
                CellValue::String("Expected".to_string()),
            ];
            assert!(!matches_row_pattern(py, &pattern_ref, &row2, 0, row2.len() - 1).unwrap());

            let row3 = vec![CellValue::String("Product".to_string())];
            assert!(!matches_row_pattern(py, &pattern_ref, &row3, 0, row3.len() - 1).unwrap());
        });
    }
}
