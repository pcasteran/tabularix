# Tabularix Usage & Strategy Guidelines

This document compiles the best practices, matching patterns, and data engineering strategies for using the Tabularix framework to extract structured data from complex spreadsheets.

---

## 🧭 Resilient Extraction Philosophy

- **NEVER Use Fixed Coordinates:** Avoid hardcoding cell addresses (e.g., `C9:D10`, `B12:F15`) or raw row/column offsets. Grid layouts change frequently when worksheets are updated.
- **Rely on Framework Matching Capabilities:** Define layout patterns declaratively using `RangePattern1D` and `RangePattern2D` composed of `CellRule` instances. Then compile them into directional `RangeMatcher`s to locate sheet boundaries.

---

## 🔍 Resilient Metadata Extraction Strategy

When a worksheet contains key-value metadata scattered outside the main tables (often stored horizontally in columns, e.g. keys in column B, values in column C):

1. **Define Vertical Matchers:** Define a vertical pattern for the header column matching keys, and a vertical pattern for the data column to the right. Both should use vertical directions (`TB` or `BT`):

    ```python
    from tabularix import RangePattern1D, regex, non_empty

    # Vertical header matcher.
    header_pattern = RangePattern1D([regex(r"^(Date|Fiscal Year)$").repeat(2, 2)])
    header_matcher = header_pattern.to_matcher(direction="TB")

    # Vertical data matcher.
    data_pattern = RangePattern1D([non_empty().repeat(2, 2)])
    data_matcher = data_pattern.to_matcher(direction="TB")
    ```

2. **Dynamic Relative Search:** Find the header range first, then locate the data range to the right of the header range using relative boundaries (`right=header_range`):
    ```python
    header_range = sheet.search_range(header_matcher)
    data_range = sheet.search_range_relative(data_matcher, right=header_range)
    ```
3. **Natively Transposed Extraction:** Extract the metadata by passing the data range and the header range to `extract_table(data_range, header_range)`. Tabularix will automatically pivot the key-value sequence into a single-row DataFrame, making Polars transpose operations obsolete:
    ```python
    table = sheet.extract_table(data_range, header_range)
    df = pl.from_arrow(table.to_arrow())
    ```

---

## 🔄 Projecting Metadata into Combined Data

When you have a single-row DataFrame containing global metadata (e.g., date, fiscal year) and a larger combined DataFrame containing row-level data (e.g., territory tables), you can project/broadcast the metadata onto all data rows:

- **Broadcasting via Cross Join:** A Polars `cross` join is the most dynamic, schema-agnostic way to append the metadata columns to every row of the target data, avoiding the need to hardcode columns or assign literals manually:
    ```python
    projected_df = territories_df.join(metadata_df, how="cross")
    ```

---

## 🔄 Pivoting Hierarchical Headers Strategy

When tables contain hierarchical headers (e.g. nested headers with products and years), we can extract them natively as Arrow structs and flatten them dynamically:

1. **Disable Header Flattening:** Set `flatten_header=False` in `sheet.extract_table()`. This prevents columns from being flattened into concatenated strings (e.g. `2025_expected`). Instead, columns are exported as nested Arrow structs (e.g. a struct column named `2025` with fields `expected` and `actual`).
2. **Unpack Root Structs:** If any root-level columns are structs (e.g. `product` column containing a nested field `product`), unpack them to clean scalar columns:
    ```python
    df = df.with_columns(pl.col("product").struct.field("product"))
    ```
3. **Dynamic Unpivot (Melt) with `on=None`:** Use Polars' `unpivot()` with `on=None`. Polars will automatically select all columns not specified in the `index` argument to be unpivoted to rows. This naturally normalizes differing/dynamic sets of columns (e.g., years) into a standard key-value row format:
    ```python
    df = df.unpivot(
        on=None,
        index=["territory", "product"],
        variable_name="year",
        value_name="metrics",
    )
    ```
4. **Unnest Nested Structs:** Unnest the melted value column struct to expand its fields back into separate columns:
    ```python
    df = df.unnest("metrics")
    ```

This approach automatically standardizes heterogeneous year/metric columns across multiple sub-tables to ensure that `pl.concat(dfs)` succeeds natively without requiring any schema alignment or manual column unions.

---

## 🥞 Stacked Table Scanning Strategy

When multiple tables are stacked vertically in a single sheet:

1. **Use a Dynamic Scanning Loop:** Use a tracking variable `search_row` (initialized to `0`) and loop while `search_row < sheet.shape[0]` to prevent `IndexError: start_row out of bounds` at the end of the sheet.
2. **Anchor-Based Search:** Locate the section title (e.g., territory name) using a 1D pattern compiled with a horizontal direction:
    ```python
    territory_pattern = RangePattern1D([regex(r"^(North|South|East|West)$")])
    territory_matcher = territory_pattern.to_matcher(direction="LR")
    ```
3. **Header & Footer Boundary Search:**
    - Compile a 2D pattern (outer flow `TB`, inner flow `LR`) to find multi-row headers:
        ```python
        header_pattern = RangePattern2D([row1_pattern, row2_pattern])
        header_matcher = header_pattern.to_matcher(outer_direction="TB", inner_direction="LR")
        ```
    - Find the header range below the title.
    - Find the footer range below the header.
    - Use `sheet.get_range_between(header_range, footer_range)` to calculate the `data_range` dynamically, eliminating manual row index math.

---

## 📏 Column Span & Width Alignment

- **Alignment Requirement:** Operations like `get_range_between` and `extract_table` require start and end ranges to align perfectly on the same columns.
- **Greedy Matchers for Dynamic Widths:** Use greedy cell repetitions like `any().zero_or_more()` to allow matched ranges to automatically expand horizontally up to the search area boundary.
- **Cell Group Matching:** Group repeating cell sequences (e.g. Year + Empty merged columns) using nested `RangePattern1D` instances combined with repetition rules:
    ```python
    # Matches a year cell followed by a merged/empty cell repeating zero or more times.
    RangePattern1D([
        regex(r"\d{4}"),
        empty()
    ]).zero_or_more()
    ```

---

## 🧪 Handling Empty Values and Formulas

- **Excel Formula Evaluation:** Excel formulas stored in workbooks without cached values are parsed by the Rust engine as `CellValue::Empty` (`None`).
- **Pattern Matching Rule:** To match formula cells or empty cells greedily, use `empty().zero_or_more()` or `any().zero_or_more()`. Do **not** use `non_empty().zero_or_more()`, as it will stop at the first empty/un-cached cell.

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
