use crate::matcher::{match_range, Range, RangeMatcher};
use calamine::{open_workbook, Data, Reader, Xlsx};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::BoundObject;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Empty,
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Error(String),
}

impl From<calamine::Data> for CellValue {
    fn from(data: calamine::Data) -> Self {
        match data {
            Data::Empty | Data::DurationIso(_) => CellValue::Empty,
            Data::String(s) | Data::DateTimeIso(s) => CellValue::String(s),
            Data::Float(f) => CellValue::Float(f),
            Data::DateTime(f) => CellValue::Float(f.as_f64()),
            Data::Int(i) => CellValue::Int(i),
            Data::Bool(b) => CellValue::Bool(b),
            Data::Error(e) => CellValue::Error(format!("{e:?}")),
        }
    }
}

impl CellValue {
    pub fn to_string_for_search(&self) -> String {
        match self {
            CellValue::Empty => String::new(),
            CellValue::String(s) => s.clone(),
            CellValue::Float(f) => f.to_string(),
            CellValue::Int(i) => i.to_string(),
            CellValue::Bool(b) => {
                if *b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            CellValue::Error(e) => format!("ERROR: {e}"),
        }
    }
}

impl<'py> IntoPyObject<'py> for CellValue {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(
        self,
        py: Python<'py>,
    ) -> Result<Self::Output, <Self as IntoPyObject<'py>>::Error> {
        match self {
            CellValue::Empty => Ok(py.None().into_bound(py)),
            CellValue::String(s) => Ok(s.into_pyobject(py)?.into_any()),
            CellValue::Float(f) => Ok(f.into_pyobject(py)?.into_any()),
            CellValue::Int(i) => Ok(i.into_pyobject(py)?.into_any()),
            CellValue::Bool(b) => Ok(pyo3::types::PyBool::new(py, b).into_bound().into_any()),
            CellValue::Error(e) => Ok(e.into_pyobject(py)?.into_any()),
        }
    }
}

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct Sheet {
    #[pyo3(get)]
    pub name: String,
    pub data: Vec<Vec<CellValue>>,
    pub merged_regions: Vec<((usize, usize), (usize, usize))>,
}

