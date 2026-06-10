import openpyxl
import os


def generate():
    os.makedirs("tests/data", exist_ok=True)

    wb = openpyxl.Workbook()
    ws = wb.active
    ws.title = "Sheet1"

    # Add some simple data
    ws["A1"] = "Header1"
    ws["B1"] = "Header2"
    ws["A2"] = "ABC"
    ws["B2"] = 123.45
    ws["A3"] = "DEF"
    ws["B3"] = 678

    # Add some merged cells to test merges later
    # Merge A4:B5
    ws.merge_cells("A4:B5")
    ws["A4"] = "MergedValue"

    wb.save("tests/data/sample.xlsx")
    print("Sample Excel created at tests/data/sample.xlsx")


if __name__ == "__main__":
    generate()
