# Tabularix Core API & Engine Design

## Problem Statement
How might we build a robust, high-performance Excel extraction framework that provides layout-flexible APIs (Active Mutator & Pattern Matching) to handle highly variable spreadsheets without requiring a proprietary configuration DSL?

## Recommended Direction
A Rust-backed Python library ("Configuration-as-Code") offering two complementary extraction paradigms that can be seamlessly combined:

1. **Active Mutator API**: Surgically clean, unmerge, and crop the spreadsheet grid relative to anchors.
2. **RowGroupMatcher API (Layex)**: Declaratively search for structural row groups using either a String DSL or a Python Object Builder.

The final output of any extraction pipeline is an **Apache Arrow Table**, allowing zero-copy integration into standard data engineering tools (Polars, Pandas, DuckDB).

## Key Assumptions to Validate
- [ ] **Performance Overhead**: Passing cell/grid structures across the Rust/Python boundary via PyO3 is performant enough for 100MB+ spreadsheets.
- [ ] **RowGroupMatcher Ergonomics**: The Layex engine can elegantly handle complex cardinality (e.g., nested groups of cells `([type:string] [type:numeric]+)*`).
- [ ] **Arrow Memory Model**: We can cleanly stream or convert the Rust internal grid representation directly into Apache Arrow memory layouts.

## MVP Scope
- **Rust Core & PyO3 Bindings**: High-performance core engine reading raw Excel files.
- **Active Mutator APIs**: `search_and_crop_before()`, `unmerge_and_fill()`, `crop()`, `drop_rows()`.
- **RowGroupMatcher Engine**:
  - Match by Exact Value, Type (String, Numeric, Empty), and **Entity** (Semantic regex bundles like `[entity:date]`).
  - Regex-style cardinality (`*`, `+`, `?`, `{n,m}`) and grouping `()`.
  - Dual Interface: String DSL (e.g. `'[entity:date] [type:numeric]+'`) and Python Builder API.
- **Table Assembly**: Construct an Apache Arrow table from discrete extracted blocks: `build_table_from_row_groups(header, data, footer)`.

## Not Doing (and Why)
- **Matching by Cell Formatting (v1)**: Deferred. Parsing styles (bold, colors) adds significant complexity. We will focus entirely on values, types, and entities for the MVP.
- **Proprietary JSON/YAML Configurations**: Deferred/Discarded. We embrace Python scripts as the native configuration language to eliminate friction and leverage existing toolchains.
- **Anonymizer & Agent Generator (v1)**: Deferred. We must build a rock-solid data manipulation engine before adding AI workflows to auto-generate the scripts.

## Example Blueprint
```python
import tabularix as tx

# 1. Load and Active Mutator Cleaning
sheet = tx.load_workbook("report.xlsx").active_sheet()
sheet.unmerge_cells(strategy="fill_down")
sheet.search_and_crop_before(marker="Invoice Date", direction="TOP")

# 2. RowGroupMatcher Pattern Definition (Using Dual API)
header_matcher = tx.RowGroupMatcher.from_layex('"Date" "Description" "Amount"')
data_matcher = tx.RowGroupMatcher().entity("date").type("string").type("numeric").one_or_more()
footer_matcher = tx.RowGroupMatcher.from_layex('("Total" | "Subtotal") [type:numeric]')

# 3. Search and Assemble
header = sheet.search_row_group(header_matcher)
footer = sheet.search_row_group(footer_matcher)
data = sheet.extract_rows_between(start=header, end=footer)

# 4. Filter and Export to Apache Arrow
data = data.filter(lambda row: not row.matches(tx.RowGroupMatcher.from_layex('"Subtotal" *')))
arrow_table = tx.build_table_from_row_groups(header=header, data=data)
```
