import tabularix as tx


def main():
    """Run a simple demonstration of Tabularix workbook loading and SVG export."""
    # Load a workbook
    wb = tx.load_workbook("tests/data/sample.xlsx")
    print(f"Sheets: {wb.sheet_names()}")

    sheet = wb.get_sheet("simple")

    shape = sheet.shape
    print(f"Shape: {shape}")

    sheet.to_svg("sheet.svg")


if __name__ == "__main__":
    main()
