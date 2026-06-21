from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    import pyarrow


from ._tabularix import (  # ty: ignore[unresolved-import]
    Range,
    RangeMatcher,
    RowPattern,
    Sheet,
    Table,
    Workbook,
    load_workbook,
)


def value(val: str) -> RowPattern:
    """Starts a row pattern with an exact cell value.

    Args:
        val: The exact string value to match.

    Returns:
        A new RowPattern instance containing the cell rule.
    """
    return RowPattern().value(val)


def regex(pattern: Any) -> RowPattern:
    """Starts a row pattern with a regex cell match.

    Args:
        pattern: A regex string or a compiled regex pattern.

    Returns:
        A new RowPattern instance containing the cell rule.
    """
    return RowPattern().regex(pattern)


def empty() -> RowPattern:
    """Starts a row pattern with an empty cell match.

    Returns:
        A new RowPattern instance containing the cell rule.
    """
    return RowPattern().empty()


def non_empty() -> RowPattern:
    """Starts a row pattern with a non-empty cell match.

    Returns:
        A new RowPattern instance containing the cell rule.
    """
    return RowPattern().non_empty()


def any() -> RowPattern:
    """Starts a row pattern with a wildcard cell match.

    Returns:
        A new RowPattern instance containing the cell rule.
    """
    return RowPattern().any()


def search_range_relative(
    self: Sheet,
    matcher: RangeMatcher,
    *,
    below: Range | None = None,
    above: Range | None = None,
    left: Range | None = None,
    right: Range | None = None,
) -> Range | None:
    """Searches for a range relative to other matched ranges.

    Args:
        self: The worksheet instance.
        matcher: The RangeMatcher pattern to search for.
        below: Optional Range boundary.
        above: Optional Range boundary.
        left: Optional Range boundary.
        right: Optional Range boundary.

    Returns:
        A Range if matched, or None.
    """
    if left is not None and right is not None:
        if right.end_col + 1 > left.start_col - 1:
            raise ValueError("Relational bounds conflict: right boundary is to the left of left boundary.")
    if above is not None and below is not None:
        if below.end_row + 1 > above.start_row - 1:
            raise ValueError("Relational bounds conflict: above boundary is below below boundary.")

    start_row = None
    end_row = None
    start_col = None
    end_col = None

    if below is not None:
        start_row = below.end_row + 1
        start_col = below.start_col
        end_col = below.end_col

    if above is not None:
        end_row = above.start_row - 1
        if start_col is None:
            start_col = above.start_col
            end_col = above.end_col
        elif start_col != above.start_col or end_col != above.end_col:
            raise ValueError("Column spans of 'below' and 'above' boundaries do not align.")

    if right is not None:
        start_col = right.end_col + 1
        start_row = right.start_row
        end_row = right.end_row

    if left is not None:
        end_col = left.start_col - 1
        if start_row is None:
            start_row = left.start_row
            end_row = left.end_row
        elif start_row != left.start_row or end_row != left.end_row:
            raise ValueError("Row spans of 'right' and 'left' boundaries do not align.")

    return self.search_range(
        matcher,
        start_row=start_row,
        end_row=end_row,
        start_col=start_col,
        end_col=end_col,
    )


Sheet.search_range_relative = search_range_relative


def to_arrow(self: Table) -> "pyarrow.Table":
    """Converts the Table to a PyArrow Table."""
    import pyarrow as pa

    reader = pa.RecordBatchReader.from_stream(self)
    return reader.read_all()


Table.to_arrow = to_arrow

__all__ = [
    "load_workbook",
    "Sheet",
    "Workbook",
    "RowPattern",
    "RangeMatcher",
    "Range",
    "Table",
    "value",
    "regex",
    "empty",
    "non_empty",
    "any",
]
