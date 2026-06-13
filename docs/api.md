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

## Classes

### `Workbook`

Represents an Excel workbook containing one or more sheets.

#### Methods

- **`active_sheet() -> Sheet`**
  Returns the active (first) sheet in the workbook.

- **`sheet_names() -> list[str]`**
  Returns a list containing the names of all worksheets in the workbook.

<!-- prettier-ignore -->
- **`get_sheet(name: str) -> Sheet`**
  Returns the worksheet with the specified name.

  **Raises:**

  - `KeyError`: If a sheet with that name does not exist.

---

### `Sheet`

Represents a single Excel worksheet as a grid of cell values.

#### Properties

- **`name`** _(str, read-only)_: The name of the worksheet.
- **`shape`** _(`tuple[int, int]`, read-only)_: The dimensions of the worksheet grid as `(rows, columns)`.

#### Methods

<!-- prettier-ignore -->
- **`cell(row: int, col: int) -> typing.Any`**
  Returns the value of the cell at the specified zero-based row and column coordinates.

  **Returns:**

  - `None` for empty cells.
  - `str`, `float`, `int`, or `bool` representing the cell's native type.

  **Raises:**

  - `IndexError`: If the coordinates are out of bounds.

<!-- prettier-ignore -->
- **`to_svg(path: str)`**
  Renders the worksheet grid into a beautifully styled SVG file, highlighting different cell types and correctly displaying merged cells.

  **Parameters:**

  - `path` (str): Output file path for the SVG.

  **Raises:**

  - `IOError`: If the SVG could not be written to the file path.
