import tabularix as tx
from tabularix import RangeMatcher, regex


def extract_metadata(sheet: tx.Sheet) -> dict:
    """Extracts key-value metadata from the worksheet."""
    # Define a RangeMatcher to locate the metadata block dynamically.
    # It searches for rows that have a cell containing "Date" or "Fiscal Year", and a non-empty value cell.
    metadata_matcher = (
        RangeMatcher()
        # Row pattern.
        .row(regex(r"^(Date|Fiscal Year)$").non_empty()).one_or_more()
    )

    metadata_range = sheet.search_range(metadata_matcher)
    if metadata_range is None:
        raise ValueError("Metadata block not found in the worksheet.")

    metadata = {}
    # Iterate through the matched rows and search dynamically for the key-value cells.
    # This prevents errors if columns shift or if key/value pairs move within the matched range.
    for row_idx in range(metadata_range.start_row, metadata_range.end_row + 1):
        for col_idx in range(metadata_range.start_col, metadata_range.end_col):
            cell_val = sheet.get_cell_value(row_idx, col_idx)
            if cell_val in ("Date", "Fiscal Year"):
                # Value is in the adjacent cell to the right
                val = sheet.get_cell_value(row_idx, col_idx + 1)

                # If key is 'Date' and value is an Excel date serial number (float)
                if cell_val == "Date" and isinstance(val, (int, float)):
                    import datetime

                    base_date = datetime.date(1899, 12, 30)
                    val = (base_date + datetime.timedelta(days=int(val))).isoformat()
                elif hasattr(val, "isoformat"):
                    val = val.isoformat()
                elif hasattr(val, "strftime"):
                    val = val.strftime("%Y-%m-%d")

                metadata[str(cell_val)] = val
                break

    return metadata


def main() -> None:
    # Load the workbook
    workbook = tx.load_workbook("tests/data/sample.xlsx")

    # Get the target worksheet
    sheet = workbook.get_sheet("multi-tables")

    # Extract metadata using resilient RangeMatcher
    metadata = extract_metadata(sheet)
    print("Extracted Metadata:")
    print(metadata)


if __name__ == "__main__":
    main()
