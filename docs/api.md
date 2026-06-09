---
title: API Reference
description: Public API documentation for Tabularix workbook and sheet classes.
---

# 📚 API Reference

This page describes the public API exposed by the Tabularix library.

---

## Functions

### `load_workbook(path: str) -> Workbook`
Loads an Excel workbook from the specified file path.

**Parameters:**
- `path` (str): The absolute or relative path to the `.xlsx` file.

**Returns:**
- `Workbook`: A `Workbook` instance containing the workbook sheets.

**Raises:**
- `FileNotFoundError`: If the file does not exist.
- `IOError`: If the file could not be parsed by Calamine.

---

### `index_to_a1(row: int, col: int) -> str`
Converts zero-based coordinates `(row, col)` into A1 notation (e.g., `(0, 0)` -> `"A1"`).

**Parameters:**
- `row` (int): Zero-based row index.
- `col` (int): Zero-based column index.

**Returns:**
- `str`: The corresponding A1 notation.

---

### `a1_to_index(a1: str) -> tuple[int, int]`
Converts A1 notation into zero-based coordinates `(row, col)` (e.g., `"A1"` -> `(0, 0)`).

**Parameters:**
- `a1` (str): Cell coordinate in A1 notation.

**Returns:**
- `tuple[int, int]`: The zero-based row and column indices as `(row, column)`.

**Raises:**
- `ValueError`: If the notation is invalid (e.g. missing coordinates, row <= 0).

---

## Classes

### `Workbook`
Represents an Excel workbook containing one or more sheets.

#### Methods

- **`active_sheet() -> Sheet`**
  Returns the active (first) sheet in the workbook.
  
- **`sheet_names() -> list[str]`**
  Returns a list containing the names of all worksheets in the workbook.
  
- **`get_sheet(name: str) -> Sheet`**
  Returns the worksheet with the specified name.
  
  **Raises:**
  - `KeyError`: If a sheet with that name does not exist.

---

### `Sheet`
Represents a single Excel worksheet as a grid of cell values.

#### Properties

- **`name`** *(str, read-only)*: The name of the worksheet.
- **`shape`** *(`tuple[int, int]`, read-only)*: The dimensions of the worksheet grid as `(rows, columns)`.

#### Methods

- **`cell(row: int, col: int) -> typing.Any`**
  Returns the value of the cell at the specified zero-based row and column coordinates.
  
  **Returns:**
  - `None` for empty cells.
  - `str`, `float`, `int`, or `bool` representing the cell's native type.
  
  **Raises:**
  - `IndexError`: If the coordinates are out of bounds.

- **`to_svg(path: str)`**
  Renders the worksheet grid into a beautifully styled SVG file, highlighting different cell types and correctly displaying merged cells.
  
  **Parameters:**
  - `path` (str): Output file path for the SVG.
  
  **Raises:**
  - `IOError`: If the SVG could not be written to the file path.