#[pymethods]
impl Sheet {
    #[getter]
    pub fn shape(&self) -> (usize, usize) {
        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };
        (rows, cols)
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn get_cell_value(&self, py: Python<'_>, row: isize, col: isize) -> PyResult<Py<PyAny>> {
        if row < 0 || col < 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }
        let r = row as usize;
        let c = col as usize;
        if r >= self.data.len() || (!self.data.is_empty() && c >= self.data[0].len()) {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }
        let val = &self.data[r][c];
        let bound = val.clone().into_pyobject(py)?;
        Ok(bound.into_any().unbind())
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn set_cell_value(&mut self, row: isize, col: isize, value: String) -> PyResult<()> {
        if row < 0 || col < 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }
        let r = row as usize;
        let c = col as usize;
        if r >= self.data.len() || (!self.data.is_empty() && c >= self.data[0].len()) {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }
        self.data[r][c] = CellValue::String(value);
        Ok(())
    }

    pub fn copy(&self) -> Self {
        self.clone()
    }

    pub fn __copy__(&self) -> Self {
        self.clone()
    }

    pub fn __deepcopy__(&self, _memo: &Bound<'_, PyAny>) -> Self {
        self.clone()
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn search_and_drop(
        &mut self,
        _py: Python<'_>,
        str_or_regex: &Bound<'_, PyAny>,
        drop_direction: &str,
    ) -> PyResult<((usize, usize), (usize, usize))> {
        let (rows, cols) = self.shape();
        if rows == 0 || cols == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Cannot search an empty sheet",
            ));
        }

        // Validate drop direction first
        let valid_directions = [
            "top",
            "bottom",
            "left",
            "right",
            "top_left",
            "top_right",
            "bottom_left",
            "bottom_right",
        ];
        if !valid_directions.contains(&drop_direction) {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Invalid drop direction '{drop_direction}'. Must be one of {valid_directions:?}"
            )));
        }

        // Determine matching strategy
        let mut matched_coords: Option<(usize, usize)> = None;

        // Check if str_or_regex is a string
        if let Ok(target_str) = str_or_regex.extract::<String>() {
            'outer_str: for r in 0..rows {
                for c in 0..cols {
                    let cell_str = self.data[r][c].to_string_for_search();
                    if cell_str == target_str {
                        matched_coords = Some((r, c));
                        break 'outer_str;
                    }
                }
            }
        } else if str_or_regex.hasattr("search").unwrap_or(false) {
            // It has a search method, treat as compiled regex pattern
            'outer_regex: for r in 0..rows {
                for c in 0..cols {
                    let cell_str = self.data[r][c].to_string_for_search();
                    let match_obj = str_or_regex.call_method1("search", (cell_str,))?;
                    if !match_obj.is_none() {
                        matched_coords = Some((r, c));
                        break 'outer_regex;
                    }
                }
            }
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "str_or_regex must be a string or a compiled regex pattern (from re.compile)",
            ));
        }

        let Some((matched_row, matched_col)) = matched_coords else {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Search term not found in sheet",
            ));
        };

        // Perform the drop operations
        let drop_top = drop_direction.contains("top") || drop_direction == "top";
        let drop_bottom = drop_direction.contains("bottom") || drop_direction == "bottom";
        let drop_left = drop_direction.contains("left") || drop_direction == "left";
        let drop_right = drop_direction.contains("right") || drop_direction == "right";

        if drop_top {
            for _ in 0..matched_row {
                self.drop_row(0)?;
            }
        }

        if drop_bottom {
            // After drop_top, the matched row is now at index 0 (if drop_top was true),
            // or remains at matched_row (if drop_top was false).
            let current_matched_row = if drop_top { 0 } else { matched_row };
            let (current_rows, _) = self.shape();
            if current_rows > current_matched_row + 1 {
                let to_drop = current_rows - 1 - current_matched_row;
                for _ in 0..to_drop {
                    self.drop_row((current_matched_row + 1) as isize)?;
                }
            }
        }

        if drop_left {
            for _ in 0..matched_col {
                self.drop_column(0)?;
            }
        }

        if drop_right {
            // After drop_left, the matched col is now at index 0 (if drop_left was true),
            // or remains at matched_col (if drop_left was false).
            let current_matched_col = if drop_left { 0 } else { matched_col };
            let (_, current_cols) = self.shape();
            if current_cols > current_matched_col + 1 {
                let to_drop = current_cols - 1 - current_matched_col;
                for _ in 0..to_drop {
                    self.drop_column((current_matched_col + 1) as isize)?;
                }
            }
        }

        let new_row = if drop_top { 0 } else { matched_row };
        let new_col = if drop_left { 0 } else { matched_col };

        Ok(((matched_row, matched_col), (new_row, new_col)))
    }

    #[pyo3(signature = (path, zero_based_indices = true))]
    pub fn to_svg(&self, path: &str, zero_based_indices: bool) -> PyResult<()> {
        self.to_svg_impl(path, zero_based_indices)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write SVG: {e}")))
    }

    #[allow(clippy::cast_sign_loss, clippy::similar_names)]
    pub fn drop_row(&mut self, row_idx: isize) -> PyResult<()> {
        if row_idx < 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }
        let target = row_idx as usize;
        if target >= self.data.len() {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }

        // Remove row from grid data
        self.data.remove(target);

        // Adjust merged regions
        let mut new_merged = Vec::new();
        for &((s_row, s_col), (e_row, e_col)) in &self.merged_regions {
            // Case 3: Completely contained in target -> Delete
            if s_row == target && e_row == target {
                continue;
            }

            let mut next_s_row = s_row;
            let mut next_e_row = e_row;

            // Case 1: Shift Up
            if s_row > target {
                next_s_row -= 1;
                next_e_row -= 1;
            }
            // Case 2: Shrink
            else if s_row <= target && e_row >= target {
                next_e_row -= 1;
            }

            // Case 4: Cleanup (if 1x1 region, discard it)
            if next_s_row == next_e_row && s_col == e_col {
                continue;
            }

            new_merged.push(((next_s_row, s_col), (next_e_row, e_col)));
        }
        self.merged_regions = new_merged;

        Ok(())
    }

    #[allow(clippy::cast_sign_loss, clippy::similar_names)]
    pub fn drop_column(&mut self, col_idx: isize) -> PyResult<()> {
        if col_idx < 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }
        let target = col_idx as usize;
        let cols = if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        };
        if target >= cols {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }

        // Remove column from grid data
        for row in &mut self.data {
            if target < row.len() {
                row.remove(target);
            }
        }

        // Adjust merged regions
        let mut new_merged = Vec::new();
        for &((s_row, s_col), (e_row, e_col)) in &self.merged_regions {
            // Case 3: Completely contained in target -> Delete
            if s_col == target && e_col == target {
                continue;
            }

            let mut next_s_col = s_col;
            let mut next_e_col = e_col;

            // Case 1: Shift Left
            if s_col > target {
                next_s_col -= 1;
                next_e_col -= 1;
            }
            // Case 2: Shrink
            else if s_col <= target && e_col >= target {
                next_e_col -= 1;
            }

            // Case 4: Cleanup (if 1x1 region, discard it)
            if s_row == e_row && next_s_col == next_e_col {
                continue;
            }

            new_merged.push(((s_row, next_s_col), (e_row, next_e_col)));
        }
        self.merged_regions = new_merged;

        Ok(())
    }

    #[pyo3(signature = (matcher, start_row = None, end_row = None, start_col = None, end_col = None))]
    pub fn search_range(
        &self,
        py: Python<'_>,
        matcher: &RangeMatcher,
        start_row: Option<isize>,
        end_row: Option<isize>,
        start_col: Option<isize>,
        end_col: Option<isize>,
    ) -> PyResult<Option<Range>> {
        let rows_count = self.data.len();
        let cols_count = if rows_count > 0 {
            self.data[0].len()
        } else {
            0
        };

        if rows_count == 0 || cols_count == 0 {
            return Ok(None);
        }

        let (resolved_start_row, resolved_end_row, resolved_start_col, resolved_end_col) =
            Self::resolve_search_bounds(
                rows_count, cols_count, start_row, end_row, start_col, end_col,
            )?;

        // Construct the row-sliced data (keeping full column width)
        let mut sliced_data: Vec<Vec<CellValue>> = Vec::new();
        for r in resolved_start_row..=resolved_end_row {
            sliced_data.push(self.data[r].clone());
        }

        Self::scan_grid_for_range(
            py,
            matcher,
            &sliced_data,
            resolved_start_row,
            resolved_start_col,
            resolved_end_col,
        )
    }

    pub fn get_range_between(&self, start: &Range, end: &Range) -> PyResult<Range> {
        let rows_count = self.data.len();

        let is_vertical = start.end_row < end.start_row;
        let is_horizontal = start.end_col < end.start_col;

        if is_vertical && !is_horizontal {
            // Vertical separation
            if start.start_col != end.start_col || start.end_col != end.end_col {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "Column spans of start and end ranges do not align for vertical separation.",
                ));
            }
            let start_row = start.end_row + 1;
            let end_row = end.start_row - 1;
            if start_row > end_row {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "No rows exist between start and end ranges.",
                ));
            }
            if end_row >= rows_count {
                return Err(pyo3::exceptions::PyIndexError::new_err(
                    "Resolved end row index is out of sheet bounds.",
                ));
            }
            Ok(Range {
                start_row,
                end_row,
                start_col: start.start_col,
                end_col: start.end_col,
            })
        } else if is_horizontal && !is_vertical {
            // Horizontal separation
            if start.start_row != end.start_row || start.end_row != end.end_row {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "Row spans of start and end ranges do not align for horizontal separation.",
                ));
            }
            let start_col = start.end_col + 1;
            let end_col = end.start_col - 1;
            if start_col > end_col {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "No columns exist between start and end ranges.",
                ));
            }
            let cols_count = if rows_count > 0 {
                self.data[0].len()
            } else {
                0
            };
            if end_col >= cols_count {
                return Err(pyo3::exceptions::PyIndexError::new_err(
                    "Resolved end col index is out of sheet bounds.",
                ));
            }
            Ok(Range {
                start_row: start.start_row,
                end_row: start.end_row,
                start_col,
                end_col,
            })
        } else if is_vertical && is_horizontal {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Ranges are separated diagonally. They must be aligned either vertically or horizontally.",
            ))
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Ranges overlap or the start range is positioned after the end range.",
            ))
        }
    }

    #[pyo3(signature = (data, header=None, clean_names=false, flatten_header=false, header_separator="_"))]
    pub fn extract_table(
        &self,
        data: &Range,
        header: Option<&Range>,
        clean_names: bool,
        flatten_header: bool,
        header_separator: &str,
    ) -> PyResult<crate::table::Table> {
        crate::table::Table::extract_from_sheet(
            self,
            data,
            header,
            clean_names,
            flatten_header,
            header_separator,
        )
    }
}

