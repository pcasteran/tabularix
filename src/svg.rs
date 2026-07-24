use crate::matcher::Range;
use crate::sheet::{index_to_col_letters, CellValue, Sheet};
use std::collections::HashMap;
use std::fmt::Write;

pub fn export_sheet_to_svg(
    sheet: &Sheet,
    path: &str,
    zero_based_indices: bool,
    anonymise_ranges: Option<&[Range]>,
) -> std::io::Result<()> {
    let (rows, cols) = sheet.shape();

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

    render_svg_styles(&mut svg);

    if rows == 0 || cols == 0 {
        svg.push_str(r##"<rect width="100%" height="100%" fill="#f9fafb"/>"##);
        svg.push_str(r##"<text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" font-size="14" fill="#9ca3af">Empty Sheet</text>"##);
        svg.push_str("</svg>\n");
        std::fs::write(path, svg)?;
        return Ok(());
    }

    render_svg_data_cells(
        sheet,
        &mut svg,
        rows,
        cols,
        cell_width,
        cell_height,
        row_hdr_width,
        col_hdr_height,
        anonymise_ranges,
    );

    render_svg_headers(
        &mut svg,
        rows,
        cols,
        cell_width,
        cell_height,
        row_hdr_width,
        col_hdr_height,
        zero_based_indices,
    );

    svg.push_str("</svg>\n");

    std::fs::write(path, svg)?;
    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn render_svg_data_cells(
    sheet: &Sheet,
    svg: &mut String,
    rows: usize,
    cols: usize,
    cell_width: usize,
    cell_height: usize,
    row_hdr_width: usize,
    col_hdr_height: usize,
    anonymise_ranges: Option<&[Range]>,
) {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(123_456_789, |d| {
            d.as_secs().wrapping_add(u64::from(d.subsec_nanos()))
        });
    let mut string_placeholders = HashMap::<String, String>::new();

    svg.push_str("  <g class=\"data-cells\">\n");

    for r in 0..rows {
        for c in 0..cols {
            let mut is_merged = false;
            let mut draw_cell = true;
            let mut c_width = cell_width;
            let mut c_height = cell_height;
            let mut merged_start = (0, 0);
            let mut merged_end = (0, 0);

            for &(start, end) in &sheet.merged_regions {
                if r >= start.0 && r <= end.0 && c >= start.1 && c <= end.1 {
                    is_merged = true;
                    merged_start = start;
                    merged_end = end;
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

            let val = &sheet.data[r][c];

            let in_anonymise_range = if let Some(ranges) = anonymise_ranges {
                ranges.iter().any(|range| {
                    r >= range.start_row
                        && r <= range.end_row
                        && c >= range.start_col
                        && c <= range.end_col
                })
            } else {
                false
            };

            let mut cell_seed = seed.wrapping_add((r as u64) << 32).wrapping_add(c as u64);
            lcg(&mut cell_seed);

            let anonymised_holder;
            let val = if in_anonymise_range {
                anonymised_holder =
                    anonymise_cell_value(val, &mut cell_seed, &mut string_placeholders);
                &anonymised_holder
            } else {
                val
            };

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
                CellValue::String(_) | CellValue::Date(_) | CellValue::DateTime(_) => {
                    text_class.push_str("val-string");
                }
                CellValue::Float(_) | CellValue::Int(_) => {
                    text_class.push_str("val-number");
                }
                CellValue::Formula(_, _) => {
                    rect_class.push_str(" rect-formula");
                    text_class.push_str("val-formula");
                }
                CellValue::Empty => {}
            }

            let cell_range = if is_merged {
                let start_letter = index_to_col_letters(merged_start.1);
                let start_row = merged_start.0 + 1;
                let end_letter = index_to_col_letters(merged_end.1);
                let end_row = merged_end.0 + 1;
                format!("{start_letter}{start_row}:{end_letter}{end_row}")
            } else {
                let cell_letter = index_to_col_letters(c);
                let cell_row = r + 1;
                format!("{cell_letter}{cell_row}")
            };

            let _ = writeln!(
                svg,
                r#"    <rect x="{cell_x}" y="{cell_y}" width="{c_width}" height="{c_height}" class="{rect_class}" data-original-range="{cell_range}" />"#
            );

            let val_str = match val {
                CellValue::Empty => String::new(),
                CellValue::String(s) => s.clone(),
                CellValue::Float(f) => f.to_string(),
                CellValue::Int(i) => i.to_string(),
                CellValue::Bool(ref b) => {
                    if *b {
                        "TRUE".to_string()
                    } else {
                        "FALSE".to_string()
                    }
                }
                CellValue::Error(e) => format!("ERROR: {e}"),
                CellValue::Date(d) => d.to_string(),
                CellValue::DateTime(dt) => dt.format("%Y-%m-%dT%H:%M:%S%.f").to_string(),
                CellValue::Formula(_, _) => "<formula>".to_string(),
            };

            if !val_str.is_empty() {
                const DOUBLE_CHAR_WIDTH_PX: usize = 13;
                let max_chars = c_width * 2 / DOUBLE_CHAR_WIDTH_PX;
                let display_str = if val_str.chars().count() > max_chars && max_chars > 3 {
                    let mut truncated: String = val_str.chars().take(max_chars - 3).collect();
                    truncated.push_str("...");
                    truncated
                } else {
                    val_str
                };

                let text_x = match val {
                    CellValue::Float(_) | CellValue::Int(_) => cell_x + c_width - 8,
                    CellValue::Bool(_) | CellValue::Error(_) | CellValue::Formula(_, _) => {
                        cell_x + c_width / 2
                    }
                    _ => cell_x + 8,
                };
                let text_y = cell_y + c_height / 2 + 4;

                let escaped = html_escape(&display_str);
                let _ = writeln!(
                    svg,
                    r#"    <text x="{text_x}" y="{text_y}" class="{text_class}">{escaped}</text>"#
                );
            }
        }
    }

    svg.push_str("  </g>\n");
}

fn render_svg_styles(svg: &mut String) {
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
  .rect-formula {
    fill: #f3f4f6;
  }
  .val-formula {
    text-anchor: middle;
    fill: #9ca3af;
    font-style: italic;
  }
</style>
"#);
}

#[allow(clippy::too_many_arguments)]
fn render_svg_headers(
    svg: &mut String,
    rows: usize,
    cols: usize,
    cell_width: usize,
    cell_height: usize,
    row_hdr_width: usize,
    col_hdr_height: usize,
    zero_based_indices: bool,
) {
    svg.push_str("  <g class=\"headers\">\n");

    for c in 0..cols {
        let cell_x = row_hdr_width + c * cell_width;
        let letters = index_to_col_letters(c);
        let col_idx = if zero_based_indices { c } else { c + 1 };
        let col_label = format!("{letters} ({col_idx})");
        let text_x = cell_x + cell_width / 2;
        let text_y = col_hdr_height / 2 + 4;
        let _ = writeln!(
            svg,
            r#"    <rect x="{cell_x}" y="0" width="{cell_width}" height="{col_hdr_height}" class="hdr-rect" />
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
            r#"    <rect x="0" y="{cell_y}" width="{row_hdr_width}" height="{cell_height}" class="hdr-rect" />
    <text x="{text_x}" y="{text_y}" class="hdr-text">{label}</text>"#
        );
    }

    let _ = writeln!(
        svg,
        r#"    <rect x="0" y="0" width="{row_hdr_width}" height="{col_hdr_height}" class="hdr-rect" />"#
    );

    svg.push_str("  </g>\n");
}

fn lcg(seed: &mut u64) -> u64 {
    *seed = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *seed
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn anonymise_cell_value(
    val: &CellValue,
    cell_seed: &mut u64,
    string_placeholders: &mut HashMap<String, String>,
) -> CellValue {
    match val {
        CellValue::Int(i) => {
            let r_val = lcg(cell_seed);
            let r_val_32 = (r_val & 0xFFFF_FFFF) as u32;
            let factor = 0.1 + (f64::from(r_val_32) / f64::from(u32::MAX)) * 9.9;
            CellValue::Int(((*i as f64) * factor).round() as i64)
        }
        CellValue::Float(f) => {
            let r_val = lcg(cell_seed);
            let r_val_32 = (r_val & 0xFFFF_FFFF) as u32;
            let factor = 0.1 + (f64::from(r_val_32) / f64::from(u32::MAX)) * 9.9;
            let scaled = f * factor;

            let s = f.to_string();
            let decimal_places = if let Some(dot_idx) = s.find('.') {
                (s.len() - dot_idx - 1).min(6)
            } else {
                0
            };

            let rounded = if decimal_places > 0 {
                let multiplier = 10f64.powi(i32::try_from(decimal_places).unwrap_or(0));
                (scaled * multiplier).round() / multiplier
            } else {
                scaled.round()
            };

            CellValue::Float(rounded)
        }
        CellValue::String(s) => {
            let placeholder = if let Some(existing) = string_placeholders.get(s) {
                existing.clone()
            } else {
                let next_idx = string_placeholders.len() + 1;
                let placeholder = format!("Text_{next_idx}");
                string_placeholders.insert(s.clone(), placeholder.clone());
                placeholder
            };
            CellValue::String(placeholder)
        }
        CellValue::Date(d) => {
            let r_val = lcg(cell_seed);
            let days_offset = -365 + i64::try_from(r_val % 731).unwrap_or(0);
            let shifted = d
                .checked_add_signed(chrono::Duration::days(days_offset))
                .unwrap_or(*d);
            CellValue::Date(shifted)
        }
        CellValue::DateTime(dt) => {
            let r_val = lcg(cell_seed);
            let days_offset = -365 + i64::try_from(r_val % 731).unwrap_or(0);
            let shifted = dt
                .checked_add_signed(chrono::Duration::days(days_offset))
                .unwrap_or(*dt);
            CellValue::DateTime(shifted)
        }
        CellValue::Formula(f_str, inner) => {
            let anon_inner = anonymise_cell_value(inner, cell_seed, string_placeholders);
            CellValue::Formula(f_str.clone(), Box::new(anon_inner))
        }
        CellValue::Bool(b) => CellValue::Bool(*b),
        CellValue::Error(e) => CellValue::Error(e.clone()),
        CellValue::Empty => CellValue::Empty,
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
