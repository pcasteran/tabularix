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

#[pyclass]
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

    pub fn cell(&self, py: Python<'_>, row: usize, col: usize) -> PyResult<PyObject> {
        if row >= self.data.len() || (!self.data.is_empty() && col >= self.data[0].len()) {
            return Err(pyo3::exceptions::PyIndexError::new_err("Out of bounds"));
        }
        let val = &self.data[row][col];
        let bound = val.clone().into_pyobject(py)?;
        Ok(bound.into_any().unbind())
    }

    pub fn to_svg(&self, path: &str) -> PyResult<()> {
        self.to_svg_impl(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write SVG: {e}")))
    }
}

impl Sheet {
    #[allow(clippy::too_many_lines)]
    fn to_svg_impl(&self, path: &str) -> std::io::Result<()> {
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
            svg.push_str("</svg>");
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
                    let display_str = if val_str.len() > max_chars && max_chars > 3 {
                        format!("{}...", &val_str[..max_chars - 3])
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
            let text_x = cell_x + cell_width / 2;
            let text_y = col_hdr_height / 2 + 4;
            let _ = writeln!(
                svg,
                r#"  <rect x="{cell_x}" y="0" width="{cell_width}" height="{col_hdr_height}" class="hdr-rect" />
  <text x="{text_x}" y="{text_y}" class="hdr-text">{letters}</text>"#
            );
        }

        for r in 0..rows {
            let cell_y = col_hdr_height + r * cell_height;
            let label = r + 1;
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

        svg.push_str("</svg>");

        std::fs::write(path, svg)?;
        Ok(())
    }
}

#[pyclass]
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

#[pyfunction]
pub fn index_to_a1(row: usize, col: usize) -> String {
    format!("{}{}", index_to_col_letters(col), row + 1)
}

#[pyfunction]
pub fn a1_to_index(a1: &str) -> PyResult<(usize, usize)> {
    let letters: String = a1.chars().take_while(char::is_ascii_alphabetic).collect();
    let numbers: String = a1.chars().skip_while(char::is_ascii_alphabetic).collect();

    if letters.is_empty() || numbers.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Invalid A1 notation: {a1}"
        )));
    }

    let row: usize = numbers.parse::<usize>().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err(format!("Invalid row number: {numbers}"))
    })?;
    if row == 0 {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Row index must be >= 1: {numbers}"
        )));
    }

    let mut col: usize = 0;
    for ch in letters.to_ascii_uppercase().chars() {
        let val = (ch as u8 - b'A' + 1) as usize;
        col = col * 26 + val;
    }
    Ok((row - 1, col - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_workbook() {
        let wb = load_workbook_impl("tests/data/sample.xlsx").unwrap();
        assert_eq!(wb.sheet_names(), vec!["Sheet1"]);

        let sheet = wb.get_sheet("Sheet1").unwrap();
        assert_eq!(sheet.name, "Sheet1");
        assert_eq!(sheet.shape(), (5, 2)); // 5 rows, 2 cols (A1 to B5)

        // Check cell values
        assert_eq!(sheet.data[0][0], CellValue::String("Header1".to_string()));
        assert_eq!(sheet.data[0][1], CellValue::String("Header2".to_string()));
        assert_eq!(sheet.data[1][0], CellValue::String("Row1Col1".to_string()));
        assert_eq!(sheet.data[1][1], CellValue::Float(123.45));
        assert_eq!(sheet.data[2][0], CellValue::String("Row2Col1".to_string()));
        assert_eq!(sheet.data[2][1], CellValue::Float(678.0));

        // Merged cell A4 is "MergedValue"
        assert_eq!(
            sheet.data[3][0],
            CellValue::String("MergedValue".to_string())
        );
        // B4, A5, B5 should be Empty in raw data because calamine does not automatically fill them
        assert_eq!(sheet.data[3][1], CellValue::Empty);
        assert_eq!(sheet.data[4][0], CellValue::Empty);
        assert_eq!(sheet.data[4][1], CellValue::Empty);

        // Merged regions check
        assert_eq!(sheet.merged_regions.len(), 1);
        assert_eq!(sheet.merged_regions[0], ((3, 0), (4, 1))); // A4:B5
    }

    #[test]
    fn test_coordinates() {
        assert_eq!(index_to_a1(0, 0), "A1");
        assert_eq!(index_to_a1(9, 25), "Z10");
        assert_eq!(index_to_a1(0, 26), "AA1");
        assert_eq!(index_to_a1(0, 27), "AB1");
        assert_eq!(index_to_a1(0, 701), "ZZ1");
        assert_eq!(index_to_a1(0, 702), "AAA1");

        assert_eq!(a1_to_index("A1").unwrap(), (0, 0));
        assert_eq!(a1_to_index("Z10").unwrap(), (9, 25));
        assert_eq!(a1_to_index("AA1").unwrap(), (0, 26));
        assert_eq!(a1_to_index("AB1").unwrap(), (0, 27));
        assert_eq!(a1_to_index("ZZ1").unwrap(), (0, 701));
        assert_eq!(a1_to_index("AAA1").unwrap(), (0, 702));

        assert!(a1_to_index("A").is_err());
        assert!(a1_to_index("1").is_err());
        assert!(a1_to_index("A0").is_err());
        assert!(a1_to_index("").is_err());
    }
}
