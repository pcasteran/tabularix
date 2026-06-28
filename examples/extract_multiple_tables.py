from typing import cast

import polars as pl
import tabularix as tx
from tabularix import RangePattern1D, RangePattern2D, any, empty, non_empty, regex, value


def extract_metadata(sheet: tx.Sheet) -> pl.DataFrame:
    """Extracts key-value metadata from the worksheet dynamically, returning a transposed Polars DataFrame.

    We need to transpose the extracted table because the metadata is stored horizontally in the spreadsheet:
    the first column (column_1) contains the headers/keys ('Date', 'Fiscal Year'), and the second column
    (column_2) contains the corresponding values. Transposing maps the keys to DataFrame column names.
    """
    # Define a RangePattern2D to locate the metadata block dynamically.
    metadata_pattern = RangePattern2D(
        [
            # Row pattern: "Date" or "Fiscal Year" followed by a non-empty value.
            RangePattern1D([regex(r"^(Date|Fiscal Year)$"), non_empty()]).one_or_more()
        ]
    )

    metadata_matcher = metadata_pattern.to_matcher(outer_direction="TB", inner_direction="LR")
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
    # Define RangePattern1D to locate any of the territory titles.
    territory_pattern = RangePattern1D([regex(r"^(North|South|East|West)$")])
    territory_matcher = territory_pattern.to_matcher(direction="LR")

    # Define RangePattern2D for headers and footers using greedy matching rules
    # to dynamically capture the full width of the table.
    header_pattern = RangePattern2D(
        [
            # First header row.
            RangePattern1D(
                [
                    # The row starts with static text "Product".
                    value("Product"),
                    # Then we have zero or more occurrences of a group of two cells.
                    RangePattern1D(
                        [
                            regex(r"\d{4}"),  # The group starts with a year (4 digits).
                            empty(),  # And finishes with an empty cell (merged with the one on the left).
                        ]
                    ).zero_or_more(),
                ]
            ),
            # Second header row.
            RangePattern1D(
                [
                    # The row starts with an empty cell (merged with the one above).
                    empty(),
                    # Then we have zero or more occurrences of a group of two cells.
                    RangePattern1D(
                        [
                            value("Expected"),  # The group starts with static text "Expected".
                            value("Actual"),  # And finishes with static text "Actual".
                        ]
                    ).zero_or_more(),
                ]
            ),
        ]
    )
    header_matcher = header_pattern.to_matcher(outer_direction="TB", inner_direction="LR")

    footer_pattern = RangePattern1D([value("Total"), any().zero_or_more()])
    footer_matcher = footer_pattern.to_matcher(direction="LR")

    dfs = []
    search_row = 0

    while search_row < sheet.shape[0]:
        # Match territory title range dynamically below the last footer.
        territory_range = sheet.search_range(territory_matcher, start_row=search_row)
        if territory_range is None:
            break

        # Retrieve the matched territory name (e.g. "North", "South").
        territory = str(sheet.get_cell_value(territory_range.start_row, territory_range.start_col))

        # Match the 2-row header range below the territory title.
        header_range = sheet.search_range(header_matcher, start_row=territory_range.end_row + 1)
        if header_range is None:
            raise ValueError(f"Header not found for territory '{territory}'.")

        # Match the footer range below the header.
        footer_range = sheet.search_range_relative(footer_matcher, below=header_range)
        if footer_range is None:
            raise ValueError(f"Footer not found for territory '{territory}'.")

        # Get the data range situated between header and footer.
        data_range = sheet.get_range_between(header_range, footer_range)

        # Extract the structured table, flattening the multi-row headers.
        table = sheet.extract_table(
            data_range,
            header=header_range,
            clean_names=True,
            flatten_header=True,
            header_separator="_",
        )

        # Convert to a Polars DataFrame and insert the territory context column at the beginning.
        df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))
        df = df.select([pl.lit(territory).alias("territory"), pl.all()])
        dfs.append(df)

        # Advance search_row to scan below the current section in the next iteration.
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
    projected_df = territories_df.join(metadata_df, how="cross")
    print("Projected DataFrame:")
    print(projected_df)


if __name__ == "__main__":
    main()
