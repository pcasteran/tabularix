## Problem statement

In most organizations, mission-critical data exists in a state of high entropy, trapped in fragmented Excel files. This "hidden data" is the primary barrier to meaningful AI adoption for all companies.

There is a need for a solution designed to identify, extract, and organize this hidden data, transforming it into a clean, structured format ready for AI and advanced analytics.

## Our mission

Our mission is to build a framework that serves as a stepping stone to solving the challenges described in the previous section.

This framework must provide Smart Data Extraction, utilizing advanced logic and workflows to help identify meaningful information while filtering out everyday data clutter.

The framework will operate in two distinct stages:

1.  **Smart Configuration Phase**: A collaborative, human-in-the-loop process where an **Extraction Configuration** is defined using sample documents representative of a **Document Kind**.
2.  **Automated Run Phase**: A high-performance execution stage where the **Extraction Configuration** is applied to extract data from new documents of the same **Document Kind**.


## Proposed Solution

The proposed solution consists of a Rust library providing a robust Excel document manipulation framework specifically designed for data extraction. This library will be exposed to Python via bindings (e.g., PyO3), combining Rust's safety and performance with Python's scripting ergonomics.

This library will expose high-level **APIs** encapsulating common lower-level operations (such as value searches, adding/removing rows and columns, modifying cell values, etc.).

These **APIs** allow the implementation of **multiple** distinct data extraction **strategies**. They can be combined within a script that defines the complete extraction recipe for a target **Document Kind**.

For example:

```python

# First, clean the document.
sheet.search_marker_and_clean_before(marker_text_or_regex="Report XYZ", direction="TOP")  # Supported: TOP, BOTTOM, LEFT, RIGHT, TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT and BOTTOM_RIGHT.
sheet.drop_columns_when_fill_ratio_less_than(0.2)
sheet.crop_all()

# Extract the sub-components of the table we are interested in.
# Match one row group at a time using a dedicated SINGLE layex and then assemble these row groups into a table.
header = sheet.search_matching_row_group(layex="...")
data = sheet.search_matching_row_group(layex="...")

table = sheet.build_table_from_row_groups(header=header, data=data)

# Or, another possible strategy:
header = sheet.search_matching_row_group(layex="...")
footer = sheet.search_matching_row_group(layex="...")
data = sheet.extract_row_group_between(begin=header, end=footer)

table = sheet.build_table_from_row_groups(header=header, data=data, footer=footer)

# Finally, output the table to Parquet files.
table.write("table_1.parquet")

# Or use it with standard data tools.
df = table.to_df()
arrow_table = table.to_arrow()
duckdb.sql("SELECT * FROM arrow_table")
```

Using Python to script these extraction rules offers several key advantages:

- **Configuration as Code**: The extraction "recipe" is written in a general-purpose programming language widely adopted across the data ecosystem, fully supported by existing toolchains (IDEs, linters, CI/CD, version control, etc.).
- **Portability**: The scripts can run in any environment and easily integrate into standard data pipelines.
- **Local Development**: The extraction logic can be run, tested, and debugged easily on a local machine.


