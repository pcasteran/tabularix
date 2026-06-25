from typing import cast

import polars as pl
import tabularix as tx
from tabularix import RangeMatcher, empty, regex, value


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


def extract_territory_tables(sheet: tx.Sheet) -> pl.DataFrame:
    """Extracts and combines the four territory tables from the worksheet dynamically."""
    # Define RangeMatcher to locate any of the territory titles.
    territory_matcher = RangeMatcher().row(regex(r"^(North|South|East|West)$"))

    # Define RangeMatchers for headers and footers using greedy matching rules
    # to dynamically capture the full width of the table.
    header_matcher = (
        RangeMatcher()
        # First header row.
        .row(
            # The row starts with static text "Product".
            value("Product")
            # Then we have zero or more occurrences of a group of two cells.
            .group(
                # The group starts with a year (4 digits).
                regex(r"\d{4}")
                # And finishes with an empty cell (merged with the one on the left).
                .empty()
            )
            .zero_or_more()
        )
        # Second header row.
        .row(
            # The row starts with an empty cell (merged with the one above).
            empty()
            # Then we have zero or more occurrences of a group of two cells.
            .group(
                # The group starts with static text "Expected".
                value("Expected")
                # And finishes with static text "Actual".
                .value("Actual")
            )
            .zero_or_more()
        )
    )

    footer_matcher = RangeMatcher().row(value("Total").any().zero_or_more())

    dfs = []
    search_row = 0

    while search_row < sheet.shape[0]:
        # Match territory title range dynamically below the last footer
        territory_range = sheet.search_range(territory_matcher, start_row=search_row)
        if territory_range is None:
            break

        # Retrieve the matched territory name (e.g. "North", "South")
        territory = str(sheet.get_cell_value(territory_range.start_row, territory_range.start_col))

        # Match the 2-row header range below the territory title.
        # We start searching from the row below the territory title to avoid
        # column bounds conflicts (since territory_range has a width of 1 column,
        # but the header_range has a width of 5 columns).
        header_range = sheet.search_range(header_matcher, start_row=territory_range.end_row + 1)
        if header_range is None:
            raise ValueError(f"Header not found for territory '{territory}'.")

        # Match the footer range below the header
        footer_range = sheet.search_range_relative(footer_matcher, below=header_range)
        if footer_range is None:
            raise ValueError(f"Footer not found for territory '{territory}'.")

        # Get the data range situated between header and footer
        data_range = sheet.get_range_between(header_range, footer_range)

        # Extract the structured table, flattening the multi-row headers
        table = sheet.extract_table(
            data_range,
            header=header_range,
            clean_names=True,
            flatten_header=True,
            header_separator="_",
        )

        # Convert to Polars and insert the territory context column at the beginning
        df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))
        df = df.select([pl.lit(territory).alias("territory"), pl.all()])
        dfs.append(df)

        # Advance search_row to scan below the current section in the next iteration
        search_row = footer_range.end_row + 1

    if not dfs:
        raise ValueError("No territory tables were found in the worksheet.")

    return pl.concat(dfs)


def main() -> None:
    """Run the multiple tables extraction example."""
    # Load the workbook.
    workbook = tx.load_workbook("tests/data/sample.xlsx")

    # Get the target worksheet.
    sheet = workbook.get_sheet("multi-tables")

    # Extract the metadata.
    metadata_df = extract_metadata(sheet)
    print("Extracted Metadata DataFrame:")
    print(metadata_df)
    print("-" * 40)

    # Extract and combine territory tables.
    territories_df = extract_territory_tables(sheet)
    print("Combined Territories DataFrame:")
    print(territories_df)
    print("-" * 40)

    # Project the metadata into the combined territories DataFrame.
    # Since metadata_df is a single-row DataFrame, a cross join broadcasts
    # its columns to all rows of the combined DataFrame.
    projected_df = territories_df.join(metadata_df, how="cross")
    print("Projected DataFrame:")
    print(projected_df)


if __name__ == "__main__":
    main()
