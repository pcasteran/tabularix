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

# Load the workbook.
workbook = tx.load_workbook("tests/data/sample.xlsx")

# Get the target worksheet.
sheet = workbook.get_sheet("complex")

# Export the sheet to SVG.
sheet.to_svg("sheet.svg")
```

The resulting structural layout is shown below:

![Worksheet Structure Analysis](assets/sheet_complex.svg)

It highlights cell borders, merged regions, empty rows, and cell types (numeric, text, formulas) so you can visually plan your extraction patterns without opening Excel.

---

### Step 2: Define a Range Matcher

In this example, we need to extract the sales table located at the top of the sheet. Instead of hardcoding cell ranges like `A3:E8` (which break if a row is added or deleted at the top), Tabularix uses **Range Matchers** to find your table's boundaries dynamically.

Let's define a match pattern for the header row and a match pattern for the data rows:

```python
from tabularix import RangeMatcher, regex, value

# Define the pattern and matcher for the header row.
# It starts with "Region", followed by 4 Quarter columns matching
#  a regex pattern (e.g. Q1, Q2, etc.).
header_pattern = (
    # Static string
    value("Region")
    # Quarter header: Q1, Q2, Q3, Q4
    .regex(r"^Q[1-4]$")
    .repeat(4, max=4)
)

header_matcher = RangeMatcher().row(header_pattern)

# Define the pattern and matcher for the data rows. The rows must:
#   - start with a region name, i.e. a string different than `Total` (which
#     is the marker of the table footer)
#   - end by 4 non-empty data cells
data_pattern = (
    # Match any string except "Total"
    regex(r"^(?!Total).*$")
    # Quarters amount
    .non_empty()
    .repeat(4, max=4)
)

data_matcher = RangeMatcher().row(data_pattern).one_or_more()
```

---

### Step 3: Scan and Locate the Table Ranges

Now, scan the worksheet to locate the header row, and then scan for the data rows relative to it (using the `below` constraint). Tabularix executes this scan in Rust for high performance:

```python
# Search for the header row anywhere in the sheet (no location constraint).
header_range = sheet.search_range(header_matcher)
if header_range is None:
    raise ValueError("Header not found")

print(f"Table header found: {header_range}")

# Search for the data rows located below the header.
data_range = sheet.search_range_relative(data_matcher, below=header_range)
if data_range is None:
    raise ValueError("Data not found")

print(f"Table data found: {data_range}")
```

---

### Step 4: Extract the Structured Table

Using the coordinates returned by our search, we extract the structured `Table` object. We will also enable `clean_names` to clean our headers into standard Python identifiers:

```python
# Extract the table from the sheet
table = sheet.extract_table(data_range, header_range, clean_names=True)

print("Columns:", table.columns)
# Output: ['region', 'q1', 'q2', 'q3', 'q4']
print("Table Shape:", table.shape)
```

---

### Step 5: Zero-Copy Integration (Arrow, Polars, Pandas, DuckDB)

Tabularix fully supports the standard **Arrow PyCapsule Interface**, allowing you to export your parsed table to modern data science frameworks with zero-copy overhead.

```python
# Zero-copy load into a Pandas dataframe.
df_pandas = table.to_arrow().to_pandas()
print("Pandas dataframe created:")
print(df_pandas.head())

# Zero-copy load into a Polars dataframe.
import polars as pl
df_polars = pl.from_arrow(table.to_arrow())
print("Polars dataframe created:")
print(df_polars)

# Zero-copy load and query in DuckDB (directly consumes the Table instance).
import duckdb
rel_duckdb = duckdb.from_arrow(table)
res_duckdb = rel_duckdb.query("sales_table", "SELECT * FROM sales_table WHERE Q1 > 12000")
print("DuckDB query result:")
print(res_duckdb)
```

---

## ➡️ Next Steps

- Learn more about [Range Matching](matching_ranges.md).
- Learn more about [Table Extraction](table_extraction.md).
- Visit the [API Reference](api.md) to explore all parameter options for `extract_table` (such as flattening hierarchical multi-row headers).
