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
                let cloned = pyo3::Python::try_attach(|py| r.clone_ref(py)).unwrap();
                CellMatchRule::Regex(cloned)
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
    pub min: usize,
    #[pyo3(get, set)]
    pub max: Option<isize>,
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
            min: 1,
            max: Some(1),
            greedy: true,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "RangePattern1D(min={}, max={:?}, greedy={}, cell_patterns={:?})",
            self.min, self.max, self.greedy, self.cell_patterns
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
            None => None,
            Some(-1) => Some(min),
            Some(m) if m >= 0 => {
                let u_max = m.unsigned_abs();
                if u_max < min {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "max count ({u_max}) cannot be less than min count ({min})"
                    )));
                }
                Some(u_max)
            }
            Some(m) => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "max count cannot be negative (got {m}, use -1 for exact repeat)"
                )));
            }
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

pub const DEFAULT_MAX_MATCH_DEPTH: usize = 1000;

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct RangeMatcher {
    #[pyo3(get)]
    pub row_patterns: Vec<RangePattern1D>,
    #[pyo3(get)]
    pub outer_direction: String,
    #[pyo3(get)]
    pub inner_direction: String,
    #[pyo3(get, set)]
    pub max_depth: usize,
}

#[pymethods]
impl RangeMatcher {
    #[new]
    #[pyo3(signature = (row_patterns, outer_direction, inner_direction, max_depth = None))]
    pub fn new(
        row_patterns: Vec<RangePattern1D>,
        outer_direction: String,
        inner_direction: String,
        max_depth: Option<usize>,
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
            max_depth: max_depth.unwrap_or(DEFAULT_MAX_MATCH_DEPTH),
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

        let r_count = sheet_data.len();
        let c_count = if r_count > 0 { sheet_data[0].len() } else { 0 };

        let needs_transpose = matches!(self.outer_direction.as_str(), "LR" | "RL");
        let reverse_rows = if needs_transpose {
            self.outer_direction == "RL"
        } else {
            self.outer_direction == "BT"
        };
        let reverse_cols = if needs_transpose {
            self.inner_direction == "BT"
        } else {
            self.inner_direction == "RL"
        };

        if r_count == 0 || c_count == 0 {
            return Ok(false);
        }

        let grid = VirtualGrid {
            sheet_data: &sheet_data,
            start_row: 0,
            end_row: r_count - 1,
            start_col: 0,
            end_col: c_count - 1,
            needs_transpose,
            reverse_rows,
            reverse_cols,
        };

        let col_end = if grid.cols_count() > 0 {
            grid.cols_count() - 1
        } else {
            0
        };
        let matched_end = match_range(
            py,
            &self.row_patterns,
            &grid,
            0,
            col_end,
            0,
            0,
            0,
            self.max_depth,
        )?;
        Ok(matched_end == Some(grid.rows_count()))
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
        CellMatchRule::Empty => Ok(val.is_empty()),
        CellMatchRule::NonEmpty => Ok(!val.is_empty()),
        CellMatchRule::Exact(expected) => {
            let s = val.to_string_for_search();
            Ok(s.as_ref() == expected)
        }
        CellMatchRule::Regex(py_regex) => {
            let s = val.to_string_for_search();
            let bound_regex = py_regex.bind(py);
            let match_obj = bound_regex.call_method1("search", (s.as_ref(),))?;
            Ok(!match_obj.is_none())
        }
        CellMatchRule::Group(_) => {
            // Group patterns are intercepted and dispatched via match_group_reps()
            // in match_cells() before this function is ever called for a Group rule.
            // This arm should be unreachable in normal operation.
            debug_assert!(
                false,
                "cell_matches_rule called with Group rule — this is a bug"
            );
            Ok(false)
        }
    }
}

pub struct VirtualGrid<'a> {
    pub sheet_data: &'a [Vec<CellValue>],
    pub start_row: usize,
    pub end_row: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub needs_transpose: bool,
    pub reverse_rows: bool,
    pub reverse_cols: bool,
}

