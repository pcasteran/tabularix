import duckdb
import pandas as pd
import polars as pl
from tabularix import (
    Table,
    any,
    extract_table_between_header_and_footer,
    extract_table_with_header_and_data,
    grid,
    group,
    load_workbook,
    non_empty,
    regex,
    value,
)

# Define patterns as constants
HEADER_PATTERN = group(
    value("Region"),  # Static string
    regex(r"^Q[1-4]$").repeat(4, max=4),  # Quarter header: Q1, Q2, Q3, Q4
)

DATA_PATTERN = grid(
    group(
        regex(r"^(?!Total).*$"),  # Match any string except "Total"
        non_empty().repeat(4, max=4),  # Quarters amount
    ).one_or_more()
)

FOOTER_PATTERN = group(
    regex(r"^Total \d{4}$"),  # Yearly total
    any().repeat(4, max=4),  # Quarters total amount
)


def use_with_pandas(table: Table) -> pd.DataFrame:
    """Converts the extracted Table to a Pandas DataFrame."""
    # Convert to a PyArrow Table.
    arrow_table = table.to_arrow()

    # Zero-copy load into a Pandas dataframe.
    df = arrow_table.to_pandas()
    print("Pandas dataframe created")
    print(df.head())

    return df


def use_with_polars(table: Table) -> pl.DataFrame:
    """Converts the extracted Table to a Polars DataFrame."""
    # Convert to a PyArrow Table.
    arrow_table = table.to_arrow()

    # Zero-copy load into a Polars dataframe.
    df = pl.from_arrow(arrow_table)
    assert isinstance(df, pl.DataFrame)  # nosec B101
    print("Polars dataframe created")
    print(df.head())

    return df


def use_with_duckdb(table: Table) -> duckdb.DuckDBPyRelation:
    """Converts the extracted Table to a DuckDB relation and queries it."""
    # Zero-copy load into a DuckDB relation.
    rel = duckdb.from_arrow(table)
    print("DuckDB relation created")

    # Execute a query and print the result.
    res = rel.query("sales_table", "SELECT * FROM sales_table WHERE Q1 > 12000")
    print(res)

    return rel


def main() -> None:
    """Demonstrates loading workbooks, creating visual renders, and extracting tables using different strategies."""
    # Load the workbook.
    workbook = load_workbook("tests/data/sample.xlsx")

    # Get the target worksheet.
    sheet = workbook.get_sheet("complex")

    # Export the sheet to SVG.
    sheet.to_svg("sheet.svg")

    # Extract table using strategy #1: Header & Data patterns.
    print("-----")
    table_1 = extract_table_with_header_and_data(sheet, HEADER_PATTERN, DATA_PATTERN, clean_names=True)
    print(f"Table extracted with strategy #1; shape: {table_1.shape}, columns: {table_1.columns}")

    # Extract table using strategy #2: Between Header & Footer patterns.
    print("-----")
    table_2 = extract_table_between_header_and_footer(sheet, HEADER_PATTERN, FOOTER_PATTERN, clean_names=True)
    print(f"Table extracted with strategy #2; shape: {table_2.shape}, columns: {table_2.columns}")

    # Use the table.
    print("-----")
    use_with_pandas(table_2)
    print("-----")
    use_with_polars(table_2)
    print("-----")
    use_with_duckdb(table_2)


if __name__ == "__main__":
    main()
