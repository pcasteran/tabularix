from typing import cast

import polars as pl
from tabularix import (
    Sheet,
    any,
    empty,
    extract_table_with_header_and_data,
    grid,
    group,
    load_workbook,
    non_empty,
    regex,
    value,
)


def extract_metadata(sheet: Sheet) -> pl.DataFrame:
    """Extracts key-value metadata from the worksheet dynamically using the High-Level API.

    The metadata is stored horizontally in the spreadsheet: the first column
    contains the headers/keys ('Date', 'Fiscal Year'), and the second column
    contains the corresponding values.

    We use the High-Level API here because the metadata behaves as a single horizontal
    table. We define the patterns and let Tabularix locate and extract it in one call.
    """
    # 1. Define the one-dimensional patterns for the headers and values
    header_pattern = group(regex(r"^(Date|Fiscal Year)$").repeat(2, 2))
    data_pattern = group(non_empty().repeat(2, 2))

    # 2. Extract using the High-Level API.
    # main_direction="LR" (Left-to-Right) since the table flows horizontally.
    # inner_direction="TB" (Top-to-Bottom) since cells in each column flow vertically.
    table = extract_table_with_header_and_data(
        sheet,
        header_pattern,
        data_pattern,
        main_direction="LR",
        inner_direction="TB",
        clean_names=False,  # Retain original capitalization for keys ('Date', 'Fiscal Year')
    )

    # Convert the extracted Table to a Polars DataFrame.
    df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))

    return df


def extract_territory_tables(sheet: Sheet) -> pl.DataFrame:
    """Extracts and combines the four territory tables from the worksheet dynamically.

    Unlike the metadata block, the worksheet contains multiple stacked tables (North, South,
    East, West), each separated by a title cell and a dynamic amount of data rows.

    Because we must dynamically scroll down the worksheet to find the next title row,
    locate the header and footer ranges, and repeat this sequentially, a single call
    to the High-Level API is not sufficient. Instead, we use the Low-Level API to manually
    search coordinates and advance our scan cursor.
    """
    # Define group pattern to locate any of the territory titles.
    territory_pattern = group(regex(r"^(North|South|East|West)$"))
    territory_matcher = territory_pattern.to_matcher(direction="LR")

    # Define grid pattern for headers and footers using greedy matching rules
    # to dynamically capture the full width of the table.
    header_pattern = grid(
        # First header row.
        group(
            # The row starts with static text "Product".
            value("Product"),
            # Then we have zero or more occurrences of a group of two cells.
            group(
                regex(r"\d{4}"),  # The group starts with a year (4 digits).
                empty(),  # And finishes with an empty cell (merged with the one on the left).
            ).zero_or_more(),
        ),
        # Second header row.
        group(
            # The row starts with an empty cell (merged with the one above).
            empty(),
            # Then we have zero or more occurrences of a group of two cells.
            group(
                value("Expected"),  # The group starts with static text "Expected".
                value("Actual"),  # And finishes with static text "Actual".
            ).zero_or_more(),
        ),
    )
    header_matcher = header_pattern.to_matcher(outer_direction="TB", inner_direction="LR")

    footer_pattern = group(value("Total"), any().zero_or_more())
    footer_matcher = footer_pattern.to_matcher(direction="LR")

    dfs = []
    search_row = 0

    # Low-Level dynamic loop to scan the worksheet sequentially
    while search_row < sheet.shape[0]:
        # 1. Match territory title range dynamically below the last footer.
        territory_range = sheet.search_range(territory_matcher, start_row=search_row)
        if territory_range is None:
            break

        # Retrieve the territory name (e.g. "North", "South", etc.).
        territory = str(sheet.get_cell_value(territory_range.start_row, territory_range.start_col))

        # 2. Match the 2-row header range below the title.
        header_range = sheet.search_range(header_matcher, start_row=territory_range.end_row + 1)
        if header_range is None:
            raise ValueError(f"Header not found for territory {territory}")

        # 3. Match the footer relative to the header.
        footer_range = sheet.search_range_relative(footer_matcher, below=header_range)
        if footer_range is None:
            raise ValueError(f"Footer not found for territory {territory}")

        # 4. Extract data rows between header and footer.
        # Once ranges are located, we use the low-level extract_table with explicit ranges.
        data_range = sheet.get_range_between(header_range, footer_range)
        table = sheet.extract_table(
            data_range,
            header=header_range,
            clean_names=True,
            flatten_header=False,
        )

        # Convert the extracted Table to a Polars DataFrame.
        df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))

        # Add the territory name as the first column of the DataFrame.
        df = df.select([pl.lit(territory).alias("territory"), pl.all()])

        # Unpack the product struct.
        df = df.with_columns(pl.col("product").struct.field("product"))

        # Unpivot the year columns to rows (expected and actual sales).
        df = df.unpivot(
            on=None,  # All columns that are not in index will be used
            index=["territory", "product"],
            variable_name="year",
            value_name="metrics",
        )

        # Unnest the metrics struct containing expected and actual fields.
        df = df.unnest("metrics")

        dfs.append(df)

        # Move the search cursor below this table's footer.
        search_row = footer_range.end_row + 1

    if not dfs:
        raise ValueError("No territory tables were found in the worksheet.")

    # Combine all territory DataFrames.
    combined_df = pl.concat(dfs)

    return combined_df


def main() -> None:
    """Demonstrates extracting metadata and multiple sub-tables dynamically from a worksheet."""
    # Load the workbook.
    workbook = load_workbook("tests/data/sample.xlsx")

    # Get the target worksheet.
    sheet = workbook.get_sheet("multi-tables")

    # 1. Extract metadata.
    metadata_df = extract_metadata(sheet)
    print("Extracted Metadata DataFrame:")
    print(metadata_df)
    print("-" * 40)

    # 2. Extract and combine territory tables.
    territories_df = extract_territory_tables(sheet)
    print("Combined Territories DataFrame:")
    print(territories_df)
    print("-" * 40)

    # 3. Project metadata onto the combined territory rows using a cross join.
    projected_df = territories_df.join(metadata_df, how="cross")
    print("Projected DataFrame:")
    print(projected_df)


if __name__ == "__main__":
    main()
