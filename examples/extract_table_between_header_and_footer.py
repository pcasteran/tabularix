import tabularix as tx
from tabularix import RowGroupMatcher, regex, value


def main() -> None:
    """Extracts a table between header and footer relative boundaries."""
    # Load the "complex" sheet of the sample workbook.
    wb = tx.load_workbook("tests/data/sample.xlsx")
    sheet = wb.get_sheet("complex")

    # Define matcher for the header and footer rows.
    header_matcher = RowGroupMatcher().row(
        value("Region").regex(r"^Q[1-4]$").repeat(4, max=4)  # Quarter header: Q1, Q2, Q3, Q4
    )

    footer_matcher = RowGroupMatcher().row(
        regex(r"^Total \d{4}$").any().repeat(4, max=4)  # Quarters total amount
    )

    # Search for the header and footer row groups.
    header = sheet.search_row_group(header_matcher)
    if header is None:
        raise ValueError("Header not found")

    footer = sheet.search_row_group(footer_matcher)
    if footer is None:
        raise ValueError("Footer not found")


if __name__ == "__main__":
    main()