impl VirtualGrid<'_> {
    pub fn rows_count(&self) -> usize {
        if self.needs_transpose {
            self.end_col.saturating_sub(self.start_col) + 1
        } else {
            self.end_row.saturating_sub(self.start_row) + 1
        }
    }

    pub fn cols_count(&self) -> usize {
        if self.needs_transpose {
            self.end_row.saturating_sub(self.start_row) + 1
        } else {
            self.end_col.saturating_sub(self.start_col) + 1
        }
    }

    pub fn get(&self, r: usize, c: usize) -> &CellValue {
        let t_r = if self.reverse_rows {
            self.rows_count().saturating_sub(1).saturating_sub(r)
        } else {
            r
        };
        let t_c = if self.reverse_cols {
            self.cols_count().saturating_sub(1).saturating_sub(c)
        } else {
            c
        };

        let (orig_r, orig_c) = if self.needs_transpose {
            (self.start_row + t_c, self.start_col + t_r)
        } else {
            (self.start_row + t_r, self.start_col + t_c)
        };

        if orig_r >= self.sheet_data.len() {
            return &CellValue::Empty;
        }
        if orig_c >= self.sheet_data[orig_r].len() {
            return &CellValue::Empty;
        }
        &self.sheet_data[orig_r][orig_c]
    }

    pub fn map_back(
        &self,
        canon_start_row: usize,
        canon_end_row: usize,
        canon_start_col: usize,
        canon_end_col: usize,
    ) -> (usize, usize, usize, usize) {
        let (t_start_row, t_end_row) = if self.reverse_rows {
            (
                self.rows_count()
                    .saturating_sub(1)
                    .saturating_sub(canon_end_row),
                self.rows_count()
                    .saturating_sub(1)
                    .saturating_sub(canon_start_row),
            )
        } else {
            (canon_start_row, canon_end_row)
        };

        let (t_start_col, t_end_col) = if self.reverse_cols {
            (
                self.cols_count()
                    .saturating_sub(1)
                    .saturating_sub(canon_end_col),
                self.cols_count()
                    .saturating_sub(1)
                    .saturating_sub(canon_start_col),
            )
        } else {
            (canon_start_col, canon_end_col)
        };

        if self.needs_transpose {
            let start_row = self.start_row + t_start_col;
            let end_row = self.start_row + t_end_col;
            let start_col = self.start_col + t_start_row;
            let end_col = self.start_col + t_end_row;
            (start_row, end_row, start_col, end_col)
        } else {
            let start_row = self.start_row + t_start_row;
            let end_row = self.start_row + t_end_row;
            let start_col = self.start_col + t_start_col;
            let end_col = self.start_col + t_end_col;
            (start_row, end_row, start_col, end_col)
        }
    }
}

#[derive(Clone, Copy)]
pub struct VirtualRow<'a> {
    pub grid: &'a VirtualGrid<'a>,
    pub r: usize,
    pub col_start: usize,
    pub col_end: usize,
}

impl VirtualRow<'_> {
    pub fn len(&self) -> usize {
        if self.col_start > self.col_end {
            0
        } else {
            self.col_end - self.col_start + 1
        }
    }

    pub fn get(&self, idx: usize) -> &CellValue {
        self.grid.get(self.r, self.col_start + idx)
    }
}

struct GroupMatchCtx<'a, 'py> {
    py: Python<'py>,
    cells: VirtualRow<'a>,
    max_depth: usize,
    depth: std::cell::Cell<usize>,
}

impl GroupMatchCtx<'_, '_> {
    fn check_and_inc_depth(&self) -> PyResult<usize> {
        let cur = self.depth.get();
        if cur >= self.max_depth {
            return Err(pyo3::exceptions::PyRecursionError::new_err(format!(
                "Maximum pattern matching recursion depth of {} exceeded",
                self.max_depth
            )));
        }
        self.depth.set(cur + 1);
        Ok(cur)
    }

    fn reset_depth(&self, prev: usize) {
        self.depth.set(prev);
    }
}

struct DepthGuard<'a, 'b, 'c> {
    ctx: &'a GroupMatchCtx<'b, 'c>,
    prev: usize,
}

impl Drop for DepthGuard<'_, '_, '_> {
    fn drop(&mut self) {
        self.ctx.reset_depth(self.prev);
    }
}

