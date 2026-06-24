from typing import cast

import polars as pl
import tabularix as tx
from tabularix import RangeMatcher, regex


def extract_metadata(sheet: tx.Sheet) -> pl.DataFrame:
    """Extracts key-value metadata from the worksheet dynamically, returning a transposed Polars DataFrame.

    We need to transpose the extracted table because the metadata is stored horizontally in the spreadsheet:
    the first column (column_1) contains the headers/keys ('Date', 'Fiscal Year'), and the second column
    (column_2) contains the corresponding values. Transposing maps the keys to DataFrame column names.
    """
    # Define a RangeMatcher to locate the metadata block dynamically.
    metadata_matcher = (
        RangeMatcher()
        # Row pattern: "Date" or "Fiscal Year" followed by a non-empty value.
        .row(regex(r"^(Date|Fiscal Year)$").non_empty())
        .one_or_more()
    )

    metadata_range = sheet.search_range(metadata_matcher)
    if metadata_range is None:
        raise ValueError("Metadata block not found in the worksheet.")

    # Extract the metadata as a Table (without a header, so columns will be default: column_1, column_2).
    table = sheet.extract_table(metadata_range)

    # Convert the extracted Table to a Polars DataFrame.
    df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))

    # Transpose the DataFrame so keys in column_1 become column names.
    df_transposed = df.transpose(column_names="column_1")

    # If "Date" column exists, parse the ISO 8601 string to a native date type.
    if "Date" in df_transposed.columns:
        df_transposed = df_transposed.with_columns(pl.col("Date").str.to_date())

    return df_transposed


def main() -> None:
    """Run the multiple tables extraction example."""
    # Load the workbook
    workbook = tx.load_workbook("tests/data/sample.xlsx")

    # Get the target worksheet
    sheet = workbook.get_sheet("multi-tables")

    # Extract the metadata.
    metadata_df = extract_metadata(sheet)
    print("Extracted Metadata DataFrame:")
    print(metadata_df)


if __name__ == "__main__":
    main()
