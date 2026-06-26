---
title: Extracting Multiple Tables
description: Learn how to locate, extract, combine, and project metadata onto multiple independent tables from a single Excel worksheet.
icon: lucide/layers
---

# 🗂️ Extract, Combine, and Project Multiple Tables

In real-world spreadsheets, you will often find multiple independent tables stacked vertically on the same worksheet, alongside key-value metadata scattered outside the main tables (e.g., report dates, fiscal years, or run parameters).

This tutorial guides you through using **Tabularix** to programmatically:

1. Locate and extract horizontal metadata, transposing it into a structured single-row DataFrame.
2. Locate, extract, and combine multiple territory-specific tables from the `"multi-tables"` worksheet of `sample.xlsx` into a single unified DataFrame.
3. Project the extracted metadata onto the combined territory rows using a Polars cross join.

---

## 🔍 The Scenario

Suppose you want to extract data from the following worksheet:

[<img src="../assets/sheet_multi_tables.svg" alt="Multi-tables Worksheet Layout" width="600" />](../assets/sheet_multi_tables.svg)

The worksheet contains some data we are not interested in at the very top.
Then the **Financial Report** section begins with a metadata block at the top (**Date** and **Fiscal Year**) and four territory tables (**North**, **South**, **East**, and **West**) below it. Each territory table has:

- A title row containing the name of the territory (e.g., `"North"`).
- A two-row header starting with `"Product"` in the top-left cell, followed by fiscal years and performance types.
- A variable number of data rows for products.
- A summary row starting with `"Total"`.

---

## 🛠️ Step 1: Extract and Transpose Metadata

Metadata is often stored horizontally: keys in one column and values in the adjacent column. To make this metadata usable as row-level dimensions, we extract it and transpose it so the keys become column headers.

### Define the Metadata Matcher

