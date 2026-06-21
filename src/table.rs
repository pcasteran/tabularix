use pyo3::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;

use arrow::array::{
    ArrayRef, BooleanBuilder, Float64Builder, Int64Builder, StringBuilder, StructArray,
};
use arrow::datatypes::{DataType, Field, Fields, Schema, SchemaRef};
use arrow::record_batch::RecordBatch;

use crate::matcher::Range;
use crate::sheet::{CellValue, Sheet};

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct Table {
    pub(crate) schema: SchemaRef,
    pub(crate) batches: Vec<RecordBatch>,
}

#[pymethods]
impl Table {
    #[getter]
    pub fn shape(&self) -> (usize, usize) {
        let rows = self.batches.iter().map(RecordBatch::num_rows).sum();
        let cols = self.schema.fields().len();
        (rows, cols)
    }

    #[getter]
    pub fn columns(&self) -> Vec<String> {
        self.schema
            .fields()
            .iter()
            .map(|f| f.name().clone())
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "Table(columns={:?}, shape={:?})",
            self.columns(),
            self.shape()
        )
    }

    fn __arrow_c_stream__<'py>(
        &self,
        py: Python<'py>,
        _requested_schema: Option<Bound<'py, pyo3::types::PyCapsule>>,
    ) -> PyResult<Bound<'py, pyo3::types::PyCapsule>> {
        use arrow::ffi_stream::FFI_ArrowArrayStream;
        use arrow::record_batch::RecordBatchIterator;
        use pyo3::types::PyCapsule;

        let iterator = RecordBatchIterator::new(
            self.batches.clone().into_iter().map(Ok),
            self.schema.clone(),
        );
        let reader: Box<dyn arrow::record_batch::RecordBatchReader + Send> = Box::new(iterator);

        let stream = FFI_ArrowArrayStream::new(reader);

        let capsule = PyCapsule::new_with_value_and_destructor(
            py,
            stream,
            c"arrow_array_stream",
            |mut stream_val, _ctx| {
                if let Some(release) = stream_val.release {
                    unsafe {
                        release(&raw mut stream_val);
                    }
                }
            },
        )?;
        Ok(capsule)
    }
}

fn validate_ranges(sheet: &Sheet, data: &Range, header: Option<&Range>) -> PyResult<()> {
    let rows_count = sheet.data.len();
    let cols_count = if rows_count > 0 {
        sheet.data[0].len()
    } else {
        0
    };

    if data.start_row > data.end_row || data.start_col > data.end_col {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Invalid data range bounds.",
        ));
    }
    if data.end_row >= rows_count || data.end_col >= cols_count {
        return Err(pyo3::exceptions::PyIndexError::new_err(
            "Data range exceeds sheet dimensions.",
        ));
    }

    if let Some(h) = header {
        if h.start_row > h.end_row || h.start_col > h.end_col {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid header range bounds.",
            ));
        }
        if h.end_row >= rows_count || h.end_col >= cols_count {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Header range exceeds sheet dimensions.",
            ));
        }
        if h.start_col != data.start_col || h.end_col != data.end_col {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Header and data ranges do not align horizontally (column spans differ).",
            ));
        }
        if h.end_row >= data.start_row {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Header range overlaps with or is positioned below the data range.",
            ));
        }
    }
    Ok(())
}