fn match_group_reps(
    ctx: &GroupMatchCtx<'_, '_>,
    sub_patterns: &[CellPattern],
    outer_patterns: &[CellPattern],
    outer_pattern_idx: usize,
    cell_idx: usize,
    current_reps: usize,
) -> PyResult<bool> {
    let prev_depth = ctx.check_and_inc_depth()?;
    let _guard = DepthGuard {
        ctx,
        prev: prev_depth,
    };

    let pattern = &outer_patterns[outer_pattern_idx];
    let min_reps = pattern.min;
    let max_reps = pattern.max;
    let can_match_more = max_reps.is_none_or(|max| current_reps < max);

    let try_match_more = |cell_idx: usize| -> PyResult<bool> {
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
        Ok(false)
    };

    let try_match_rest = || -> PyResult<bool> {
        if current_reps >= min_reps {
            match_cells(ctx, outer_patterns, outer_pattern_idx + 1, cell_idx)
        } else {
            Ok(false)
        }
    };

    if pattern.greedy {
        if can_match_more && try_match_more(cell_idx)? {
            return Ok(true);
        }
        if try_match_rest()? {
            return Ok(true);
        }
    } else {
        if try_match_rest()? {
            return Ok(true);
        }
        if can_match_more && try_match_more(cell_idx)? {
            return Ok(true);
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
    let prev_depth = ctx.check_and_inc_depth()?;
    let _guard = DepthGuard {
        ctx,
        prev: prev_depth,
    };

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
            Some(m) => std::cmp::min(m, ctx.cells.len().saturating_sub(current_cell_idx)),
            None => ctx.cells.len().saturating_sub(current_cell_idx),
        };

        if max_limit < pattern.min {
            return Ok(());
        }

        let mut matchable = 0;
        while matchable < max_limit {
            if cell_matches_rule(
                ctx.py,
                &pattern.rule,
                ctx.cells.get(current_cell_idx + matchable),
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
    let prev_depth = ctx.check_and_inc_depth()?;
    let _guard = DepthGuard {
        ctx,
        prev: prev_depth,
    };

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
    let prev_depth = ctx.check_and_inc_depth()?;
    let _guard = DepthGuard {
        ctx,
        prev: prev_depth,
    };

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
        Some(m) => std::cmp::min(m, ctx.cells.len().saturating_sub(cell_idx)),
        None => ctx.cells.len().saturating_sub(cell_idx),
    };

    if max_limit < pattern.min {
        return Ok(false);
    }

    let mut matchable = 0;
    while matchable < max_limit {
        if cell_matches_rule(ctx.py, &pattern.rule, ctx.cells.get(cell_idx + matchable))? {
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

#[allow(clippy::too_many_arguments)]
fn matches_row_pattern(
    py: Python<'_>,
    pattern: &RangePattern1D,
    grid: &VirtualGrid<'_>,
    row_idx: usize,
    col_start: usize,
    col_end: usize,
    depth: usize,
    max_depth: usize,
) -> PyResult<bool> {
    if col_start > col_end || col_end >= grid.cols_count() {
        return Ok(false);
    }
    let ctx = GroupMatchCtx {
        py,
        cells: VirtualRow {
            grid,
            r: row_idx,
            col_start,
            col_end,
        },
        max_depth,
        depth: std::cell::Cell::new(depth),
    };
    match_cells(&ctx, &pattern.cell_patterns, 0, 0)
}

#[allow(clippy::too_many_arguments)]
pub fn match_range(
    py: Python<'_>,
    patterns: &[RangePattern1D],
    grid: &VirtualGrid<'_>,
    col_start: usize,
    col_end: usize,
    pattern_idx: usize,
    sheet_row_idx: usize,
    depth: usize,
    max_depth: usize,
) -> PyResult<Option<usize>> {
    if depth >= max_depth {
        return Err(pyo3::exceptions::PyRecursionError::new_err(format!(
            "Maximum pattern matching recursion depth of {max_depth} exceeded"
        )));
    }

    if pattern_idx == patterns.len() {
        return Ok(Some(sheet_row_idx));
    }

    let pattern = &patterns[pattern_idx];
    let min_rows = pattern.min;
    let max_rows = match pattern.max {
        Some(-1) => Some(pattern.min),
        Some(val) => usize::try_from(val).ok(),
        None => None,
    };

    let max_limit = match max_rows {
        Some(m) => std::cmp::min(m, grid.rows_count().saturating_sub(sheet_row_idx)),
        None => grid.rows_count().saturating_sub(sheet_row_idx),
    };

    if max_limit < min_rows {
        return Ok(None);
    }

    let mut matchable = 0;
    while matchable < max_limit {
        if matches_row_pattern(
            py,
            pattern,
            grid,
            sheet_row_idx + matchable,
            col_start,
            col_end,
            depth + 1,
            max_depth,
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
                grid,
                col_start,
                col_end,
                pattern_idx + 1,
                sheet_row_idx + k,
                depth + 1,
                max_depth,
            )? {
                return Ok(Some(end_idx));
            }
        }
    } else {
        for k in min_rows..=matchable {
            if let Some(end_idx) = match_range(
                py,
                patterns,
                grid,
                col_start,
                col_end,
                pattern_idx + 1,
                sheet_row_idx + k,
                depth + 1,
                max_depth,
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
            let grid_data1 = vec![row1.clone()];
            let grid1 = VirtualGrid {
                sheet_data: &grid_data1,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row1.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(matches_row_pattern(
                py,
                &pattern_ref,
                &grid1,
                0,
                0,
                row1.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());

            let row2 = vec![
                CellValue::String("Date".to_string()),
                CellValue::String("Total".to_string()),
            ];
            let grid_data2 = vec![row2.clone()];
            let grid2 = VirtualGrid {
                sheet_data: &grid_data2,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row2.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(!matches_row_pattern(
                py,
                &pattern_ref,
                &grid2,
                0,
                0,
                row2.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());

            let row3 = vec![
                CellValue::String("Date".to_string()),
                CellValue::String("Description".to_string()),
                CellValue::Int(123),
            ];
            let grid_data3 = vec![row3.clone()];
            let grid3 = VirtualGrid {
                sheet_data: &grid_data3,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row3.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(!matches_row_pattern(
                py,
                &pattern_ref,
                &grid3,
                0,
                0,
                row3.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());
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
            let grid_data1 = vec![row1.clone()];
            let grid1 = VirtualGrid {
                sheet_data: &grid_data1,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row1.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(matches_row_pattern(
                py,
                &pattern_ref,
                &grid1,
                0,
                0,
                row1.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());

            let row2 = vec![
                CellValue::String("Q1".to_string()),
                CellValue::String("Q2".to_string()),
                CellValue::Empty,
                CellValue::Empty,
            ];
            let grid_data2 = vec![row2.clone()];
            let grid2 = VirtualGrid {
                sheet_data: &grid_data2,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row2.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(matches_row_pattern(
                py,
                &pattern_ref,
                &grid2,
                0,
                0,
                row2.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());

            let row3 = vec![CellValue::String("Q1".to_string()), CellValue::Empty];
            let grid_data3 = vec![row3.clone()];
            let grid3 = VirtualGrid {
                sheet_data: &grid_data3,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row3.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(!matches_row_pattern(
                py,
                &pattern_ref,
                &grid3,
                0,
                0,
                row3.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());
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
            let grid_data1 = vec![row1.clone()];
            let grid1 = VirtualGrid {
                sheet_data: &grid_data1,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row1.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(matches_row_pattern(
                py,
                &pattern_ref,
                &grid1,
                0,
                0,
                row1.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());

            let row2 = vec![
                CellValue::String("Product".to_string()),
                CellValue::String("Expected".to_string()),
                CellValue::String("Expected".to_string()),
            ];
            let grid_data2 = vec![row2.clone()];
            let grid2 = VirtualGrid {
                sheet_data: &grid_data2,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row2.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(!matches_row_pattern(
                py,
                &pattern_ref,
                &grid2,
                0,
                0,
                row2.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());

            let row3 = vec![CellValue::String("Product".to_string())];
            let grid_data3 = vec![row3.clone()];
            let grid3 = VirtualGrid {
                sheet_data: &grid_data3,
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: row3.len() - 1,
                needs_transpose: false,
                reverse_rows: false,
                reverse_cols: false,
            };
            assert!(!matches_row_pattern(
                py,
                &pattern_ref,
                &grid3,
                0,
                0,
                row3.len() - 1,
                0,
                DEFAULT_MAX_MATCH_DEPTH
            )
            .unwrap());
        });
    }
}
