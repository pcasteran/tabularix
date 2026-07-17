# Layout-Safe Spreadsheet Anonymiser

## Problem Statement

How might we render a visual and structural representation of a worksheet that anonymises cell values within user-specified ranges—matching numbers, dates, and plain text with secure, cell-specific random offsets and placeholders—while replacing formula cells with a distinct visual placeholder?

## Recommended Direction

We will update [to_svg](file:///workspaces/tabularix/python/tabularix/__init__.py#L15) to accept an optional list of `anonymise_ranges`.

During workbook loading ([load_workbook_impl](file:///workspaces/tabularix/src/sheet.rs#L1016)), we will load formula cells from calamine's `worksheet_formula` and represent them using a new enum variant `CellValue::Formula(String, Box<CellValue>)`. This ensures that even formulas with empty cached properties are tracked.

When rendering the SVG:

1.  **Formulas**: Any cell containing a formula gets rendered with a light-grey background (`rect-formula` style) and the text `"<formula>"` in place of its value.
2.  **Anonymised Values**: For cells inside the `anonymise_ranges`, we generate random values that match their original data type:
    - **Floats / Integers**: Scaled by a _unique, fresh random factor_ in `[0.1, 10]` generated per cell, keeping the original decimal precision and order of magnitude.
    - **Dates / DateTimes**: Shifted by a _unique, fresh random offset_ of `[-365, 365]` days generated per cell.
    - **All Strings**: Treated identically, regardless of whether they represent text or formatted numbers. String values are mapped to a temporary unique placeholder (e.g. `"Text_1"`, `"Text_2"`) generated per unique string within the range, preserving structural alignment across rows.

### Python API

```python
def to_svg(
    self: Sheet,
    path: str,
    zero_based_indices: bool = False,
    *,
    anonymise_ranges: list[Range | str] | None = None,
) -> None:
    """Renders the worksheet's structure into a beautifully styled SVG layout file.

    Args:
        path: Path where the SVG will be written.
        zero_based_indices: If True, uses zero-based coordinate headers.
        anonymise_ranges: Optional list of range(s) to anonymise. Only cells in
            these ranges will have their values obfuscated.
    """
```

## Key Assumptions to Validate

- [ ] **Formula Range Alignment**: Ensure calamine's `worksheet_formula` range coordinate bounds map 1:1 with `worksheet_range` coordinate bounds in all loaded sheets.

## MVP Scope

- **Formula Type**: Add `CellValue::Formula(String, Box<CellValue>)` to [CellValue](file:///workspaces/tabularix/src/sheet.rs#L12) enum. Update matches in `matcher.rs` and `sheet.rs`.
- **Formula Loading**: Modify `load_workbook_impl` to load worksheet formulas and wrap values inside `CellValue::Formula`.
- **SVG Rendering**:
    - Draw formula cells with a light-grey fill (`#f3f4f6`) and display the text `"<formula>"`.
    - Draw normal cells as before.
- **Anonymisation Filter**:
    - For cells inside `anonymise_ranges`:
        - Obfuscate numbers (cell-specific random scale factor).
        - Obfuscate dates (cell-specific random day offset).
        - Obfuscate all strings using a deterministic lookup map (e.g., mapping `"Alice"` to `"Text_1"`).

## Not Doing (and Why)

- **Specialized Currency Regex Parsing**: Removed to avoid formatting complexity. All cell values parsed as strings by the reader are anonymised using the generic string placeholder logic.
- **Excel-to-Excel Obfuscation (`.xlsx` output)**: Avoided to keep Python package dependencies lightweight. SVG is natively supported by modern LLMs and contains all visual/structural cues needed.

## Open Questions

- Should the actual formula string (e.g. `SUM(B4:B7)`) be included in the SVG tag as a metadata tooltip/attribute for advanced LLMs?
