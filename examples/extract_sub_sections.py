from typing import cast

import polars as pl
import tabularix as tx
from tabularix import (
    Sheet,
    any,
    group,
    load_workbook,
    regex,
    value,
)


def extract_sub_sections(sheet: Sheet) -> pl.DataFrame:
    """Extracts tables containing sub-headers and sub-footers from the worksheet dynamically.

    The worksheet contains a main header row ("Project", "Budget", "Spent", "Remaining")
    followed by multiple department sub-sections (Development Department, Marketing Department,
    Sales Department). Each sub-section starts with a merged sub-header row, contains several
    data rows, and finishes with a sub-footer row ("Total"). Finally, the entire table
    finishes with a main footer row ("Grand Total").

    We first anchor the column boundaries using the main header row, then dynamically scan
    down the worksheet to extract each sub-section's data between its sub-header and sub-footer.
    """
    # 1. Main Header pattern to anchor column boundaries
    main_header_pattern = group(
        value("Project"),
        value("Budget"),
        value("Spent"),
        value("Remaining"),
    )
    main_header_matcher = main_header_pattern.to_matcher(direction="LR")

    main_header_range = sheet.search_range(main_header_matcher)
    if main_header_range is None:
        raise ValueError("Main table header not found.")

    # 2. Sub-header pattern matching department names (e.g. "Development Department")
    sub_header_pattern = group(regex(r"^.+\s+Department$"))
    sub_header_matcher = sub_header_pattern.to_matcher(direction="LR")

    # 3. Sub-footer pattern matching section totals (starts with "Total")
    sub_footer_pattern = group(value("Total"), any().zero_or_more())
    sub_footer_matcher = sub_footer_pattern.to_matcher(direction="LR")

    dfs = []
    search_row = main_header_range.end_row + 1

    # Dynamic scanning loop to extract each department sub-section
    while search_row < sheet.shape[0]:
        # Match next sub-header (department title) starting at search_row
        sub_header_range = sheet.search_range(sub_header_matcher, start_row=search_row)
        if sub_header_range is None:
            break

        # Retrieve the department name from the sub-header cell
        department = str(sheet.get_cell_value(sub_header_range.start_row, sub_header_range.start_col))

        # Match the sub-footer ("Total") relative to the sub-header (below sub_header_range)
        sub_footer_range = sheet.search_range_relative(sub_footer_matcher, below=sub_header_range)
        if sub_footer_range is None:
            raise ValueError(f"Sub-footer not found for department '{department}'.")

        # Dynamically calculate the row bounds situated between sub-header and sub-footer
        raw_data_range = sheet.get_range_between(sub_header_range, sub_footer_range)

        # Align column span of the data range with the main header column boundaries
        data_range = tx.Range(
            raw_data_range.start_row,
            raw_data_range.end_row,
            main_header_range.start_col,
            main_header_range.end_col,
        )

        # Extract structured table using the main_header_range for column headers
        table = sheet.extract_table(
            data_range,
            header=main_header_range,
            clean_names=True,
        )

        # Convert to a Polars DataFrame
        df = cast(pl.DataFrame, pl.from_arrow(table.to_arrow()))

        # Prepend department name as the first column
        df = df.select([pl.lit(department).alias("department"), pl.all()])

        # Note: The 'remaining' column contains null values because formula cells in this test sheet
        # were generated without cached values. In real-world Excel files saved by spreadsheet software,
        # formula cells contain cached values that Tabularix reads directly.

        dfs.append(df)

        # Advance search cursor below the current sub-footer
        search_row = sub_footer_range.end_row + 1

    if not dfs:
        raise ValueError("No department sub-sections were found in the worksheet.")

    # Combine all department DataFrames into a single, clean DataFrame
    combined_df = pl.concat(dfs)

    return combined_df


def main() -> None:
    """Demonstrates extracting a table containing sub-headers and sub-footers dynamically."""
    # Load the workbook.
    workbook = load_workbook("tests/data/sample.xlsx")

    # Get the target worksheet.
    sheet = workbook.get_sheet("sub-sections")

    # Extract department sub-sections.
    df = extract_sub_sections(sheet)
    print("Extracted Sub-sections DataFrame:")
    print(df)


if __name__ == "__main__":
    main()
