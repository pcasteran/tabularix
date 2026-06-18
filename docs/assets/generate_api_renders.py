import re

import tabularix as tx
from tabularix import Sheet

ASSETS_DIR = "docs/assets"


def generate_drop_row_renders(sheet_simple: Sheet):
    """Generate renders demonstrating the usage of the drop_row() API."""
    # Non merged row.
    sheet = sheet_simple.copy()
    sheet.drop_row(1)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_non_merged.svg")

    # Merged row (non-first).
    sheet = sheet_simple.copy()
    sheet.drop_row(4)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_merged_non_first.svg")

    # Merged row (first).
    sheet = sheet_simple.copy()
    sheet.drop_row(3)
    sheet.to_svg(f"{ASSETS_DIR}/drop_row_merged_first.svg")


def generate_drop_column_renders(sheet_simple: Sheet):
    """Generate renders demonstrating the usage of the drop_column() API."""
    # Non merged column.
    sheet = sheet_simple.copy()
    sheet.drop_column(2)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_non_merged.svg")

    # Merged column (non-first).
    sheet = sheet_simple.copy()
    sheet.drop_column(1)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_merged_non_first.svg")

    # Merged column (first).
    sheet = sheet_simple.copy()
    sheet.drop_column(0)
    sheet.to_svg(f"{ASSETS_DIR}/drop_column_merged_first.svg")


def generate_search_and_drop_renders(sheet_complex: Sheet):
    """Generate renders demonstrating the usage of the search_and_drop() API."""
    # Search a plain string and drop top from it.
    sheet = sheet_complex.copy()
    sheet.search_and_drop("Name", "top")
    sheet.to_svg(f"{ASSETS_DIR}/search_and_drop_str_top.svg")

    # Search a regex and drop bottom from it.
    sheet = sheet_complex.copy()
    sheet.search_and_drop(re.compile(r"Total \d{4}"), "bottom")
    sheet.to_svg(f"{ASSETS_DIR}/search_and_drop_regex_bottom.svg")


def main():
    """Run all API documentation render generator functions."""
    # Load the sheets from the sample workbook.
    wb = tx.load_workbook("tests/data/sample.xlsx")

    sheet_simple = wb.get_sheet("simple")
    sheet_simple.to_svg(f"{ASSETS_DIR}/sheet_simple.svg")

    sheet_complex = wb.get_sheet("complex")
    sheet_complex.to_svg(f"{ASSETS_DIR}/sheet_complex.svg")

    # Generate the renders for the APIs.
    generate_drop_row_renders(sheet_simple)
    generate_drop_column_renders(sheet_simple)
    generate_search_and_drop_renders(sheet_complex)

    print("Documentation API renders generated")


if __name__ == "__main__":
    main()
