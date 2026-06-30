import duckdb
import pandas as pd
import polars as pl
import tabularix as tx
from tabularix import RangePattern1D, RangePattern2D, Sheet, Table, any, non_empty, regex, value


def extract_table_below_header(sheet: Sheet) -> Table:
    """Extracts a table below the header row."""
    # Define the pattern and matcher for the header row.
    # It starts with "Region", followed by 4 Quarter columns matching a regex pattern (e.g. Q1, Q2, etc.).
    header_pattern = RangePattern1D(
        value("Region"),  # Static string
        regex(r"^Q[1-4]$").repeat(4, max=4),  # Quarter header: Q1, Q2, Q3, Q4
    )

    header_matcher = header_pattern.to_matcher(direction="LR")

    # Define the pattern and matcher for the data rows. The rows must:
    #   - start with a region name, i.e. a string different than `Total` (which is the marker of the table footer)
    #   - end by 4 non-empty data cells
    data_pattern = RangePattern2D(
        RangePattern1D(
            regex(r"^(?!Total).*$"),  # Match any string except "Total"
            non_empty().repeat(4, max=4),  # Quarters amount
        ).one_or_more()
    )

    data_matcher = data_pattern.to_matcher(outer_direction="TB", inner_direction="LR")

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

    # Extract the table from the sheet.
    table = sheet.extract_table(data_range, header_range, clean_names=True)

    return table


def extract_table_between_header_and_footer(sheet: Sheet) -> Table:
    """Extracts a table between header and footer relative boundaries."""
    # Define the pattern and matcher for the header row.
    # It starts with "Region", followed by 4 Quarter columns matching a regex pattern (e.g. Q1, Q2, etc.).
    header_pattern = RangePattern1D(
        value("Region"),  # Static string
        regex(r"^Q[1-4]$").repeat(4, max=4),  # Quarter header: Q1, Q2, Q3, Q4
    )

    header_matcher = header_pattern.to_matcher(direction="LR")

    # Define the pattern and matcher for the footer row.
    # It starts with "Total [YEAR]", followed by 4 non-empty cells.
    footer_pattern = RangePattern1D(
        regex(r"^Total \d{4}$"),  # Yearly total
        any().repeat(4, max=4),  # Quarters total amount
    )

    footer_matcher = footer_pattern.to_matcher(direction="LR")

    # Search for the header row anywhere in the sheet (no location constraint).
    header_range = sheet.search_range(header_matcher)
    if header_range is None:
        raise ValueError("Header not found")

    print(f"Table header found: {header_range}")

    # Search for the footer rows located below the header.
    footer_range = sheet.search_range_relative(footer_matcher, below=header_range)
    if footer_range is None:
        raise ValueError("Footer not found")

    print(f"Table footer found: {footer_range}")

    # Get the data range between the header and footer.
    data_range = sheet.get_range_between(header_range, footer_range)
    print(f"Table data found: {data_range}")

    # Extract the table from the sheet.
    table = sheet.extract_table(data_range, header_range, clean_names=True)

    return table


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
    workbook = tx.load_workbook("tests/data/sample.xlsx")

    # Get the target worksheet.
    sheet = workbook.get_sheet("complex")

    # Export the sheet to SVG.
    sheet.to_svg("sheet.svg")

    # Extract table using strategy #1.
    print("-----")
    table_1 = extract_table_below_header(sheet)
    print(f"Table extracted with strategy #1; shape: {table_1.shape}, columns: {table_1.columns}")

    # Extract table using strategy #2.
    print("-----")
    table_2 = extract_table_between_header_and_footer(sheet)
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
