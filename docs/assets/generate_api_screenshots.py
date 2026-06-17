import tabularix as tx
from tabularix import Sheet

ASSETS_DIR = "docs/assets"


def generate_drop_row_screenshots(sheet_simple: Sheet):
    """Generate screenshots demonstrating the usage of the drop_row() API."""
    # Non merged row.
    sheet = sheet_simple.copy()
    sheet.drop_row(1)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_non_merged_after.svg")

    # Merged row (non-first).
    sheet = sheet_simple.copy()
    sheet.drop_row(4)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_merged_non_first_after.svg")

    # Merged row (first).
    sheet = sheet_simple.copy()
    sheet.drop_row(3)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_merged_first_after.svg")


def generate_drop_column_screenshots(sheet_simple: Sheet):
    """Generate screenshots demonstrating the usage of the drop_column() API."""
    # Non merged column.
    sheet = sheet_simple.copy()
    sheet.drop_column(2)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_non_merged_after.svg")

    # Merged column (non-first).
    sheet = sheet_simple.copy()
    sheet.drop_column(1)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_merged_non_first_after.svg")

    # Merged column (first).
    sheet = sheet_simple.copy()
    sheet.drop_column(0)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_merged_first_after.svg")


def main():
    """Run all API documentation screenshot generator functions."""
    # Load the simple sheet from the sample workbook.
    wb = tx.load_workbook("tests/data/sample.xlsx")
    sheet_simple = wb.get_sheet("simple")
    sheet_simple.to_svg(f"{ASSETS_DIR}/sheet_simple.svg")

    generate_drop_row_screenshots(sheet_simple)
    generate_drop_column_screenshots(sheet_simple)

    print("Documentation API screenshots generated")


if __name__ == "__main__":
    main()
