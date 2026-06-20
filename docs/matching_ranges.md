---
title: Range Matching
description: Guide to the RangeMatcher API and the Layex matching engine.
icon: lucide/target
---

# 🎯 Matching Ranges

In many real-world spreadsheets, tables of interest are not nicely formatted or aligned. They might begin after arbitrary headers, contain multiline merged cells, or have variable numbers of columns and rows.

To reliably locate and extract these tables, **Tabularix** provides the **Layex Pattern Matching Engine**. **Layex** stands for **Layout Expression**, a concept and syntax derived from regular expressions but tailored for layout-level structures of spreadsheets. With Layex, you define a structural pattern of cell sequences (called a `RowPattern`) and row sequences (called a `RangeMatcher`) programmatically.

---

## 🏗️ Core Concepts

The layout matcher uses two primary builders:

1. **`RowPattern`**: Represents the expected horizontal sequence of cells in a single row. It is constructed using cell-matching rules (`value`, `regex`, `empty`, `non_empty`, `any`) and cell-level cardinalities (how many columns match this rule).
2. **`RangeMatcher`**: Represents the expected vertical sequence of rows. It compiles one or more `RowPattern`s together, along with row-level cardinalities (how many times a row or block of rows repeats).

---

## 🛠️ Top-Level Helper Functions

Tabularix exports top-level helper functions to cleanly start a new `RowPattern` in Python:

- `value(val)`: Matches cells with the exact string value `val`.
- `regex(pattern)`: Matches cells against a regular expression pattern (compiled or plain string).
- `empty()`: Matches blank or empty cells.
- `non_empty()`: Matches cells containing any non-empty value.
- `any()`: Matches any cell value (wildcard).

---

## 🔄 Cardinalities (Repetitions)

Both `RowPattern` (cells) and `RangeMatcher` (rows) support the same cardinality methods to control matches:

- `.one_or_more()`: Matches 1 or more times (regex `+`).
- `.zero_or_more()`: Matches 0 or more times (regex `*`).
- `.optional()`: Matches 0 or 1 time (regex `?`).
- `.repeat(min, max=None)`: Matches a custom range count. E.g. `.repeat(3)` (exactly 3) or `.repeat(1, 4)` (between 1 and 4).

<!-- prettier-ignore -->
!!! important "Cardinality Exclusivity"
    You can only configure a cardinality method once per cell or row pattern. Chaining multiple cardinalities (e.g. `.optional().one_or_more()`) will raise a `ValueError`.

---

## 📖 Usage Examples

### 1. Basic Exact Value & Regex Row Matching

For example, to match a tabular data row structured like this:

| Column A (Date) | Column B (Label) | Column C (Q1) | Column D (Q2) | Column E (Q3) | Column F (Q4) |
| :-------------- | :--------------- | :------------ | :------------ | :------------ | :------------ |
| `2026-06-19`    | `Sales Revenue`  | `$1250.00`    | `$1430.50`    | `$920.00`     | `$1780.75`    |

We define a single row pattern starting with a date (regex `^\d{4}-\d{2}-\d{2}$`), followed by a non-empty text label, followed by exactly 4 currency amount values:

```python linenums="1"
from tabularix import RangeMatcher, value, regex, non_empty

matcher = (
    RangeMatcher()
    .row(
        regex(r"^\d{4}-\d{2}-\d{2}$")   # 1 date cell
        .non_empty()                    # 1 description label
        .regex(r"^\$\d+(?:\.\d{2})?$").repeat(4)  # Exactly 4 currency amounts
    )
)
```

### 2. Multi-line Headers (Variable Columns)

Usually, spreadsheets have multiline headers where cells merge across columns. For example, to match a 2-line header sequence structured like this:

| Column A            | Column B   | Column C  | Column D   | Column E  |
| :------------------ | :--------- | :-------- | :--------- | :-------- |
| `Sales Report 2026` | `H1`       | _(empty)_ | `H2`       | _(empty)_ |
| `Product`           | `Forecast` | `Actual`  | `Forecast` | `Actual`  |

We can match this 2-line header sequence by combining different row patterns inside `RangeMatcher`:

```python linenums="1"
from tabularix import RangeMatcher, value, regex, empty

multiline_header_matcher = (
    RangeMatcher()
        # Header Row 1: The title block with half-year category headers
        .row(
            value("Sales Report 2026")
            .value("H1").empty()
            .value("H2").empty()
        )
        # Header Row 2: Product category followed by sub-columns
        .row(
            value("Product")
            .regex(r"^(Forecast|Actual)$").repeat(4)
        )
)
```

### 3. Check Row Matches Directly

You can check if list data matches a configured range pattern using `matches_range`:

```python linenums="1"
from tabularix import RangeMatcher, value, non_empty

matcher = (
    RangeMatcher()
    .row(
        value("Category")
        .non_empty().one_or_more()
    )
)

# Returns True (starts with "Category" followed by one or more non-empty cells)
print(matcher.matches_range([["Category", "A", "B", "C"]]))

# Returns False (does not match Category)
print(matcher.matches_range([["Total", 123]]))
```

---

## 🔍 Searching Worksheets

Once you have defined your `RangeMatcher` pattern, you can use it to search for matched regions within a `Sheet`. Tabularix supports two main methods for searching: absolute coordinate search and relative layout search. Both return a `Range` object enclosing the matched region boundaries, or `None` if no match is found.

### 📐 The `Range` Class

A successful search returns a `Range` object that represents the matched region:

- **Properties (All Inclusive)**:
    - `start_row`: The 0-based index of the first matched row.
    - `end_row`: The 0-based index of the last matched row.
    - `start_col`: The 0-based index of the first matched column.
    - `end_col`: The 0-based index of the last matched column.
- **String Representation**: `<Range rows=start_row..end_row, cols=start_col..end_col>`

---

### 📍 Absolute Search

Use `search_range` when you want to search the entire sheet or restrict the scanning area to a specific sub-grid using absolute indices:

```python
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
header_range = sheet.search_range(header_matcher)

# 2. Match the footer
footer_range = sheet.search_range(footer_matcher)

# 3. Search for the data rows in between the header and footer,
# inheriting the column span of the header/footer
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