impl Table {
    pub fn extract_from_sheet(
        sheet: &Sheet,
        data: &Range,
        header: Option<&Range>,
        clean_names: bool,
        flatten_header: bool,
        header_separator: &str,
    ) -> PyResult<Self> {
        validate_ranges(sheet, data, header)?;

        let (fields, arrays) = if let Some(h) = header {
            if flatten_header || h.start_row == h.end_row {
                let mut resolved_fields = Vec::new();
                let mut resolved_arrays = Vec::new();
                let mut seen = HashSet::new();

                for c in data.start_col..=data.end_col {
                    let raw_name = if h.start_row == h.end_row {
                        sheet
                            .get_merged_cell_value(h.start_row, c)
                            .to_string_for_search()
                    } else {
                        (h.start_row..=h.end_row)
                            .map(|r| sheet.get_merged_cell_value(r, c).to_string_for_search())
                            .collect::<Vec<_>>()
                            .join(header_separator)
                    };

                    let mut base_name = if clean_names {
                        clean_name(&raw_name)
                    } else {
                        raw_name.trim().to_string()
                    };

                    if base_name.is_empty() {
                        base_name = format!("column_{}", c - data.start_col + 1);
                    }

                    let mut name = base_name.clone();
                    let mut suffix = 1;
                    while seen.contains(&name) {
                        name = format!("{base_name}_{suffix}");
                        suffix += 1;
                    }
                    seen.insert(name.clone());

                    let (datatype, array) =
                        build_flat_array(sheet, data.start_row, data.end_row, c);
                    resolved_fields.push(Field::new(name, datatype, true));
                    resolved_arrays.push(array);
                }

                (resolved_fields, resolved_arrays)
            } else {
                let ctx = ExtractionContext {
                    sheet,
                    header: h,
                    data_start_row: data.start_row,
                    data_end_row: data.end_row,
                    data_start_col: data.start_col,
                    clean_names,
                };
                build_nested_fields_and_arrays(&ctx, h.start_row, data.start_col, data.end_col)?
            }
        } else {
            let mut resolved_fields = Vec::new();
            let mut resolved_arrays = Vec::new();
            for c in data.start_col..=data.end_col {
                let name = format!("column_{}", c - data.start_col + 1);
                let (datatype, array) = build_flat_array(sheet, data.start_row, data.end_row, c);
                resolved_fields.push(Field::new(name, datatype, true));
                resolved_arrays.push(array);
            }
            (resolved_fields, resolved_arrays)
        };

        let schema = Arc::new(Schema::new(fields));
        let batch = RecordBatch::try_new(schema.clone(), arrays)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok(Table {
            schema,
            batches: vec![batch],
        })
    }
}

fn clean_name(name: &str) -> String {
    let mut cleaned = String::new();
    let mut prev_was_underscore = false;

    for c in name.chars() {
        if c.is_alphanumeric() {
            cleaned.push(c.to_ascii_lowercase());
            prev_was_underscore = false;
        } else if !prev_was_underscore {
            cleaned.push('_');
            prev_was_underscore = true;
        }
    }

    let mut s = cleaned.as_str();
    while s.starts_with('_') {
        s = &s[1..];
    }
    while s.ends_with('_') {
        s = &s[..s.len() - 1];
    }
    s.to_string()
}

#[allow(clippy::cast_precision_loss)]
fn build_flat_array(
    sheet: &Sheet,
    start_row: usize,
    end_row: usize,
    col: usize,
) -> (DataType, ArrayRef) {
    let mut cells = Vec::new();
    for r in start_row..=end_row {
        cells.push(sheet.get_merged_cell_value(r, col));
    }

    let mut has_string = false;
    let mut has_float = false;
    let mut has_int = false;
    let mut has_bool = false;

    for cell in &cells {
        match cell {
            CellValue::String(_) => has_string = true,
            CellValue::Float(_) => has_float = true,
            CellValue::Int(_) => has_int = true,
            CellValue::Bool(_) => has_bool = true,
            CellValue::Empty | CellValue::Error(_) => {}
        }
    }

    let datatype = if has_string || (has_bool && (has_int || has_float)) {
        DataType::Utf8
    } else if has_bool {
        DataType::Boolean
    } else if has_float {
        DataType::Float64
    } else if has_int {
        DataType::Int64
    } else {
        DataType::Utf8
    };

    let array: ArrayRef = match datatype {
        DataType::Int64 => {
            let mut builder = Int64Builder::new();
            for cell in cells {
                match cell {
                    CellValue::Int(v) => builder.append_value(*v),
                    _ => builder.append_null(),
                }
            }
            Arc::new(builder.finish())
        }
        DataType::Float64 => {
            let mut builder = Float64Builder::new();
            for cell in cells {
                match cell {
                    CellValue::Float(v) => builder.append_value(*v),
                    CellValue::Int(v) => builder.append_value(*v as f64),
                    _ => builder.append_null(),
                }
            }
            Arc::new(builder.finish())
        }
        DataType::Boolean => {
            let mut builder = BooleanBuilder::new();
            for cell in cells {
                match cell {
                    CellValue::Bool(v) => builder.append_value(*v),
                    _ => builder.append_null(),
                }
            }
            Arc::new(builder.finish())
        }
        DataType::Utf8 => {
            let mut builder = StringBuilder::new();
            for cell in cells {
                match cell {
                    CellValue::String(v) => builder.append_value(v),
                    CellValue::Int(v) => builder.append_value(v.to_string()),
                    CellValue::Float(v) => builder.append_value(v.to_string()),
                    CellValue::Bool(v) => builder.append_value(if *v { "TRUE" } else { "FALSE" }),
                    _ => builder.append_null(),
                }
            }
            Arc::new(builder.finish())
        }
        _ => unreachable!(),
    };

    (datatype, array)
}

