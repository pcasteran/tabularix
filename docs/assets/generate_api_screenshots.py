import tabularix as tx

ASSETS_DIR = "docs/assets"


def _get_simple_sheet() -> tx.Sheet:
    wb = tx.load_workbook("tests/data/sample.xlsx")
    return wb.get_sheet("simple")


def generate_drop_row_screenshots():
    """Generate screenshots demonstrating the usage of the drop_row() API."""
    # Non merged row.
    sheet = _get_simple_sheet()
    sheet.drop_row(1)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_non_merged_after.svg")

    # Merged row (non-first).
    sheet = _get_simple_sheet()
    sheet.drop_row(4)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_merged_non_first_after.svg")

    # Merged row (first).
    sheet = _get_simple_sheet()
    sheet.drop_row(3)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_merged_first_after.svg")


def generate_drop_column_screenshots():
    """Load sample.xlsx, drop column 1 from the simple sheet, and generate before/after SVGs."""
    # Non merged column.
    sheet = _get_simple_sheet()
    sheet.drop_column(2)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_non_merged_after.svg")

    # Merged column (non-first).
    sheet = _get_simple_sheet()
    sheet.drop_column(1)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_merged_non_first_after.svg")

    # Merged column (first).
    sheet = _get_simple_sheet()
    sheet.drop_column(0)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_merged_first_after.svg")


def main():
    """Run all API documentation screenshot generator functions."""
    sheet_simple = _get_simple_sheet()
    sheet_simple.to_svg(f"{ASSETS_DIR}/sheet_simple.svg")

    generate_drop_row_screenshots()
    generate_drop_column_screenshots()

    print("Documentation API screenshots generated")


if __name__ == "__main__":
    main()
