from __future__ import annotations

from typing import TYPE_CHECKING, Any, Literal

if TYPE_CHECKING:
    import pyarrow


from ._tabularix import (  # ty: ignore[unresolved-import]
    Range,
    RangeMatcher,
    Sheet,
    Table,
    Workbook,
    load_workbook,
)
from ._tabularix import (  # ty: ignore[unresolved-import]
    RangePattern1D as _RangePattern1D,
)

Direction = Literal["LR", "RL", "TB", "BT"]
RuleType = Literal["exact", "regex", "empty", "non_empty", "any"]


class CellRule:
    """Represents a rule for matching a single cell in a 1D sequence."""

    def __init__(self, rule_type: RuleType, value: Any = None):
        self.rule_type = rule_type
        self.value = value
        self.min = 1
        self.max = 1
        self.greedy = True

    def repeat(self, min: int, max: int | None = None, greedy: bool = True) -> CellRule:
        self.min = min
        self.max = max
        self.greedy = greedy
        return self

    def one_or_more(self, greedy: bool = True) -> CellRule:
        return self.repeat(1, None, greedy)

    def zero_or_more(self, greedy: bool = True) -> CellRule:
        return self.repeat(0, None, greedy)

    def optional(self, greedy: bool = True) -> CellRule:
        return self.repeat(0, 1, greedy)


class RangePattern1D:
    """A direction-agnostic one-dimensional sequence of cell pattern rules."""

    def __init__(self, *elements: CellRule | RangePattern1D):
        """Initializes a new 1D pattern with the given cell rules or nested 1D patterns."""
        self.elements = list(elements)
        self.min = 1
        self.max = 1
        self.greedy = True

    def repeat(self, min: int, max: int | None = None, greedy: bool = True) -> RangePattern1D:
        """Sets the cardinality of the 1D pattern to repeat a custom number of times or range."""
        self.min = min
        self.max = max
        self.greedy = greedy
        return self

    def one_or_more(self, greedy: bool = True) -> RangePattern1D:
        """Sets the cardinality of this 1D pattern to one-or-more (+)."""
        return self.repeat(1, None, greedy)

    def zero_or_more(self, greedy: bool = True) -> RangePattern1D:
        """Sets the cardinality of this 1D pattern to zero-or-more (*)."""
        return self.repeat(0, None, greedy)

    def optional(self, greedy: bool = True) -> RangePattern1D:
        """Sets the cardinality of this 1D pattern to optional (?)."""
        return self.repeat(0, 1, greedy)

    def to_rust(self) -> _RangePattern1D:
        """Compiles this Python pattern into a Rust-native _RangePattern1D object."""
        rust_p = _RangePattern1D()
        for element in self.elements:
            if isinstance(element, RangePattern1D):
                sub_rust = element.to_rust()
                rust_p = rust_p.group(sub_rust)
                if element.min != 1 or element.max != 1:
                    if element.min == 1 and element.max is None:
                        rust_p = rust_p.one_or_more(element.greedy)
                    elif element.min == 0 and element.max is None:
                        rust_p = rust_p.zero_or_more(element.greedy)
                    elif element.min == 0 and element.max == 1:
                        rust_p = rust_p.optional(element.greedy)
                    else:
                        rust_p = rust_p.repeat(
                            element.min, element.max if element.max is not None else -1, element.greedy
                        )
            elif isinstance(element, CellRule):
                if element.rule_type == "exact":
                    rust_p = rust_p.value(element.value)
                elif element.rule_type == "regex":
                    rust_p = rust_p.regex(element.value)
                elif element.rule_type == "empty":
                    rust_p = rust_p.empty()
                elif element.rule_type == "non_empty":
                    rust_p = rust_p.non_empty()
                elif element.rule_type == "any":
                    rust_p = rust_p.any()

                if element.min != 1 or element.max != 1:
                    if element.min == 1 and element.max is None:
                        rust_p = rust_p.one_or_more(element.greedy)
                    elif element.min == 0 and element.max is None:
                        rust_p = rust_p.zero_or_more(element.greedy)
                    elif element.min == 0 and element.max == 1:
                        rust_p = rust_p.optional(element.greedy)
                    else:
                        rust_p = rust_p.repeat(
                            element.min, element.max if element.max is not None else -1, element.greedy
                        )
        return rust_p

    def _compile_rust_pattern(self) -> _RangePattern1D:
        """Helper to compile this Python pattern and apply top-level cardinality metadata for Rust.

        Note: the cardinality-applying logic here is structurally similar to the inner loop in
        ``to_rust()``, but operates on a different Rust binding mechanism (attribute assignment
        vs. method calls). These two blocks are intentionally kept separate.
        """
        rust_p = self.to_rust()
        if self.min != 1 or self.max != 1:
            if self.min == 1 and self.max is None:
                rust_p.cardinality = "+"
                rust_p.greedy = self.greedy
            elif self.min == 0 and self.max is None:
                rust_p.cardinality = "*"
                rust_p.greedy = self.greedy
            elif self.min == 0 and self.max == 1:
                rust_p.cardinality = "?"
                rust_p.greedy = self.greedy
            else:
                rust_p.cardinality = f"{{{self.min},{self.max if self.max is not None else ''}}}"
                rust_p.greedy = self.greedy
        return rust_p

    def to_matcher(self, direction: Direction = "LR") -> RangeMatcher:
        """Converts this 1D pattern to a RangeMatcher bound to the specified matching direction."""
        if direction in ("LR", "RL"):
            outer_direction = "TB"
        else:
            outer_direction = "LR"

        rust_p = self._compile_rust_pattern()
        return RangeMatcher([rust_p], outer_direction, direction)


