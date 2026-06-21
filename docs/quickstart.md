---
title: Quickstart
description: Get up and running with Tabularix in minutes.
icon: lucide/rocket
---

# ⚡ Quickstart

Getting started with **Tabularix** is easy. Tabularix is a fast, layout-independent engine for parsing Excel spreadsheets and extracting structured tables.

Written in Rust for maximum speed, it integrates directly with Python to provide zero-copy sharing with frameworks like PyArrow, Polars, Pandas and DuckDB.

---

## 📦 Installation

Currently, Tabularix is in active development. You can compile and install it from source using `uv` and `maturin`:

```bash
# 1. Clone the repository
git clone https://github.com/pcasteran/tabularix.git
cd tabularix

# 2. Create a virtual environment and compile the Rust bindings
uv venv
uv run maturin develop
```

---

## 🚀 Tutorial: Extracting Tables from Messy Sheets

Excel spreadsheets are often structured for humans to read, not for computers to parse. They frequently contain header titles, metadata rows, empty spacing rows, and multiple tables nested in a single worksheet.

In this tutorial, we will take a messy spreadsheet and programmatically locate, extract, and clean a table, then load it into a Polars DataFrame for data science.

### Step 1: Visual Structure Analysis

The first step in extracting tables from a spreadsheet is understanding its layout. Tabularix allows you to render the worksheet's structure into a beautifully styled SVG layout file:

```python
import tabularix as tx

# Load the workbook
workbook = tx.load_workbook("tests/data/sample.xlsx")

# Get the target worksheet
sheet = workbook.get_sheet("complex")

# Save a structural layout SVG
sheet.to_svg("layout_structure.svg")
```

The resulting structural layout is shown below:

![Worksheet Structure Analysis](assets/sheet_complex.svg)

It highlights cell borders, merged regions, empty rows, and cell types (numeric, text, formulas) so you can visually plan your extraction patterns without opening Excel.

---

### Step 2: Define a Range Matcher

In this example, we need to extract the sales table located at the top of the sheet. Instead of hardcoding cell ranges like `A3:E8` (which break if a row is added or deleted at the top), Tabularix uses **Range Matchers** to find your table's boundaries dynamically.

Let's define a match pattern for a table that has a header row (with columns `Region` and four quarters) and variable-length data rows:

```python
# 1. Define the pattern of the header row.
# It starts with "Region", followed by 4 Quarter columns matching a regex pattern (e.g. Q1, Q2, etc.)
header_pattern = (
    tx.value("Region")
    .regex("^Q[1-4]$").repeat(4)
)

# 2. Define the pattern of the data rows.
# The row must:
#   - start with a region name, i.e. a string different than `Total` (which is the table footer)
#   - end by 4 non-empty data cells
data_pattern = (
    tx.regex(r"^(?!Total).*$")
    .non_empty().repeat(4)
)

# 3. Create the RangeMatcher.
# We expect one header row, followed by one-or-more data rows
matcher = (
    tx.RangeMatcher()
    .row(header_pattern)
    .row(data_pattern).one_or_more()
)
```

---

### Step 3: Scan and Locate the Table

Now, scan the worksheet to find the exact boundaries matching your matcher. Tabularix executes this scan in Rust for high performance:

```python
# Search the worksheet for the matching table pattern
found_range = sheet.search_range(matcher)

if found_range:
    print(f"Table located successfully!")
    print(f"Rows: {found_range.start_row} to {found_range.end_row}")
    print(f"Columns: {found_range.start_col} to {found_range.end_col}")
else:
    print("Table pattern not found in sheet.")
```

---

### Step 4: Extract the Structured Table

Using the coordinates returned by our search, we can define the separate header and data ranges and extract a structured `Table` object. We will also enable `clean_names` to clean our headers into standard Python identifiers:

```python
# The first row of the matched block is the header
header_range = tx.Range(
    start_row=found_range.start_row,
    end_row=found_range.start_row,
    start_col=found_range.start_col,
    end_col=found_range.end_col
)

# The subsequent rows are the data block
data_range = tx.Range(
    start_row=found_range.start_row + 1,
    end_row=found_range.end_row,
    start_col=found_range.start_col,
    end_col=found_range.end_col
)

# Extract and clean the table
table = sheet.extract_table(data_range, header=header_range, clean_names=True)

print("Columns:", table.columns)
# Output: ['region', 'q1', 'q2', 'q3', 'q4']
print("Table Shape:", table.shape)
```

---

### Step 5: Zero-Copy Integration (Arrow, Polars, Pandas, DuckDB)

Tabularix fully supports the standard **Arrow PyCapsule Interface**, allowing you to export your parsed table to modern data science frameworks with zero-copy overhead.

```python
# Convert to a PyArrow Table
arrow_table = table.to_arrow()

# 1. Zero-copy load into Polars
import polars as pl
df_polars = pl.from_arrow(arrow_table)
print(df_polars)

# 2. Zero-copy load into Pandas
df_pandas = arrow_table.to_pandas()
print(df_pandas.head())
```

---

## ➡️ Next Steps

- Learn more about [Range Matching](matching_ranges.md).
- Learn more about [Table Extraction](table_extraction.md).
- Visit the [API Reference](api.md) to explore all parameter options for `extract_table` (such as flattening hierarchical multi-row headers).
