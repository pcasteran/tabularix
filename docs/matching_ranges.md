---
title: Range Matching
description: Guide to the Layex matching engine using group and grid patterns.
icon: lucide/target
---

# 🎯 Matching Ranges

In many real-world spreadsheets, tables of interest are not nicely formatted or aligned. They might begin after arbitrary headers, contain multiline merged cells, or have variable numbers of columns and rows.

To reliably locate and extract these tables, **Tabularix** provides the **Layex Pattern Matching Engine**. **Layex** stands for **Layout Expression**, a concept and syntax derived from regular expressions but tailored for layout-level structures of spreadsheets. With Layex, you define a structural pattern of cell sequences (called a `group`) and row sequences (called a `grid`) programmatically, which are then passed directly to the high-level extraction API or compiled into a `RangeMatcher` for advanced coordinate control.

---

## 🏗️ Core Concepts

The layout matcher uses two primary builders to construct patterns:

1. **`group`** (also known as `RangePattern1D`): Represents a one-dimensional sequence of cells or sub-groups. It is constructed using cell-matching rules (`value`, `regex`, `empty`, `non_empty`, `any`) and cell-level cardinalities (how many columns or cells match this rule).
2. **`grid`** (also known as `RangePattern2D`): Represents a two-dimensional range pattern. It compiles one or more `group` row patterns together, along with row-level cardinalities (how many times a row repeats).

When using the **High-Level API**, you pass these patterns directly to `extract_table_with_header_and_data` or `extract_table_between_header_and_footer` without needing to compile them manually.

---

## 🛠️ Top-Level Helper Functions

Tabularix exports top-level helper functions to cleanly construct cell rules inside `group` patterns:

- `value(val)`: Matches cells with the exact string value `val`.
- `regex(pattern)`: Matches cells against a regular expression pattern (compiled or plain string).
- `empty()`: Matches blank or empty cells.
- `non_empty()`: Matches cells containing any non-empty value.
- `any()`: Matches any cell value (wildcard).

---

## 🔄 Cardinalities (Repetitions)

Both cell rules inside `group` (horizontal columns) and row patterns inside `grid` (vertical rows) support the same cardinality methods to control matches:

- `.one_or_more()`: Matches 1 or more times (regex `+`).
- `.zero_or_more()`: Matches 0 or more times (regex `*`).
- `.optional()`: Matches 0 or 1 time (regex `?`).
- `.repeat(min, max=None)`: Matches a custom range count. E.g. `.repeat(3)` (exactly 3) or `.repeat(1, 4)` (between 1 and 4).

<!-- prettier-ignore -->
!!! important "Cardinality Exclusivity"
    You can only configure a cardinality method once per cell rule or row pattern. Chaining multiple cardinalities (e.g. `.optional().one_or_more()`) will raise a `ValueError`.

### 🪵 Greedy vs. Lazy Matching

By default, all repetition cardinalities in Tabularix (such as `.one_or_more()`, `.zero_or_more()`, `.optional()`, and `.repeat()`) match **greedily**.

When scanning a row, the matching engine will always attempt to match the **largest possible horizontal span of cells (columns)** that satisfies the layout pattern. If a larger span fails to satisfy the entire vertical sequence of row patterns, the engine backtracks and automatically tries progressively smaller widths until a full match is found.

#### Configuring Lazy Matching

If you want the matching engine to match the **smallest possible span** (lazy matching), you can pass `greedy=False` to any of the repetition methods on both cell rules and row patterns:

- `.one_or_more(greedy=False)`
- `.zero_or_more(greedy=False)`
- `.optional(greedy=False)`
- `.repeat(min, max=None, greedy=False)`

This allows configuration of greediness for both directions:

- **Horizontal Matching (Columns)**: Configured on cell rules in `group`. Controls how many columns are matched by repeating cell patterns.
- **Vertical Matching (Rows)**: Configured on row patterns in `grid`. Controls how many repeating rows are matched.

When lazy matching is enabled, the engine starts by evaluating the minimum required columns/rows and only expands to match more elements if the remainder of the matcher rules fail. This is useful when you want to avoid matching too wide of a range, such as when parsing sub-table headers or optional separators.

---

## 📖 Usage Examples

### 1. Basic Exact Value & Regex Row Matching

For example, to match a tabular data row structured like this:

| Column A (Date) | Column B (Label) | Column C (Q1) | Column D (Q2) | Column E (Q3) | Column F (Q4) |
| :-------------- | :--------------- | :------------ | :------------ | :------------ | :------------ |
| `2026-06-19`    | `Sales Revenue`  | `$1250.00`    | `$1430.50`    | `$920.00`     | `$1780.75`    |

We define a single row pattern starting with a date (regex `^\d{4}-\d{2}-\d{2}$`), followed by a non-empty text label, followed by exactly 4 currency amount values:

```python linenums="1"
from tabularix import group, regex, non_empty

# Define a 1D group pattern representing a row
pattern = group(
    regex(r"^\d{4}-\d{2}-\d{2}$"),              # 1 date cell
    non_empty(),                                # 1 description label
    regex(r"^\$\d+(?:\.\d{2})?$").repeat(4)     # Exactly 4 currency amounts
)
```

### 2. Multi-line Headers (Variable Columns)

Usually, spreadsheets have multiline headers where cells merge across columns. For example, to match a 2-line header sequence structured like this:

