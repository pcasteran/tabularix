import tabularix as tx


def main():
    """Run a simple demonstration of Tabularix workbook loading and SVG export."""
    # Load a workbook
    wb = tx.load_workbook("tests/data/sample.xlsx")
    print(wb.sheet_names())

    sheet = wb.get_sheet("Sheet1")

    shape = sheet.shape
    print(shape)

    sheet.to_svg("a.svg")


if __name__ == "__main__":
    main()