We define a [RangeMatcher](../api.md#tabularix.RangeMatcher) matching "Date" or "Fiscal Year" followed by a non-empty cell:

```python
from tabularix import RangeMatcher, regex

metadata_matcher = (
    RangeMatcher()
    .row(regex(r"^(Date|Fiscal Year)$").non_empty())
    .one_or_more()
)
```

### Transpose and Type Cast with Polars

We extract the range, convert it to a Polars DataFrame, transpose it, and parse the date string:

```python
from typing import cast
import polars as pl
import tabularix as tx

def extract_metadata(sheet: tx.Sheet) -> pl.DataFrame:
    metadata_range = sheet.search_range(metadata_matcher)
    if metadata_range is None:
        raise ValueError("Metadata block not found.")

    table = sheet.extract_table(metadata_range)

    # Cast to pl.DataFrame to satisfy static type checkers
    df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))

    # Transpose so keys in column_1 become headers
    df_transposed = df.transpose(column_names="column_1")

    if "Date" in df_transposed.columns:
        df_transposed = df_transposed.with_columns(pl.col("Date").str.to_date())

    return df_transposed
```

---

## 🛠️ Step 2: Define Territory Layout Patterns

To match the territory tables dynamically, we define layout matchers for the headers and footers. Because of merged cells and empty cells representing empty values or un-cached formulas, we use **greedy cell matchers** to capture the full width of the table.

```python
from tabularix import RangeMatcher, empty, regex, value

# Locate the territory title
territory_matcher = RangeMatcher().row(regex(r"^(North|South|East|West)$"))

# Locate the two-row merged header
header_matcher = (
    RangeMatcher()
    .row(
        value("Product")
        .group(
            regex(r"\d{4}")
            .empty()  # Matches the merged cell next to the year
        )
        .zero_or_more()
    )
    .row(
        empty()       # Matches the merged cell below "Product"
        .group(
            value("Expected")
            .value("Actual")
        )
        .zero_or_more()
    )
)

# Locate the footer row using greedy wildcard matching
footer_matcher = RangeMatcher().row(value("Total").any().zero_or_more())
```

---

## 🔄 Step 3: Dynamic Scanning Loop

To extract all territory tables, we scan the worksheet vertically using a `while` loop, advancing our search starting point (`search_row`) below the footer of each extracted table.

We insert the `territory` context column as the **first column** of each DataFrame by using Polars' `select` syntax with `pl.all()`:

```python
def extract_territory_tables(sheet: tx.Sheet) -> pl.DataFrame:
    dfs = []
    search_row = 0

    while search_row < sheet.shape[0]:
        # 1. Match the territory name (e.g. "North")
        territory_range = sheet.search_range(territory_matcher, start_row=search_row)
        if territory_range is None:
            break

        territory = str(sheet.get_cell_value(territory_range.start_row, territory_range.start_col))

        # 2. Match the 2-row header range below the title
        header_range = sheet.search_range(header_matcher, start_row=territory_range.end_row + 1)
        if header_range is None:
            raise ValueError(f"Header not found for {territory}")

        # 3. Match the footer relative to the header
        footer_range = sheet.search_range_relative(footer_matcher, below=header_range)
        if footer_range is None:
            raise ValueError(f"Footer not found for {territory}")

        # 4. Extract data rows between header and footer
        data_range = sheet.get_range_between(header_range, footer_range)
        table = sheet.extract_table(
            data_range,
            header=header_range,
            clean_names=True,
            flatten_header=True,
            header_separator="_",
        )

        # 5. Convert to Polars and insert territory as the first column
        df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))
        df = df.select([pl.lit(territory).alias("territory"), pl.all()])
        dfs.append(df)

        # Move the cursor below this table's footer
        search_row = footer_range.end_row + 1

    return pl.concat(dfs)
```

---

## 🧬 Step 4: Projecting Metadata (Cross Join)

Once we have a single-row metadata DataFrame and a combined 12-row territory DataFrame, we can merge them. A Polars **cross join** acts as a broadcast join, appending the metadata fields (`Date` and `Fiscal Year`) to every row of the combined data dynamically.

```python
# Broadcast metadata to all territory rows
projected_df = territories_df.join(metadata_df, how="cross")
```

---

## 📄 Complete Implementation Code

The complete, runnable implementation of this tutorial is located at [examples/extract_multiple_tables.py](https://github.com/pcasteran/tabularix/blob/main/examples/extract_multiple_tables.py).

To execute it:

```bash
uv run examples/extract_multiple_tables.py
```

### Script Output

The script outputs three separate stages of the extraction process, culminating in the projected, unified DataFrame:

```text
Extracted Metadata DataFrame:
shape: (1, 2)
┌────────────┬─────────────┐
│ Date       ┆ Fiscal Year │
│ ---        ┆ ---         │
│ date       ┆ str         │
╞════════════╪═════════════╡
│ 2026-06-23 ┆ 2025-2026   │
└────────────┴─────────────┘
----------------------------------------
Combined Territories DataFrame:
shape: (12, 6)
┌───────────┬─────────────┬─────────────┬────────────┬────────────┬────────────┐
│ territory ┆ product_pro ┆ 2025_expect ┆ 2025_actua ┆ 2026_expec ┆ 2026_actua │
│ ---       ┆ duct        ┆ ed          ┆ l          ┆ ted        ┆ l          │
│ str       ┆ ---         ┆ ---         ┆ ---        ┆ ---        ┆ ---        │
│           ┆ str         ┆ f64         ┆ f64        ┆ f64        ┆ f64        │
╞═══════════╪═════════════╪═════════════╪════════════╪════════════╪════════════╡
│ North     ┆ Product A   ┆ 427.0       ┆ 147.0      ┆ 122.0      ┆ 479.0      │
│ North     ┆ Product B   ┆ 240.0       ┆ 215.0      ┆ 224.0      ┆ 171.0      │
│ North     ┆ Product C   ┆ 477.0       ┆ 142.0      ┆ 456.0      ┆ 479.0      │
│ South     ┆ Product A   ┆ 379.0       ┆ 134.0      ┆ 412.0      ┆ 316.0      │
│ South     ┆ Product B   ┆ 116.0       ┆ 105.0      ┆ 157.0      ┆ 211.0      │
│ …         ┆ …           ┆ …           ┆ …          ┆ …          ┆ …          │
│ East      ┆ Product A   ┆ 459.0       ┆ 369.0      ┆ 324.0      ┆ 212.0      │
│ East      ┆ Product B   ┆ 329.0       ┆ 391.0      ┆ 252.0      ┆ 514.0      │
│ West      ┆ Product A   ┆ 103.0       ┆ 478.0      ┆ 522.0      ┆ 181.0      │
│ West      ┆ Product B   ┆ 457.0       ┆ 306.0      ┆ 284.0      ┆ 242.0      │
│ West      ┆ Product C   ┆ 179.0       ┆ 200.0      ┆ 500.0      ┆ 272.0      │
└───────────┴─────────────┴─────────────┴────────────┴────────────┴────────────┘
----------------------------------------
Projected DataFrame:
shape: (12, 8)
┌─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬────────┐
│ territo ┆ product ┆ 2025_ex ┆ 2025_ac ┆ 2026_ex ┆ 2026_ac ┆ Date    ┆ Fiscal │
│ ry      ┆ _produc ┆ pected  ┆ tual    ┆ pected  ┆ tual    ┆ ---     ┆ Year   │
│ ---     ┆ t       ┆ ---     ┆ ---     ┆ ---     ┆ ---     ┆ date    ┆ ---    │
│ str     ┆ ---     ┆ f64     ┆ f64     ┆ f64     ┆ f64     ┆         ┆ str    │
│         ┆ str     ┆         ┆         ┆         ┆         ┆         ┆        │
╞═════════╪═════════╪═════════╪═════════╪═════════╪═════════╪═════════╪════════╡
│ North   ┆ Product ┆ 427.0   ┆ 147.0   ┆ 122.0   ┆ 479.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ A       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ North   ┆ Product ┆ 240.0   ┆ 215.0   ┆ 224.0   ┆ 171.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ B       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ North   ┆ Product ┆ 477.0   ┆ 142.0   ┆ 456.0   ┆ 479.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ C       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ South   ┆ Product ┆ 379.0   ┆ 134.0   ┆ 412.0   ┆ 316.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ A       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ South   ┆ Product ┆ 116.0   ┆ 105.0   ┆ 157.0   ┆ 211.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ B       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ …       ┆ …       ┆ …       ┆ …       ┆ …       ┆ …       ┆ …       ┆ …      │
│ East    ┆ Product ┆ 459.0   ┆ 369.0   ┆ 324.0   ┆ 212.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ A       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ East    ┆ Product ┆ 329.0   ┆ 391.0   ┆ 252.0   ┆ 514.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ B       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ West    ┆ Product ┆ 103.0   ┆ 478.0   ┆ 522.0   ┆ 181.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ A       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ West    ┆ Product ┆ 457.0   ┆ 306.0   ┆ 284.0   ┆ 242.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ B       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
│ West    ┆ Product ┆ 179.0   ┆ 200.0   ┆ 500.0   ┆ 272.0   ┆ 2026-06 ┆ 2025-2 │
│         ┆ C       ┆         ┆         ┆         ┆         ┆ -23     ┆ 026    │
└─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴────────┘
```
