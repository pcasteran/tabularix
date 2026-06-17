import tabularix as tx


def generate_drop_row_screenshots():
    """Load sample.xlsx, drop row 1 from the simple sheet, and generate before/after SVGs."""
    wb = tx.load_workbook("tests/data/sample.xlsx")
    sheet = wb.get_sheet("simple")

    sheet.to_svg("docs/assets/drop_row_before.svg")
    sheet.drop_row(1)
    sheet.to_svg("docs/assets/drop_row_after.svg")


def generate_drop_column_screenshots():
    """Load sample.xlsx, drop column 1 from the simple sheet, and generate before/after SVGs."""
    wb = tx.load_workbook("tests/data/sample.xlsx")
    sheet = wb.get_sheet("simple")

    sheet.to_svg("docs/assets/drop_column_before.svg")
    sheet.drop_column(1)
    sheet.to_svg("docs/assets/drop_column_after.svg")


def main():
    """Run all API documentation screenshot generator functions."""
    generate_drop_row_screenshots()
    generate_drop_column_screenshots()


if __name__ == "__main__":
    main()
