---
title: API Reference
description: Public API documentation for Tabularix workbook and sheet classes.
icon: lucide/library
---

# 📚 API Reference

This page describes the public API exposed by the Tabularix library.

---

## Functions

::: tabularix.load_workbook
    options:
      heading_level: 3

---

## Classes

::: tabularix.Workbook
    options:
      heading_level: 3

---

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

<!-- ::: tabularix.Sheet.drop_row
    options:
      heading_level: 4
      show_root_full_path: false

!!! example "Visual Transformation Example"
    | Before | After |
    | :---: | :---: |
    | ![Original Excel Sheet](assets/drop_row_before.svg) | ![Rendered SVG Output](assets/drop_row_after.svg) | -->
