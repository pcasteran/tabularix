---
title: API Reference
description: Public API documentation for Tabularix workbook and sheet classes.
icon: lucide/library
---

# 📚 API Reference

This page describes the public API exposed by the Tabularix library.

---

## Functions

<!-- prettier-ignore -->
::: tabularix.load_workbook
    options:
      heading_level: 3

---

## Classes

<!-- prettier-ignore -->
::: tabularix.Workbook
    options:
      heading_level: 3

---

<!-- prettier-ignore -->
::: tabularix.Sheet
    options:
      heading_level: 3
      members:
        - name
        - shape
        - get_cell_value
        - set_cell_value
        - to_svg

<!-- drow_row() -->

<!-- prettier-ignore -->
::: tabularix.Sheet.drop_row
    options:
      heading_level: 4
      show_root_full_path: false

<!-- prettier-ignore -->
!!! example "Example"
    ```python
    sheet.drop_row(1)
    ```

    | Before | After |
    | :---: | :---: |
    | ![Original Excel Sheet](assets/drop_row_before.svg) | ![Rendered SVG Output](assets/drop_row_after.svg) |

<!-- drop_column() -->

<!-- prettier-ignore -->
::: tabularix.Sheet.drop_column
    options:
      heading_level: 4
      show_root_full_path: false

<!-- prettier-ignore -->
!!! example "Example"
    ```python
    sheet.drop_column(1)
    ```

    | Before | After |
    | :---: | :---: |
    | ![Original Excel Sheet](assets/drop_column_before.svg) | ![Rendered SVG Output](assets/drop_column_after.svg) |