impl Sheet {
    #[allow(clippy::cast_sign_loss)]
    fn resolve_search_bounds(
        rows_count: usize,
        cols_count: usize,
        start_row: Option<isize>,
        end_row: Option<isize>,
        start_col: Option<isize>,
        end_col: Option<isize>,
    ) -> PyResult<(usize, usize, usize, usize)> {
        let resolved_start_row = match start_row {
            Some(r) => {
                if r < 0 || r as usize >= rows_count {
                    return Err(pyo3::exceptions::PyIndexError::new_err(
                        "start_row out of bounds",
                    ));
                }
                r as usize
            }
            None => 0,
        };

        let resolved_end_row = match end_row {
            Some(r) => {
                if r < 0 || r as usize >= rows_count {
                    return Err(pyo3::exceptions::PyIndexError::new_err(
                        "end_row out of bounds",
                    ));
                }
                r as usize
            }
            None => rows_count - 1,
        };

        let resolved_start_col = match start_col {
            Some(c) => {
                if c < 0 || c as usize >= cols_count {
                    return Err(pyo3::exceptions::PyIndexError::new_err(
                        "start_col out of bounds",
                    ));
                }
                c as usize
            }
            None => 0,
        };

        let resolved_end_col = match end_col {
            Some(c) => {
                if c < 0 || c as usize >= cols_count {
                    return Err(pyo3::exceptions::PyIndexError::new_err(
                        "end_col out of bounds",
                    ));
                }
                c as usize
            }
            None => cols_count - 1,
        };

        if resolved_start_row > resolved_end_row {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "start_row ({resolved_start_row}) cannot be greater than end_row ({resolved_end_row})"
            )));
        }
        if resolved_start_col > resolved_end_col {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "start_col ({resolved_start_col}) cannot be greater than end_col ({resolved_end_col})"
            )));
        }

        Ok((
            resolved_start_row,
            resolved_end_row,
            resolved_start_col,
            resolved_end_col,
        ))
    }

    fn scan_grid_for_range(
        py: Python<'_>,
        matcher: &RangeMatcher,
        sliced_data: &[Vec<CellValue>],
        resolved_start_row: usize,
        resolved_start_col: usize,
        resolved_end_col: usize,
    ) -> PyResult<Option<Range>> {
        let (min_w, max_w) = if let Some(first_pattern) = matcher.row_patterns.first() {
            first_pattern.width_bounds()
        } else {
            (1, None)
        };

        for i in 0..sliced_data.len() {
            for c_start in resolved_start_col..=resolved_end_col {
                let min_end = match c_start.checked_add(min_w) {
                    Some(sum) => sum.saturating_sub(1),
                    None => continue,
                };
                if min_end > resolved_end_col {
                    continue;
                }
                let max_end = match max_w {
                    Some(max_len) => match c_start.checked_add(max_len) {
                        Some(sum) => std::cmp::min(sum.saturating_sub(1), resolved_end_col),
                        None => resolved_end_col,
                    },
                    None => resolved_end_col,
                };

                for c_end in min_end..=max_end {
                    if let Some(end_idx) =
                        match_range(py, &matcher.row_patterns, sliced_data, c_start, c_end, 0, i)?
                    {
                        if end_idx > i {
                            return Ok(Some(Range {
                                start_row: resolved_start_row + i,
                                end_row: resolved_start_row + end_idx - 1,
                                start_col: c_start,
                                end_col: c_end,
                            }));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    pub fn get_merged_cell_value(&self, row: usize, col: usize) -> &CellValue {
        for &((s_row, s_col), (e_row, e_col)) in &self.merged_regions {
            if row >= s_row && row <= e_row && col >= s_col && col <= e_col {
                return &self.data[s_row][s_col];
            }
        }
        &self.data[row][col]
    }

    #[allow(clippy::too_many_lines)]
    fn to_svg_impl(&self, path: &str, zero_based_indices: bool) -> std::io::Result<()> {
        let (rows, cols) = self.shape();

        let cell_width = 120;
        let cell_height = 30;
        let row_hdr_width = 40;
        let col_hdr_height = 25;

        let svg_width = row_hdr_width + cols * cell_width;
        let svg_height = col_hdr_height + rows * cell_height;

        let mut svg = String::new();
        let _ = write!(
            svg,
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{svg_width}" height="{svg_height}">"#
        );

        svg.push_str(r#"
<style>
  text {
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    font-size: 11px;
    fill: #1f2937;
  }
  .hdr-text {
    font-weight: 600;
    fill: #4b5563;
    text-anchor: middle;
  }
  .grid-line {
    stroke: #e5e7eb;
    stroke-width: 1;
  }
  .cell-rect {
    fill: #ffffff;
    stroke: #e5e7eb;
    stroke-width: 1;
  }
  .hdr-rect {
    fill: #f3f4f6;
    stroke: #d1d5db;
    stroke-width: 1;
  }
  .cell-merged {
    fill: #fbfbfb;
  }
  .val-string {
    text-anchor: start;
    fill: #2563eb;
  }
  .val-number {
    text-anchor: end;
    fill: #059669;
  }
  .val-bool {
    text-anchor: middle;
    fill: #7c3aed;
    font-weight: 500;
  }
  .rect-bool {
    fill: #f5f3ff;
  }
  .val-error {
    text-anchor: middle;
    fill: #dc2626;
    font-weight: 500;
  }
  .rect-error {
    fill: #fee2e2;
  }
</style>
"#);

        if rows == 0 || cols == 0 {
            svg.push_str(r##"<rect width="100%" height="100%" fill="#f9fafb"/>"##);
            svg.push_str(r##"<text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" font-size="14" fill="#9ca3af">Empty Sheet</text>"##);
            svg.push_str("</svg>\n");
            std::fs::write(path, svg)?;
            return Ok(());
        }

        for r in 0..rows {
            for c in 0..cols {
                let mut is_merged = false;
                let mut draw_cell = true;
                let mut c_width = cell_width;
                let mut c_height = cell_height;

                for &(start, end) in &self.merged_regions {
                    if r >= start.0 && r <= end.0 && c >= start.1 && c <= end.1 {
                        is_merged = true;
                        if r == start.0 && c == start.1 {
                            c_width = (end.1 - start.1 + 1) * cell_width;
                            c_height = (end.0 - start.0 + 1) * cell_height;
                        } else {
                            draw_cell = false;
                        }
                        break;
                    }
                }

                if !draw_cell {
                    continue;
                }

                let cell_x = row_hdr_width + c * cell_width;
                let cell_y = col_hdr_height + r * cell_height;

                let val = &self.data[r][c];

                let mut rect_class = "cell-rect".to_string();
                let mut text_class = String::new();

                if is_merged {
                    rect_class.push_str(" cell-merged");
                }

                match val {
                    CellValue::Bool(_) => {
                        rect_class.push_str(" rect-bool");
                        text_class.push_str("val-bool");
                    }
                    CellValue::Error(_) => {
                        rect_class.push_str(" rect-error");
                        text_class.push_str("val-error");
                    }
                    CellValue::String(_) => {
                        text_class.push_str("val-string");
                    }
                    CellValue::Float(_) | CellValue::Int(_) => {
                        text_class.push_str("val-number");
                    }
                    CellValue::Empty => {}
                }

                let _ = writeln!(
                    svg,
                    r#"  <rect x="{cell_x}" y="{cell_y}" width="{c_width}" height="{c_height}" class="{rect_class}" />"#
                );

                let val_str = match val {
                    CellValue::Empty => String::new(),
                    CellValue::String(s) => s.clone(),
                    CellValue::Float(f) => f.to_string(),
                    CellValue::Int(i) => i.to_string(),
                    CellValue::Bool(b) => {
                        if *b {
                            "TRUE".to_string()
                        } else {
                            "FALSE".to_string()
                        }
                    }
                    CellValue::Error(e) => format!("ERROR: {e}"),
                };

                if !val_str.is_empty() {
                    let max_chars = c_width * 2 / 13;
                    let display_str = if val_str.chars().count() > max_chars && max_chars > 3 {
                        let mut truncated: String = val_str.chars().take(max_chars - 3).collect();
                        truncated.push_str("...");
                        truncated
                    } else {
                        val_str
                    };

                    let text_x = match val {
                        CellValue::Float(_) | CellValue::Int(_) => cell_x + c_width - 8,
                        CellValue::Bool(_) | CellValue::Error(_) => cell_x + c_width / 2,
                        _ => cell_x + 8,
                    };
                    let text_y = cell_y + c_height / 2 + 4;

                    let escaped = html_escape(&display_str);
                    let _ = writeln!(
                        svg,
                        r#"  <text x="{text_x}" y="{text_y}" class="{text_class}">{escaped}</text>"#
                    );
                }
            }
        }

        for c in 0..cols {
            let cell_x = row_hdr_width + c * cell_width;
            let letters = index_to_col_letters(c);
            let col_idx = if zero_based_indices { c } else { c + 1 };
            let col_label = format!("{letters} ({col_idx})");
            let text_x = cell_x + cell_width / 2;
            let text_y = col_hdr_height / 2 + 4;
            let _ = writeln!(
                svg,
                r#"  <rect x="{cell_x}" y="0" width="{cell_width}" height="{col_hdr_height}" class="hdr-rect" />
  <text x="{text_x}" y="{text_y}" class="hdr-text">{col_label}</text>"#
            );
        }

        for r in 0..rows {
            let cell_y = col_hdr_height + r * cell_height;
            let label = if zero_based_indices { r } else { r + 1 };
            let text_x = row_hdr_width / 2;
            let text_y = cell_y + cell_height / 2 + 4;
            let _ = writeln!(
                svg,
                r#"  <rect x="0" y="{cell_y}" width="{row_hdr_width}" height="{cell_height}" class="hdr-rect" />
  <text x="{text_x}" y="{text_y}" class="hdr-text">{label}</text>"#
            );
        }

        let _ = writeln!(
            svg,
            r#"  <rect x="0" y="0" width="{row_hdr_width}" height="{col_hdr_height}" class="hdr-rect" />"#
        );

        svg.push_str("</svg>\n");

        std::fs::write(path, svg)?;
        Ok(())
    }
}

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct Workbook {
    pub sheets: HashMap<String, Sheet>,
    pub active_sheet_name: String,
}

#[pymethods]
impl Workbook {
    pub fn active_sheet(&self) -> PyResult<Sheet> {
        if let Some(sheet) = self.sheets.get(&self.active_sheet_name) {
            Ok(sheet.clone())
        } else if let Some(sheet) = self.sheets.values().next() {
            Ok(sheet.clone())
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "No sheets in workbook",
            ))
        }
    }

    pub fn sheet_names(&self) -> Vec<String> {
        self.sheets.keys().cloned().collect()
    }

    pub fn get_sheet(&self, name: &str) -> PyResult<Sheet> {
        if let Some(sheet) = self.sheets.get(name) {
            Ok(sheet.clone())
        } else {
            Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "Sheet '{name}' not found"
            )))
        }
    }
}

pub fn load_workbook_impl(path: &str) -> PyResult<Workbook> {
    let path_buf = Path::new(path);
    if !path_buf.exists() {
        return Err(pyo3::exceptions::PyFileNotFoundError::new_err(format!(
            "File not found: {path}"
        )));
    }

    let mut excel: Xlsx<_> = open_workbook(path_buf)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Calamine error: {e}")))?;

    let _ = excel.load_merged_regions();

    let sheet_names = excel.sheet_names();
    let mut sheets = HashMap::new();
    let mut active_sheet_name = String::new();

    if !sheet_names.is_empty() {
        active_sheet_name.clone_from(&sheet_names[0]);
    }

    let all_merged_regions: Vec<_> = excel
        .merged_regions()
        .iter()
        .map(|(s_name, _path, dim)| {
            (
                s_name.clone(),
                (
                    (dim.start.0 as usize, dim.start.1 as usize),
                    (dim.end.0 as usize, dim.end.1 as usize),
                ),
            )
        })
        .collect();

    for name in sheet_names {
        if let Ok(range) = excel.worksheet_range(&name) {
            let (start_row, start_col) = range.start().unwrap_or((0, 0));
            let (end_row, end_col) = range.end().unwrap_or((0, 0));

            let mut height = if range.start().is_none() {
                0
            } else {
                end_row as usize + 1
            };
            let mut width = if range.start().is_none() {
                0
            } else {
                end_col as usize + 1
            };

            let sheet_merges: Vec<((usize, usize), (usize, usize))> = all_merged_regions
                .iter()
                .filter(|(s_name, _)| s_name == &name)
                .map(|(_, region)| *region)
                .collect();

            for &(_start_coord, end_coord) in &sheet_merges {
                if end_coord.0 >= height {
                    height = end_coord.0 + 1;
                }
                if end_coord.1 >= width {
                    width = end_coord.1 + 1;
                }
            }

            let mut data = vec![vec![CellValue::Empty; width]; height];

            for (row_idx, row) in range.rows().enumerate() {
                for (col_idx, cell) in row.iter().enumerate() {
                    let abs_row = row_idx + start_row as usize;
                    let abs_col = col_idx + start_col as usize;
                    data[abs_row][abs_col] = CellValue::from(cell.clone());
                }
            }

            sheets.insert(
                name.clone(),
                Sheet {
                    name,
                    data,
                    merged_regions: sheet_merges,
                },
            );
        }
    }

    Ok(Workbook {
        sheets,
        active_sheet_name,
    })
}

pub fn index_to_col_letters(col: usize) -> String {
    let mut col_str = String::new();
    let mut temp = col;
    loop {
        let remainder = temp % 26;
        let ch = (b'A' + u8::try_from(remainder).unwrap()) as char;
        col_str.insert(0, ch);
        if temp < 26 {
            break;
        }
        temp = temp / 26 - 1;
    }
    col_str
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_workbook() {
        let wb = load_workbook_impl("tests/data/sample.xlsx").unwrap();
        let mut names = wb.sheet_names();
        names.sort();
        assert_eq!(names, vec!["complex", "multi-tables", "simple"]);

        let sheet = wb.get_sheet("simple").unwrap();
        assert_eq!(sheet.name, "simple");
        assert_eq!(sheet.shape(), (5, 3)); // 5 rows, 3 cols (A1 to C5)

        // Check cell values
        assert_eq!(sheet.data[0][0], CellValue::String("Header #1".to_string()));
        assert_eq!(sheet.data[0][1], CellValue::String("Header #2".to_string()));
        assert_eq!(sheet.data[0][2], CellValue::String("Header #3".to_string()));
        assert_eq!(sheet.data[1][0], CellValue::String("ABC".to_string()));
        assert_eq!(sheet.data[1][1], CellValue::Float(123.45));
        assert_eq!(sheet.data[1][2], CellValue::String("Alice".to_string()));
        assert_eq!(sheet.data[2][0], CellValue::String("DEF".to_string()));
        assert_eq!(sheet.data[2][1], CellValue::Float(678.0));
        assert_eq!(sheet.data[2][2], CellValue::String("Bob".to_string()));

        // Merged cell A4 is "Merged value"
        assert_eq!(
            sheet.data[3][0],
            CellValue::String("Merged value".to_string())
        );
        // B4, A5, B5 should be Empty in raw data because calamine does not automatically fill them
        assert_eq!(sheet.data[3][1], CellValue::Empty);
        assert_eq!(sheet.data[4][0], CellValue::Empty);
        assert_eq!(sheet.data[4][1], CellValue::Empty);
        assert_eq!(sheet.data[3][2], CellValue::String("Charlie".to_string()));
        assert_eq!(sheet.data[4][2], CellValue::String("David".to_string()));

        // Merged regions check
        assert_eq!(sheet.merged_regions.len(), 1);
        assert_eq!(sheet.merged_regions[0], ((3, 0), (4, 1))); // A4:B5

        // Verify complex sheet loading
        let complex_sheet = wb.get_sheet("complex").unwrap();
        assert_eq!(complex_sheet.name, "complex");
        assert_eq!(complex_sheet.shape(), (15, 5)); // 15 rows, 5 columns
        assert_eq!(
            complex_sheet.data[0][0],
            CellValue::String("Financial Report 2026".to_string())
        );
        assert_eq!(
            complex_sheet.data[3][0],
            CellValue::String("North".to_string())
        );
        assert_eq!(complex_sheet.data[10][2], CellValue::Bool(true));
        assert_eq!(complex_sheet.merged_regions.len(), 2);

        // Verify get_cell_value and set_cell_value
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            assert!(sheet.get_cell_value(py, 5, 0).is_err()); // Out of bounds
            let val = sheet.get_cell_value(py, 0, 0).unwrap();
            let s: String = val.extract(py).unwrap();
            assert_eq!(s, "Header #1");

            let mut mut_sheet = sheet.clone();
            mut_sheet
                .set_cell_value(0, 0, "NewHeader".to_string())
                .unwrap();
            let val = mut_sheet.get_cell_value(py, 0, 0).unwrap();
            let s: String = val.extract(py).unwrap();
            assert_eq!(s, "NewHeader");
            assert!(mut_sheet.set_cell_value(5, 0, "Err".to_string()).is_err());
            // Out of bounds
        });

        // Verify drop_row and drop_column
        {
            let mut test_sheet = sheet.clone();
            // Drop row 1 (the second row, index 1)
            test_sheet.drop_row(1).unwrap();
            assert_eq!(test_sheet.shape(), (4, 3));
            // Merged region ((3, 0), (4, 1)) should shift up by 1 to ((2, 0), (3, 1))
            assert_eq!(test_sheet.merged_regions.len(), 1);
            assert_eq!(test_sheet.merged_regions[0], ((2, 0), (3, 1)));

            // Drop row 2 (which is index 2, now inside the merged region ((2, 0), (3, 1)))
            test_sheet.drop_row(2).unwrap();
            assert_eq!(test_sheet.shape(), (3, 3));
            // Merged region should shrink from 2 rows to 1 row: ((2, 0), (2, 1))
            assert_eq!(test_sheet.merged_regions.len(), 1);
            assert_eq!(test_sheet.merged_regions[0], ((2, 0), (2, 1)));

            // Drop column 1 (index 1, which is inside the merged region ((2, 0), (2, 1)))
            test_sheet.drop_column(1).unwrap();
            assert_eq!(test_sheet.shape(), (3, 2));
            // Merged region ((2, 0), (2, 1)) should shrink in width to ((2, 0), (2, 0)),
            // which becomes a 1x1 region, so it must be cleaned up (deleted).
            assert_eq!(test_sheet.merged_regions.len(), 0);

            // Test out of bounds drop
            assert!(test_sheet.drop_row(-1).is_err());
            assert!(test_sheet.drop_row(3).is_err());
            assert!(test_sheet.drop_column(-1).is_err());
            assert!(test_sheet.drop_column(2).is_err());

            // Drop remaining rows until empty
            test_sheet.drop_row(0).unwrap();
            test_sheet.drop_row(0).unwrap();
            test_sheet.drop_row(0).unwrap();
            assert_eq!(test_sheet.shape(), (0, 0));
        }
    }

    #[test]
    fn test_sheet_copy() {
        let wb = load_workbook_impl("tests/data/sample.xlsx").unwrap();
        let sheet = wb.get_sheet("simple").unwrap();

        let cloned_sheet = sheet.copy();
        assert_eq!(cloned_sheet.name, sheet.name);
        assert_eq!(cloned_sheet.shape(), sheet.shape());
        assert_eq!(cloned_sheet.merged_regions, sheet.merged_regions);

        // Mutate clone and check that original is unchanged
        let mut mut_clone = cloned_sheet;
        mut_clone
            .set_cell_value(0, 0, "Mutated".to_string())
            .unwrap();

        // Original has "Header #1"
        assert_eq!(sheet.data[0][0], CellValue::String("Header #1".to_string()));
        // Clone has "Mutated"
        assert_eq!(
            mut_clone.data[0][0],
            CellValue::String("Mutated".to_string())
        );

        // Verify PyO3 bindings for copy, __copy__, __deepcopy__
        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            let bound_sheet = sheet.clone().into_pyobject(py).unwrap();

            // test copy
            let copied: Sheet = bound_sheet.call_method0("copy").unwrap().extract().unwrap();
            assert_eq!(copied.name, sheet.name);

            // test __copy__
            let copied_dunder: Sheet = bound_sheet
                .call_method0("__copy__")
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(copied_dunder.name, sheet.name);

            // test __deepcopy__
            let memo = pyo3::types::PyDict::new(py);
            let deep_copied: Sheet = bound_sheet
                .call_method1("__deepcopy__", (memo,))
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(deep_copied.name, sheet.name);
        });
    }

    #[test]
    fn test_search_and_drop() {
        let wb = load_workbook_impl("tests/data/sample.xlsx").unwrap();
        let sheet = wb.get_sheet("simple").unwrap();

        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            // Test 1: exact match with plain string, drop direction "top"
            let mut test_sheet = sheet.clone();
            let query = "DEF".into_pyobject(py).unwrap().into_any();
            let coords = test_sheet.search_and_drop(py, &query, "top").unwrap();
            assert_eq!(coords, ((2, 0), (0, 0)));
            assert_eq!(test_sheet.shape(), (3, 3));
            assert_eq!(test_sheet.data[0][0], CellValue::String("DEF".to_string()));

            // Test 2: exact match with plain string, drop direction "bottom"
            let mut test_sheet = sheet.clone();
            let query = "DEF".into_pyobject(py).unwrap().into_any();
            let coords = test_sheet.search_and_drop(py, &query, "bottom").unwrap();
            assert_eq!(coords, ((2, 0), (2, 0)));
            assert_eq!(test_sheet.shape(), (3, 3));
            assert_eq!(test_sheet.data[2][0], CellValue::String("DEF".to_string()));

            // Test 3: exact match with plain string, drop direction "left"
            let mut test_sheet = sheet.clone();
            let query = "Header #2".into_pyobject(py).unwrap().into_any();
            let coords = test_sheet.search_and_drop(py, &query, "left").unwrap();
            assert_eq!(coords, ((0, 1), (0, 0)));
            assert_eq!(test_sheet.shape(), (5, 2));
            assert_eq!(
                test_sheet.data[0][0],
                CellValue::String("Header #2".to_string())
            );

            // Test 4: exact match with plain string, drop direction "right"
            let mut test_sheet = sheet.clone();
            let query = "Header #2".into_pyobject(py).unwrap().into_any();
            let coords = test_sheet.search_and_drop(py, &query, "right").unwrap();
            assert_eq!(coords, ((0, 1), (0, 1)));
            assert_eq!(test_sheet.shape(), (5, 2));
            assert_eq!(
                test_sheet.data[0][1],
                CellValue::String("Header #2".to_string())
            );

            // Test 5: diagonal direction "top_left"
            let mut test_sheet = sheet.clone();
            let query = "Alice".into_pyobject(py).unwrap().into_any();
            let coords = test_sheet.search_and_drop(py, &query, "top_left").unwrap();
            assert_eq!(coords, ((1, 2), (0, 0)));
            assert_eq!(test_sheet.shape(), (4, 1));
            assert_eq!(
                test_sheet.data[0][0],
                CellValue::String("Alice".to_string())
            );

            // Test 6: Python regex match (using re.compile)
            let re = py.import("re").unwrap();
            let pattern = re.call_method1("compile", ("^[D-F]{3}$",)).unwrap();
            let mut test_sheet = sheet.clone();
            let coords = test_sheet.search_and_drop(py, &pattern, "top").unwrap();
            assert_eq!(coords, ((2, 0), (0, 0)));
            assert_eq!(test_sheet.shape(), (3, 3));
            assert_eq!(test_sheet.data[0][0], CellValue::String("DEF".to_string()));

            // Test 7: invalid direction error
            let mut test_sheet = sheet.clone();
            let query = "DEF".into_pyobject(py).unwrap().into_any();
            let res = test_sheet.search_and_drop(py, &query, "invalid_dir");
            assert!(res.is_err());

            // Test 8: search term not found error
            let mut test_sheet = sheet.clone();
            let query = "NOT_FOUND".into_pyobject(py).unwrap().into_any();
            let res = test_sheet.search_and_drop(py, &query, "top");
            assert!(res.is_err());
        });
    }

    #[test]
    fn test_search_range() {
        use crate::matcher::{CellMatchRule, CellPattern, RangeMatcher, RowPattern};

        let wb = load_workbook_impl("tests/data/sample.xlsx").unwrap();
        let sheet = wb.get_sheet("simple").unwrap();

        pyo3::Python::initialize();
        pyo3::Python::attach(|py| {
            // Build a matcher for:
            // RowPattern: non_empty, any, any (matches header rows)
            // RowPattern: "ABC", any, any (matches ABC data row)

            let mut pattern1 = RowPattern::new();
            pattern1.cell_patterns.push(CellPattern {
                rule: CellMatchRule::NonEmpty,
                min: 1,
                max: Some(1),
            });
            pattern1.cell_patterns.push(CellPattern {
                rule: CellMatchRule::Any,
                min: 1,
                max: Some(1),
            });
            pattern1.cell_patterns.push(CellPattern {
                rule: CellMatchRule::Any,
                min: 1,
                max: Some(1),
            });

            let mut pattern2 = RowPattern::new();
            pattern2.cell_patterns.push(CellPattern {
                rule: CellMatchRule::Exact("ABC".to_string()),
                min: 1,
                max: Some(1),
            });
            pattern2.cell_patterns.push(CellPattern {
                rule: CellMatchRule::Any,
                min: 1,
                max: Some(1),
            });
            pattern2.cell_patterns.push(CellPattern {
                rule: CellMatchRule::Any,
                min: 1,
                max: Some(1),
            });

            let mut matcher = RangeMatcher::new();
            matcher.row_patterns.push(pattern1);
            matcher.row_patterns.push(pattern2);

            // Test search on entire sheet
            let range = sheet
                .search_range(py, &matcher, None, None, None, None)
                .unwrap()
                .unwrap();
            assert_eq!(range.start_row, 0);
            assert_eq!(range.end_row, 1);
            assert_eq!(range.start_col, 0);
            assert_eq!(range.end_col, 2);

            // Test search with start_row boundary that excludes the header row
            let range_opt = sheet
                .search_range(py, &matcher, Some(1), None, None, None)
                .unwrap();
            assert!(range_opt.is_none());

            // Test search out of bounds
            let err = sheet.search_range(py, &matcher, Some(-1), None, None, None);
            assert!(err.is_err());

            let err2 = sheet.search_range(py, &matcher, Some(10), None, None, None);
            assert!(err2.is_err());

            // This slice restricts column width to 2 cols (0 and 1).
            // But pattern1 expects 3 cell patterns. So it shouldn't match.
            let range_col_restricted = sheet
                .search_range(py, &matcher, None, None, Some(0), Some(1))
                .unwrap();
            assert!(range_col_restricted.is_none());
        });
    }

    #[test]
    fn test_get_range_between() {
        use crate::matcher::Range;

        let wb = load_workbook_impl("tests/data/sample.xlsx").unwrap();
        let sheet = wb.get_sheet("simple").unwrap();

        pyo3::Python::initialize();
        pyo3::Python::attach(|_py| {
            // 1. Vertical separation test
            let r1 = Range {
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: 2,
            };
            let r2 = Range {
                start_row: 4,
                end_row: 4,
                start_col: 0,
                end_col: 2,
            };
            let res = sheet.get_range_between(&r1, &r2).unwrap();
            assert_eq!(res.start_row, 1);
            assert_eq!(res.end_row, 3);
            assert_eq!(res.start_col, 0);
            assert_eq!(res.end_col, 2);

            // 2. Vertical separation mismatch column span error
            let r1_mismatch = Range {
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: 1,
            };
            assert!(sheet.get_range_between(&r1_mismatch, &r2).is_err());

            // 3. Horizontal separation test
            let rh1 = Range {
                start_row: 1,
                end_row: 3,
                start_col: 0,
                end_col: 0,
            };
            let rh2 = Range {
                start_row: 1,
                end_row: 3,
                start_col: 2,
                end_col: 2,
            };
            let res_h = sheet.get_range_between(&rh1, &rh2).unwrap();
            assert_eq!(res_h.start_row, 1);
            assert_eq!(res_h.end_row, 3);
            assert_eq!(res_h.start_col, 1);
            assert_eq!(res_h.end_col, 1);

            // 4. Horizontal separation mismatch row span error
            let mismatched_row_span = Range {
                start_row: 1,
                end_row: 2,
                start_col: 0,
                end_col: 0,
            };
            assert!(sheet.get_range_between(&mismatched_row_span, &rh2).is_err());

            // 5. Diagonal separation error
            let r_diag1 = Range {
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: 0,
            };
            let r_diag2 = Range {
                start_row: 2,
                end_row: 2,
                start_col: 2,
                end_col: 2,
            };
            assert!(sheet.get_range_between(&r_diag1, &r_diag2).is_err());

            // 6. Overlap error
            assert!(sheet.get_range_between(&r1, &r1).is_err());
        });
    }
}