struct ExtractionContext<'a> {
    sheet: &'a Sheet,
    header: &'a Range,
    data_start_row: usize,
    data_end_row: usize,
    data_start_col: usize,
    clean_names: bool,
}

fn build_nested_fields_and_arrays(
    ctx: &ExtractionContext<'_>,
    cur_header_row: usize,
    col_start: usize,
    col_end: usize,
) -> PyResult<(Vec<Field>, Vec<ArrayRef>)> {
    if cur_header_row == ctx.header.end_row {
        let mut fields = Vec::new();
        let mut arrays = Vec::new();
        let mut seen = HashSet::new();

        for c in col_start..=col_end {
            let raw_name = ctx
                .sheet
                .get_merged_cell_value(cur_header_row, c)
                .to_string_for_search();
            let mut base_name = if ctx.clean_names {
                clean_name(&raw_name)
            } else {
                raw_name.trim().to_string()
            };

            if base_name.is_empty() {
                base_name = format!("column_{}", c - ctx.data_start_col + 1);
            }

            let mut name = base_name.clone();
            let mut suffix = 1;
            while seen.contains(&name) {
                name = format!("{base_name}_{suffix}");
                suffix += 1;
            }
            seen.insert(name.clone());

            let (datatype, array) =
                build_flat_array(ctx.sheet, ctx.data_start_row, ctx.data_end_row, c);
            fields.push(Field::new(name, datatype, true));
            arrays.push(array);
        }

        Ok((fields, arrays))
    } else {
        let mut fields = Vec::new();
        let mut arrays: Vec<ArrayRef> = Vec::new();
        let mut seen = HashSet::new();

        let mut c_start = col_start;
        while c_start <= col_end {
            let raw_name = ctx
                .sheet
                .get_merged_cell_value(cur_header_row, c_start)
                .to_string_for_search();

            let mut c_end = c_start;
            while c_end < col_end
                && ctx
                    .sheet
                    .get_merged_cell_value(cur_header_row, c_end + 1)
                    .to_string_for_search()
                    == raw_name
            {
                c_end += 1;
            }

            let mut base_name = if ctx.clean_names {
                clean_name(&raw_name)
            } else {
                raw_name.trim().to_string()
            };

            if base_name.is_empty() {
                base_name = format!("group_{}", c_start - ctx.data_start_col + 1);
            }

            let mut name = base_name.clone();
            let mut suffix = 1;
            while seen.contains(&name) {
                name = format!("{base_name}_{suffix}");
                suffix += 1;
            }
            seen.insert(name.clone());

            let (child_fields, child_arrays) =
                build_nested_fields_and_arrays(ctx, cur_header_row + 1, c_start, c_end)?;

            let struct_array =
                StructArray::try_new(Fields::from(child_fields.clone()), child_arrays, None)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

            fields.push(Field::new(
                name,
                DataType::Struct(Fields::from(child_fields)),
                true,
            ));
            arrays.push(Arc::new(struct_array));

            c_start = c_end + 1;
        }

        Ok((fields, arrays))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sheet::load_workbook_impl;

    #[test]
    fn test_clean_name() {
        assert_eq!(clean_name("Header #2"), "header_2");
        assert_eq!(clean_name("Sales Amount ($)"), "sales_amount");
        assert_eq!(clean_name("??"), "");
    }

    #[test]
    fn test_extract_table_validation() {
        let wb = load_workbook_impl("tests/data/sample.xlsx").unwrap();
        let sheet = wb.get_sheet("simple").unwrap();

        pyo3::Python::initialize();
        pyo3::Python::attach(|_| {
            // Out of bounds data range
            let bad_data = Range {
                start_row: 0,
                end_row: 10,
                start_col: 0,
                end_col: 2,
            };
            assert!(Table::extract_from_sheet(&sheet, &bad_data, None, false, false, "_").is_err());

            // Out of bounds header range
            let data = Range {
                start_row: 1,
                end_row: 3,
                start_col: 0,
                end_col: 2,
            };
            let bad_header = Range {
                start_row: 0,
                end_row: 10,
                start_col: 0,
                end_col: 2,
            };
            assert!(
                Table::extract_from_sheet(&sheet, &data, Some(&bad_header), false, false, "_")
                    .is_err()
            );

            // Alignment mismatch (columns don't match)
            let misaligned_header = Range {
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: 1,
            };
            assert!(Table::extract_from_sheet(
                &sheet,
                &data,
                Some(&misaligned_header),
                false,
                false,
                "_"
            )
            .is_err());

            // Overlapping range (header row overlaps data row)
            let overlapping_header = Range {
                start_row: 0,
                end_row: 1,
                start_col: 0,
                end_col: 2,
            };
            assert!(Table::extract_from_sheet(
                &sheet,
                &data,
                Some(&overlapping_header),
                false,
                false,
                "_"
            )
            .is_err());
        });
    }

    #[test]
    fn test_extract_table_success() {
        let wb = load_workbook_impl("tests/data/sample.xlsx").unwrap();
        let sheet = wb.get_sheet("simple").unwrap();

        pyo3::Python::initialize();
        pyo3::Python::attach(|_| {
            let data = Range {
                start_row: 1,
                end_row: 2,
                start_col: 0,
                end_col: 2,
            };
            let header = Range {
                start_row: 0,
                end_row: 0,
                start_col: 0,
                end_col: 2,
            };

            // Build with header, clean_names = true
            let table =
                Table::extract_from_sheet(&sheet, &data, Some(&header), true, false, "_").unwrap();
            assert_eq!(table.shape(), (2, 3));
            assert_eq!(table.columns(), vec!["header_1", "header_2", "header_3"]);

            // Build with header, clean_names = false
            let table_raw =
                Table::extract_from_sheet(&sheet, &data, Some(&header), false, false, "_").unwrap();
            assert_eq!(
                table_raw.columns(),
                vec!["Header #1", "Header #2", "Header #3"]
            );

            // Build without header
            let table_no_hdr =
                Table::extract_from_sheet(&sheet, &data, None, false, false, "_").unwrap();
            assert_eq!(
                table_no_hdr.columns(),
                vec!["column_1", "column_2", "column_3"]
            );
        });
    }

    #[test]
    fn test_extract_table_multi_row() {
        let sheet = Sheet {
            name: "multi_row".to_string(),
            data: vec![
                vec![
                    CellValue::String("2026".to_string()),
                    CellValue::Empty,
                    CellValue::String("2027".to_string()),
                    CellValue::Empty,
                ],
                vec![
                    CellValue::String("Q1".to_string()),
                    CellValue::String("Q2".to_string()),
                    CellValue::String("Q1".to_string()),
                    CellValue::String("Q2".to_string()),
                ],
                vec![
                    CellValue::Int(10),
                    CellValue::Int(20),
                    CellValue::Int(30),
                    CellValue::Int(40),
                ],
                vec![
                    CellValue::Int(15),
                    CellValue::Int(25),
                    CellValue::Int(35),
                    CellValue::Int(45),
                ],
            ],
            merged_regions: vec![((0, 0), (0, 1)), ((0, 2), (0, 3))],
        };

        pyo3::Python::initialize();
        pyo3::Python::attach(|_| {
            let data = Range {
                start_row: 2,
                end_row: 3,
                start_col: 0,
                end_col: 3,
            };
            let header = Range {
                start_row: 0,
                end_row: 1,
                start_col: 0,
                end_col: 3,
            };

            // Test multi-row flattened header
            let table_flat =
                Table::extract_from_sheet(&sheet, &data, Some(&header), true, true, "_").unwrap();
            assert_eq!(table_flat.shape(), (2, 4));
            assert_eq!(
                table_flat.columns(),
                vec!["2026_q1", "2026_q2", "2027_q1", "2027_q2"]
            );

            // Test multi-row nested header
            let table_nested =
                Table::extract_from_sheet(&sheet, &data, Some(&header), true, false, "_").unwrap();
            assert_eq!(table_nested.shape(), (2, 2));
            assert_eq!(table_nested.columns(), vec!["2026", "2027"]);

            let fields = table_nested.schema.fields();
            assert_eq!(fields[0].name(), "2026");
            match fields[0].data_type() {
                arrow::datatypes::DataType::Struct(subfields) => {
                    assert_eq!(subfields.len(), 2);
                    assert_eq!(subfields[0].name(), "q1");
                    assert_eq!(subfields[1].name(), "q2");
                }
                _ => panic!("Expected Struct data type"),
            }
        });
    }
}
