# Tabularix Usage & Strategy Guidelines

This document compiles the best practices, matching patterns, and data engineering strategies for using the Tabularix framework to extract structured data from complex spreadsheets.

---

## 🧭 Resilient Extraction Philosophy

- **NEVER Use Fixed Coordinates:** Avoid hardcoding cell addresses (e.g., `C9:D10`, `B12:F15`) or raw row/column offsets. Grid layouts change frequently when worksheets are updated.
- **Rely on Framework Matching Capabilities:** Use dynamic anchors (using `RangeMatcher`, `RowPattern`, and relative queries like `search_range_relative` or `get_range_between`) to locate boundaries.

---

## 🔍 Resilient Metadata Extraction Strategy

When a worksheet contains key-value metadata scattered outside the main tables:

1. **Define a Structural Row Pattern:** Use a `RangeMatcher` matching exactly the keys of interest (using `regex`) and the adjacent value cell.
2. **Dynamic Horizontal Anchoring:** Match the exact row segment containing the key-value pair. Tabularix will return a range cropped horizontally to just those columns, removing the need to dynamically search for column offsets.
3. **Pivoting via Transpose:** Extract the range as a table (without headers) to get columns `column_1` (keys) and `column_2` (values). Convert to a Polars DataFrame, and pivot it into a single-row DataFrame using `.transpose(column_names="column_1")`.
4. **Static Type Casting:** Polars `from_arrow()` may infer a return type of `DataFrame | Series`. Cast the result to a `pl.DataFrame` before calling `.transpose()` to satisfy static type checkers:
    ```python
    from typing import cast
    df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))
    ```

---

## 🔄 Projecting Metadata into Combined Data

When you have a single-row DataFrame containing global metadata (e.g., date, fiscal year) and a larger combined DataFrame containing row-level data (e.g., territory tables), you can project/broadcast the metadata onto all data rows:

- **Broadcasting via Cross Join:** A Polars `cross` join is the most dynamic, schema-agnostic way to append the metadata columns to every row of the target data, avoiding the need to hardcode columns or assign literals manually:
    ```python
    projected_df = territories_df.join(metadata_df, how="cross")
    ```

---

## 🥞 Stacked Table Scanning Strategy

When multiple tables are stacked vertically in a single sheet:

1. **Use a Dynamic Scanning Loop:** Use a tracking variable `search_row` (initialized to `0`) and loop while `search_row < sheet.shape[0]` to prevent `IndexError: start_row out of bounds` at the end of the sheet.
2. **Anchor-Based Search:** Locate the section title (e.g., territory name) using a regex matcher. Use the title's row to start the search for the header.
3. **Header & Footer Boundary Search:**
    - Find the header range below the title.
    - Find the footer range below the header.
    - Use `sheet.get_range_between(header_range, footer_range)` to calculate the `data_range` dynamically, eliminating manual row index math.

---

## 📏 Column Span & Width Alignment

- **Alignment Requirement:** Operations like `get_range_between` and `extract_table` require start and end ranges to align perfectly on the same columns.
- **Greedy Matchers for Dynamic Widths:** Use greedy cell repetitions like `.zero_or_more()` to allow matched ranges to automatically expand horizontally up to the search area boundary (defined by relative search bounds).
- **Cell Group Matching:** Use `.group(...)` combined with `.zero_or_more()` to match repeated column pairs (e.g., Year + Empty merged columns, or Expected + Actual columns) dynamically.

---

## 🧪 Handling Empty Values and Formulas

- **Excel Formula Evaluation:** Excel formulas stored in workbooks without cached values are parsed by the Rust engine as `CellValue::Empty` (`None`).
- **Pattern Matching Rule:** To match formula cells or empty cells greedily, use `.empty().zero_or_more()` or `.any().zero_or_more()`. Do **not** use `.non_empty().zero_or_more()`, as it will stop at the first empty/un-cached cell.

---

## 📅 Date and DateTime Handling

- **Natively Supported Dates:** Tabularix natively maps `Date` and `DateTime` cells to Arrow `Date32` and `Timestamp` types.
- **Mixed-Type Conversions:** If a column has mixed types (e.g. metadata containing both date and text values), cells fall back to strings (`DataType::Utf8`) formatted in ISO 8601 (`YYYY-MM-DD`).
- **Polars Conversion:** Convert date string columns back to native date types using Polars' native parser:
    ```python
    df = df.with_columns(pl.col("Date").str.to_date())
    ```
- **Excel Serial Fallback (Classic):** If a date is parsed as a raw Excel float serial number, convert it to an ISO date string using date arithmetic:
    ```python
    import datetime
    base_date = datetime.date(1899, 12, 30)
    date_str = (base_date + datetime.timedelta(days=int(excel_float_value))).isoformat()
    ```
