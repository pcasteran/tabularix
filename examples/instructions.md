# Tabularix Usage & Strategy Guidelines

This document outlines the best practices and strategies for using the Tabularix framework to extract structured data from complex spreadsheets.

## 🧭 Resilient Extraction Philosophy

- **NEVER Use Fixed Coordinates:** Avoid hardcoding cell addresses (e.g., `C9:D10`, `B12:F15`) or fixed row/column offsets. Hardcoded layouts break when worksheets are updated, rows/columns are added/deleted, or styles change.
- **Rely on Framework Matching Capabilities:** Use dynamic anchors (using `RangeMatcher`, `RowPattern`, and relative queries like `search_range_relative`) to locate the boundaries of tables and metadata.

## 🔍 Resilient Metadata Extraction Strategy

When a worksheet contains key-value metadata scattered outside the main data tables:

1. **Define a Structural Row Pattern:** Match the rows containing the metadata using a `RangeMatcher` combined with generic cell rules (e.g., empty cells in unused columns, regex patterns for keys, and non-empty checks for values).
2. **Scan the Sheet:** Locate the metadata region using `sheet.search_range(metadata_matcher)`.
3. **Inspect the Matched Region Dynamically:** Iterate over the cells within the returned `Range` to find the keys and values. This isolates you from column shifting and insertion.

### 📅 Excel Date Serial Numbers

Excel represents dates as serial numbers (floating-point numbers representing the count of days since December 30, 1899). When extracting date values, check for float types and reconstruct the date representation programmatically:

```python
import datetime
base_date = datetime.date(1899, 12, 30)
date_str = (base_date + datetime.timedelta(days=int(excel_float_value))).isoformat()
```