| Column A            | Column B   | Column C  | Column D   | Column E  |
| :------------------ | :--------- | :-------- | :--------- | :-------- |
| `Sales Report 2026` | `H1`       | _(empty)_ | `H2`       | _(empty)_ |
| `Product`           | `Forecast` | `Actual`  | `Forecast` | `Actual`  |

We can match this 2-line header sequence by combining different row patterns:

```python linenums="1"
from tabularix import grid, group, value, regex, empty

# Header Row 1: The title block with half-year category headers
row1 = group(
    value("Sales Report 2026"),
    value("H1"),
    empty(),
    value("H2"),
    empty()
)

# Header Row 2: Product category followed by sub-columns
row2 = group(
    value("Product"),
    regex(r"^(Forecast|Actual)$").repeat(4)
)

# Combine into a 2D grid pattern
pattern = grid(row1, row2)
```

### 3. Check Row Matches Directly

You can compile a pattern to a matcher to check if list data matches a configured pattern using `matches_range`:

```python linenums="1"
from tabularix import group, value, non_empty

pattern = group(
    value("Category"),
    non_empty().one_or_more()
)
matcher = pattern.to_matcher(direction="LR")

# Returns True (starts with "Category" followed by one or more non-empty cells)
print(matcher.matches_range([["Category", "A", "B", "C"]]))

# Returns False (does not match Category)
print(matcher.matches_range([["Total", 123]]))
```

---

## 🛠️ Low-Level Pattern Scanning (Advanced Control)

In most cases, you will pass your `group` and `grid` patterns directly to the High-Level API. However, if you need to inspect coordinates or scan regions manually, compile your patterns into a `RangeMatcher` and search the worksheet.

### 📐 The `Range` Class

A successful search returns a `Range` object that represents the matched region:

- **Properties**:
    - `start_row`: The 0-based index of the first matched row.
    - `end_row`: The 0-based index of the last matched row.
    - `start_col`: The 0-based index of the first matched column.
    - `end_col`: The 0-based index of the last matched column.
- **String Representation**: `Range(A2:C4, cols=start_col..end_col, rows=start_row..end_row)` (e.g. `Range(A1:C1, cols=0..2, rows=0..0)`)

---

### 📍 Absolute Search

Use `search_range` to scan the entire sheet or a specific sub-grid using a compiled `RangeMatcher`:

```python
from tabularix import load_workbook, group, value

wb = load_workbook("tests/data/sample.xlsx")
sheet = wb.get_sheet("simple")

pattern = group(value("Header #1"))
matcher = pattern.to_matcher(direction="LR")

# Search the entire sheet
matched_range = sheet.search_range(matcher)

# Search within a specific sub-grid (all bounds are inclusive)
matched_range = sheet.search_range(
    matcher,
    start_row=10,   # Start scanning from row 10
    end_row=100,    # Stop scanning at row 100
    start_col=2,    # Start scanning from column 2
    end_col=8,      # Stop scanning at column 8
)
```

<!-- prettier-ignore -->
!!! note "Bounds Checking"
    If any indices are out of bounds, an `IndexError` is raised. If `start_row > end_row` or `start_col > end_col`, a `ValueError` is raised.

---

### 🧩 Partial Column Matching

Unlike regular expressions that match a full row, Tabularix's matching engine matches a row **for the table to extract** rather than the entire worksheet row. This means `search_range` performs **partial column matching** to find a table anywhere horizontally within the worksheet.

For example, given a worksheet with 5 columns (A to E), if you define a 3-column `group` matching `"Header #1"`, `"Header #2"`, and `"Header #3"`, the layout engine will check only the valid 3-column sub-spans:

- `A:C` (columns 0 to 2)
- `B:D` (columns 1 to 3)
- `C:E` (columns 2 to 4)

If a match is found on one of these spans, the returned `Range` will enclose only those matched columns.

---

### 🔗 Relational Search

In many layout structures, tables are located relative to other landmarks (such as headers or footers) rather than at fixed indices. `search_range_relative` dynamically resolves coordinates and inherits boundaries from previously matched `Range` objects:

- `below=range`: Restricts the search vertically below `range.end_row + 1` and automatically inherits its column span (`start_col`, `end_col`).
- `above=range`: Restricts the search vertically above `range.start_row - 1` and inherits its column span.
- `right=range`: Restricts the search horizontally to the right of `range.end_col + 1` and inherits its row span (`start_row`, `end_row`).
- `left=range`: Restricts the search horizontally to the left of `range.start_col - 1` and inherits its row span.

#### Combining Boundaries

You can combine opposing boundaries (e.g. `below` and `above` to search in between, or `left` and `right`) to restrict the search region:

```python
# 1. Match the header first
header_matcher = header_pattern.to_matcher(direction="LR")
header_range = sheet.search_range(header_matcher)

# 2. Match the footer
footer_matcher = footer_pattern.to_matcher(direction="LR")
footer_range = sheet.search_range(footer_matcher)

# 3. Search for the data rows in between the header and footer,
# inheriting the column span of the header/footer
data_matcher = data_pattern.to_matcher(outer_direction="TB", inner_direction="LR")
data_range = sheet.search_range_relative(
    data_matcher,
    below=header_range,
    above=footer_range,
)
```

<!-- prettier-ignore -->
!!! important "Boundary Rules"
    - If opposing bounds cross (e.g. `below` is set to a range that is physically below the `above` range), a `ValueError` is raised.
    - If opposing bounds are specified, their spans must align (e.g. `below` and `above` must share the same column spans), otherwise a `ValueError` is raised.
