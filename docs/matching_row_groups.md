---
title: Matching Row Groups
description: Guide to the RowGroupMatcher API and the Layex matching engine.
icon: lucide/target
---

# 🎯 Matching Row Groups

In many real-world spreadsheets, tables of interest are not nicely formatted or aligned. They might begin after arbitrary headers, contain multiline merged cells, or have variable numbers of columns and rows.

To reliably locate and extract these tables, **Tabularix** provides the **Layex Pattern Matching Engine**. **Layex** stands for **Layout Expression**, a concept and syntax derived from regular expressions but tailored for layout-level structures of spreadsheets. With Layex, you define a structural pattern of cell sequences (called a `RowPattern`) and row sequences (called a `RowGroupMatcher`) programmatically.

---

## 🏗️ Core Concepts

The layout matcher uses two primary builders:

1. **`RowPattern`**: Represents the expected horizontal sequence of cells in a single row. It is constructed using cell-matching rules (`value`, `regex`, `empty`, `non_empty`, `any`) and cell-level cardinalities (how many columns match this rule).
2. **`RowGroupMatcher`**: Represents the expected vertical sequence of rows. It compiles one or more `RowPattern`s together, along with row-level cardinalities (how many times a row or block of rows repeats).

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

Both `RowPattern` (cells) and `RowGroupMatcher` (rows) support the same cardinality methods to control matches:

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
from tabularix import RowGroupMatcher, value, regex, non_empty

matcher = (
    RowGroupMatcher()
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

We can match this 2-line header sequence by combining different row patterns inside `RowGroupMatcher`:

```python linenums="1"
from tabularix import RowGroupMatcher, value, regex, empty

multiline_header_matcher = (
    RowGroupMatcher()
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

You can check if list data matches a configured row group pattern using `matches_row_group`:

```python linenums="1"
from tabularix import RowGroupMatcher, value, non_empty

matcher = (
    RowGroupMatcher()
    .row(
        value("Category")
        .non_empty().one_or_more()
    )
)

# Returns True (starts with "Category" followed by one or more non-empty cells)
print(matcher.matches_row_group([["Category", "A", "B", "C"]]))

# Returns False (does not match Category)
print(matcher.matches_row_group([["Total", 123]]))
```