class RangePattern2D:
    """A direction-agnostic two-dimensional pattern consisting of a sequence of one-dimensional patterns."""

    def __init__(self, *patterns: RangePattern1D):
        """Initializes a new 2D pattern with the given list of 1D patterns."""
        self.patterns = list(patterns)

    def to_matcher(self, outer_direction: Direction = "TB", inner_direction: Direction = "LR") -> RangeMatcher:
        """Converts this 2D pattern to a RangeMatcher bound to the specified scanning directions."""
        rust_patterns = [pat._compile_rust_pattern() for pat in self.patterns]
        return RangeMatcher(rust_patterns, outer_direction, inner_direction)


def value(val: str) -> CellRule:
    """Creates a cell rule matching an exact string value."""
    return CellRule("exact", val)


def regex(pattern: Any) -> CellRule:
    """Creates a cell rule matching a regex pattern or string."""
    return CellRule("regex", pattern)


def empty() -> CellRule:
    """Creates a cell rule matching an empty cell."""
    return CellRule("empty")


def non_empty() -> CellRule:
    """Creates a cell rule matching a non-empty cell."""
    return CellRule("non_empty")


# Note: `any` intentionally shadows the Python built-in `builtins.any`. This module does not
# need the built-in, and the name provides a clean, consistent API surface for cell rule builders.
def any() -> CellRule:
    """Creates a wildcard cell rule matching any cell."""
    return CellRule("any")


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


def to_arrow(self: Table) -> pyarrow.Table:
    """Converts the Table to a PyArrow Table."""
    import pyarrow as pa

    reader = pa.RecordBatchReader.from_stream(self)
    return reader.read_all()


Table.to_arrow = to_arrow


# High-Level API Aliases
group = RangePattern1D
grid = RangePattern2D


def _compile_pattern(
    pattern: group | grid,
    outer_dir: Direction,
    inner_dir: Direction,
) -> RangeMatcher:
    """Compiles a 1D or 2D pattern into a RangeMatcher bound to layout directions.

    Args:
        pattern: The pattern (group or grid) to compile.
        outer_dir: The direction of pattern sequence flow.
        inner_dir: The direction of cell sequence flow.

    Returns:
        A compiled RangeMatcher.
    """
    if isinstance(pattern, grid):
        return pattern.to_matcher(outer_direction=outer_dir, inner_direction=inner_dir)
    else:
        return pattern.to_matcher(direction=inner_dir)


def _get_relative_constraint(direction: Direction, header_range: Range) -> dict[str, Range]:
    """Maps a layout direction to the corresponding relative search constraint.

    Args:
        direction: The main table layout direction.
        header_range: The matched header Range.

    Returns:
        A dict containing the relative search boundary (e.g. {"below": header_range}).
    """
    match direction:
        case "TB":
            return {"below": header_range}
        case "BT":
            return {"above": header_range}
        case "LR":
            return {"right": header_range}
        case "RL":
            return {"left": header_range}
        case _:
            raise ValueError(f"Invalid direction: {direction!r}. Must be one of 'TB', 'BT', 'LR', 'RL'.")


def extract_table_with_header_and_data(
    sheet: Sheet,
    header_pattern: group | grid,
    data_pattern: group | grid,
    *,
    main_direction: Direction = "TB",
    inner_direction: Direction = "LR",
    clean_names: bool = True,
    flatten_header: bool = True,
    header_separator: str = "_",
) -> Table:
    """Extracts a structured Table using explicit header and data patterns.

    This function locates the header row/columns in the worksheet and searches
    for the matching data records situated relative to the header.

    Args:
        sheet: The worksheet to extract the table from.
        header_pattern: The pattern representing the table's header.
        data_pattern: The pattern representing the table's data rows/columns.
        main_direction: The direction of record flow (e.g. "TB" for vertical tables).
        inner_direction: The direction of cells inside each record (e.g. "LR" for row cells).
        clean_names: If True, cleans header column names to lower snake_case.
        flatten_header: If True, flattens multi-row/column headers into single strings
            joined by header_separator.
        header_separator: The separator used to join multi-row headers when flatten_header is True.

    Returns:
        The extracted structured Table.

    Raises:
        ValueError: If the header pattern or data pattern cannot be found.
    """
    header_matcher = _compile_pattern(header_pattern, main_direction, inner_direction)
    header_range = sheet.search_range(header_matcher)
    if header_range is None:
        raise ValueError("Table header range not found.")

    data_matcher = _compile_pattern(data_pattern, main_direction, inner_direction)
    relative_kwargs = _get_relative_constraint(main_direction, header_range)

    data_range = sheet.search_range_relative(data_matcher, **relative_kwargs)
    if data_range is None:
        raise ValueError("Table data range not found relative to header.")

    return sheet.extract_table(
        data_range,
        header_range,
        clean_names=clean_names,
        flatten_header=flatten_header,
        header_separator=header_separator,
    )


def extract_table_between_header_and_footer(
    sheet: Sheet,
    header_pattern: group | grid,
    footer_pattern: group | grid,
    *,
    main_direction: Direction = "TB",
    inner_direction: Direction = "LR",
    clean_names: bool = True,
    flatten_header: bool = True,
    header_separator: str = "_",
) -> Table:
    """Extracts a structured Table situated between a matched header and footer.

    This function locates the header and footer in the worksheet and extracts the
    region situated between them.

    Args:
        sheet: The worksheet to extract the table from.
        header_pattern: The pattern representing the table's header.
        footer_pattern: The pattern representing the table's footer.
        main_direction: The direction of record flow (e.g. "TB" for vertical tables).
        inner_direction: The direction of cells inside each record (e.g. "LR" for row cells).
        clean_names: If True, cleans header column names to lower snake_case.
        flatten_header: If True, flattens multi-row/column headers into single strings
            joined by header_separator.
        header_separator: The separator used to join multi-row headers when flatten_header is True.

    Returns:
        The extracted structured Table.

    Raises:
        ValueError: If the header pattern or footer pattern cannot be found.

    Note:
        The footer is searched relative to the **header** (not the data area). Any match
        positioned after the header boundary — in the ``main_direction`` — is considered
        a valid footer candidate. The data range is then computed as the region between
        the header and footer.
    """
    header_matcher = _compile_pattern(header_pattern, main_direction, inner_direction)
    header_range = sheet.search_range(header_matcher)
    if header_range is None:
        raise ValueError("Table header range not found.")

    footer_matcher = _compile_pattern(footer_pattern, main_direction, inner_direction)
    relative_kwargs = _get_relative_constraint(main_direction, header_range)

    footer_range = sheet.search_range_relative(footer_matcher, **relative_kwargs)
    if footer_range is None:
        raise ValueError("Table footer range not found relative to header.")

    data_range = sheet.get_range_between(header_range, footer_range)

    return sheet.extract_table(
        data_range,
        header_range,
        clean_names=clean_names,
        flatten_header=flatten_header,
        header_separator=header_separator,
    )


__all__ = [
    "load_workbook",
    "Sheet",
    "Workbook",
    "RangePattern1D",
    "RangePattern2D",
    "RangeMatcher",
    "Range",
    "Table",
    "value",
    "regex",
    "empty",
    "non_empty",
    "any",
    "group",
    "grid",
    "extract_table_with_header_and_data",
    "extract_table_between_header_and_footer",
]
