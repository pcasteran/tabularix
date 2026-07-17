from typing import Any, Literal, Pattern

import pyarrow as pa

Direction = Literal["LR", "RL", "TB", "BT"]
RuleType = Literal["exact", "regex", "empty", "non_empty", "any"]

def load_workbook(path: str) -> Workbook:
    """Loads an Excel workbook from the specified file path.

    Args:
        path: Path to the .xlsx file.

    Returns:
        A Workbook object containing the parsed sheets.

    Raises:
        FileNotFoundError: If the file does not exist at the given path.
        IOError: If there is an error reading or parsing the file.
    """
    ...

class Sheet:
    """Represents a single worksheet as a grid of cell values. It provides methods to inspect and modify cells, update the sheet content, and render the sheet visually."""

    @property
    def name(self) -> str:
        """The name of the worksheet."""
        ...

    @property
    def shape(self) -> tuple[int, int]:
        """A tuple of (rows, columns) representing the size of the cell grid."""
        ...

    def get_cell_value(self, row: int, col: int) -> Any:
        """Retrieves the value of a cell at the given 0-based row and column indices.

        Args:
            row: 0-based row index.
            col: 0-based column index.

        Returns:
            The cell's value (None, str, float, int, bool, datetime.date, datetime.datetime, or an error string).

        Raises:
            IndexError: If the indices are out of bounds.
        """
        ...

    def set_cell_value(self, row: int, col: int, value: str) -> None:
        """Sets the value of a cell at the given 0-based row and column indices.

        Args:
            row: 0-based row index.
            col: 0-based column index.
            value: The string value to write to the cell.

        Raises:
            IndexError: If the indices are out of bounds.
            TypeError: If the value is not a string.
        """
        ...

    def to_svg(
        self,
        path: str,
        zero_based_indices: bool = True,
        anonymise_ranges: list[Range | str] | None = None,
    ) -> None:
        """Renders the worksheet to a beautifully styled SVG file.

        Args:
            path: Target file path where the SVG should be saved.
            zero_based_indices: If True, uses 0-based indexing for headers (default); otherwise 1-based.
            anonymise_ranges: Optional list of Range objects or A1 notation strings
                defining regions to anonymise.

        Raises:
            IOError: If writing to the destination path fails.
        """
        ...

    def drop_row(self, row_idx: int) -> None:
        """Deletes a row from the sheet.

        Args:
            row_idx: Zero-based index of the row to drop.

        Raises:
            IndexError: If row_idx is out of bounds or negative.
        """
        ...

    def drop_column(self, col_idx: int) -> None:
        """Deletes a column from the sheet.

        Args:
            col_idx: Zero-based index of the column to drop.

        Raises:
            IndexError: If col_idx is out of bounds or negative.
        """
        ...

    def copy(self) -> Sheet:
        """Creates an independent copy (deep copy) of the worksheet.

        Returns:
            A new Sheet instance that is a deep copy of this sheet.
        """
        ...

    def __copy__(self) -> Sheet:
        """Shallow copy protocol support (returns a deep copy)."""
        ...

    def __deepcopy__(self, memo: dict[int, Any]) -> Sheet:
        """Deep copy protocol support.

        Args:
            memo: The memoization dictionary used by Python's copy module.
        """
        ...

    def search_and_drop(
        self,
        str_or_regex: str | Pattern[str],
        drop_direction: Literal[
            "top", "bottom", "left", "right", "top_left", "top_right", "bottom_left", "bottom_right"
        ],
    ) -> tuple[tuple[int, int], tuple[int, int]]:
        """Searches for a text or compiled regex pattern and drops rows/columns in the specified direction. Regex matches use [Python flavor Regular Expressions](https://docs.python.org/3/library/re.html).

        Args:
            str_or_regex: A string to search for (exact match), or a compiled regex pattern (from `re.compile`).
            drop_direction: The direction in which to drop rows/columns relative to the match.

        Returns:
            A nested tuple of ((orig_row, orig_col), (new_row, new_col)) representing the 0-based coordinates of the matched cell before and after the drop operations.

        Raises:
            TypeError: If str_or_regex is neither a string nor a compiled regex pattern.
            ValueError: If the search term is not found, or if an invalid drop_direction is provided.
        """
        ...

    def search_range(
        self,
        matcher: RangeMatcher,
        start_row: int | None = None,
        end_row: int | None = None,
        start_col: int | None = None,
        end_col: int | None = None,
    ) -> Range | None:
        """Searches the worksheet (or a sub-grid of it) for the first matching sequence of rows.

        Args:
            matcher: The RangeMatcher pattern to search for.
            start_row: Optional 0-based row to start searching from (defaults to 0).
            end_row: Optional 0-based row to stop searching at (inclusive, defaults to last row).
            start_col: Optional 0-based column limit (inclusive, defaults to 0).
            end_col: Optional 0-based column limit (inclusive, defaults to last column).

        Returns:
            A Range enclosing the matched boundaries, or None if no match is found.

        Raises:
            ValueError: If start_row > end_row or start_col > end_col.
            IndexError: If any of the indices are out of bounds.
        """
        ...

    def search_range_relative(
        self,
        matcher: RangeMatcher,
        *,
        below: Range | None = None,
        above: Range | None = None,
        left: Range | None = None,
        right: Range | None = None,
    ) -> Range | None:
        """Searches for a range relative to one or more previously matched ranges.

        Args:
            matcher: The RangeMatcher pattern to search for.
            below: Optional Range boundary.
            above: Optional Range boundary.
            left: Optional Range boundary.
            right: Optional Range boundary.

        Returns:
            A Range enclosing the matched boundaries, or None if no match is found.

        Raises:
            ValueError: If relational boundaries conflict or if opposing spans do not align.
        """
        ...

    def get_range_between(self, start: Range, end: Range) -> Range:
        """Calculates the Range coordinates situated between two non-overlapping ranges.

        Supports ranges separated either vertically or horizontally.

        Args:
            start: The boundary Range positioned first (above or to the left).
            end: The boundary Range positioned second (below or to the right).

        Returns:
            A new Range object representing the coordinates between the start and end ranges.

        Raises:
            ValueError: If the ranges overlap, are separated diagonally, or if their
                        respective aligning spans (columns/rows) do not match.
        """
        ...

    def extract_table(
        self,
        data: Range,
        header: Range | None = None,
        clean_names: bool = False,
        flatten_header: bool = False,
        header_separator: str = "_",
    ) -> Table:
        """Extracts a structured Table from the specified data and header ranges.

        Args:
            data: The Range defining the rows and columns containing the table's data.
            header: An optional Range defining the column headers.
            clean_names: If True, cleans header column names to lower snake_case.
            flatten_header: If True, flattens multi-row headers into single strings
                            joined by header_separator. If False (default), represents
                            multi-row headers as nested structures.
            header_separator: Delimiter used to join multi-row headers when flatten_header
                              is True. Defaults to "_".

        Returns:
            A Table object wrapping the structured data.

        Raises:
            ValueError: If the header and data ranges do not align horizontally,
                        if they overlap, or if their column counts differ.
            IndexError: If the ranges exceed the current dimensions of the sheet.
        """
        ...

class Workbook:
    """Represents an Excel workbook containing multiple worksheets."""

    def active_sheet(self) -> Sheet:
        """Retrieves the active worksheet of the workbook.

        Returns:
            The active Sheet object.

        Raises:
            ValueError: If the workbook contains no sheets.
        """
        ...

    def sheet_names(self) -> list[str]:
        """Returns a list of all sheet names in the workbook.

        Returns:
            A list of sheet name strings.
        """
        ...

    def get_sheet(self, name: str) -> Sheet:
        """Retrieves a worksheet by its name.

        Args:
            name: The case-sensitive name of the sheet.

        Returns:
            The Sheet object.

        Raises:
            KeyError: If no worksheet with the given name exists.
        """
        ...

class CellRule:
    """Represents a rule for matching a single cell in a 1D sequence."""
    def repeat(self, min: int, max: int | None = None, greedy: bool = True) -> CellRule: ...
    def one_or_more(self, greedy: bool = True) -> CellRule: ...
    def zero_or_more(self, greedy: bool = True) -> CellRule: ...
    def optional(self, greedy: bool = True) -> CellRule: ...

def value(val: str) -> CellRule:
    """Creates a cell rule matching an exact string value.

    Args:
        val: The exact string value to match.

    Returns:
        A new CellRule instance.
    """
    ...

def regex(pattern: str | Pattern[str]) -> CellRule:
    """Creates a cell rule matching a regex pattern.

    Args:
        pattern: A regex string or a compiled regex pattern.

    Returns:
        A new CellRule instance.
    """
    ...

def empty() -> CellRule:
    """Creates a cell rule matching an empty cell.

    Returns:
        A new CellRule instance.
    """
    ...

def non_empty() -> CellRule:
    """Creates a cell rule matching a non-empty cell.

    Returns:
        A new CellRule instance.
    """
    ...

def any() -> CellRule:
    """Creates a wildcard cell rule matching any cell.

    Returns:
        A new CellRule instance.
    """
    ...

class RangePattern1D:
    """A direction-agnostic one-dimensional sequence of cell pattern rules."""

    def __init__(self, *elements: CellRule | RangePattern1D) -> None:
        """Initializes a new 1D pattern with the given rules and nested 1D patterns."""
        ...

    def repeat(self, min: int, max: int | None = None, greedy: bool = True) -> RangePattern1D:
        """Sets the cardinality of the 1D pattern to repeat a custom number of times.

        Args:
            min: Minimum number of repetitions.
            max: Optional maximum number of repetitions. If None, matches min or more.
            greedy: Whether matching should be greedy (default True).

        Returns:
            This RangePattern1D instance for chaining.
        """
        ...

    def one_or_more(self, greedy: bool = True) -> RangePattern1D:
        """Sets the cardinality of this 1D pattern to one-or-more (+).

        Args:
            greedy: Whether matching should be greedy.

        Returns:
            This RangePattern1D instance for chaining.
        """
        ...

    def zero_or_more(self, greedy: bool = True) -> RangePattern1D:
        """Sets the cardinality of this 1D pattern to zero-or-more (*).

        Args:
            greedy: Whether matching should be greedy.

        Returns:
            This RangePattern1D instance for chaining.
        """
        ...

    def optional(self, greedy: bool = True) -> RangePattern1D:
        """Sets the cardinality of this 1D pattern to optional (?).

        Args:
            greedy: Whether matching should be greedy.

        Returns:
            This RangePattern1D instance for chaining.
        """
        ...

    def to_matcher(self, direction: Direction = "LR") -> RangeMatcher:
        """Converts this 1D pattern to a RangeMatcher bound to the specified matching direction.

        Args:
            direction: The matching direction ("LR", "RL", "TB", "BT").

        Returns:
            A RangeMatcher instance.
        """
        ...

class RangePattern2D:
    """A direction-agnostic two-dimensional pattern consisting of a sequence of one-dimensional patterns."""

    def __init__(self, *patterns: RangePattern1D) -> None:
        """Initializes a new 2D pattern with the given 1D pattern list."""
        ...

    def to_matcher(self, outer_direction: Direction = "TB", inner_direction: Direction = "LR") -> RangeMatcher:
        """Converts this 2D pattern to a RangeMatcher bound to the specified scanning directions.

        Args:
            outer_direction: The direction of pattern sequence flow.
            inner_direction: The direction of cell sequence flow.

        Returns:
            A RangeMatcher instance.
        """
        ...

class Range:
    """Represents a region inside a worksheet enclosing absolute coordinate boundaries."""

    @property
    def start_row(self) -> int:
        """The 0-based index of the first row (inclusive)."""
        ...

    @property
    def end_row(self) -> int:
        """The 0-based index of the last row (inclusive)."""
        ...

    @property
    def start_col(self) -> int:
        """The 0-based index of the first column (inclusive)."""
        ...

    @property
    def end_col(self) -> int:
        """The 0-based index of the last column (inclusive)."""
        ...

    def __init__(self, start_row: int, end_row: int, start_col: int, end_col: int) -> None:
        """Initializes a new Range instance with absolute bounds."""
        ...

    @staticmethod
    def from_a1(a1_str: str) -> Range:
        """Creates a Range from an A1 notation string (e.g. "B2:D6" or "B2").

        Args:
            a1_str: The A1 notation range string.

        Returns:
            A Range instance enclosing the parsed coordinates.

        Raises:
            ValueError: If the A1 string is invalid or has unbounded formats (e.g. "A:B", "1:2").
        """
        ...

class RangeMatcher:
    """Represents a compiled pattern matcher bound to specific matching directions."""

    def __init__(
        self, row_patterns: list[RangePattern1D], outer_direction: Direction, inner_direction: Direction
    ) -> None:
        """Initializes a RangeMatcher with a list of 1D patterns and the orthogonal outer and inner directions."""
        ...

    def matches_range(self, rows: list[list[Any]]) -> bool:
        """Checks if a sequence of rows matches the range patterns.

        Args:
            rows: A list of rows, where each row is a list of cell values.

        Returns:
            True if all row patterns match the sequence of rows; False otherwise.
        """
        ...

class Table:
    """Represents a structured table extracted from a worksheet."""

    @property
    def shape(self) -> tuple[int, int]:
        """Returns the dimensions (num_rows, num_cols) of the table."""
        ...

    @property
    def columns(self) -> list[str]:
        """Returns the column names of the table."""
        ...

    def to_arrow(self) -> pa.Table:
        """Converts the Table to a PyArrow Table.

        Returns:
            A PyArrow Table.
        """
        ...

    def __arrow_c_stream__(self, requested_schema: Any = None) -> Any:
        """Exports the table data as an Arrow C stream pointer wrapped in a PyCapsule.

        This method implements the standard Arrow PyCapsule Interface for streams.

        Args:
            requested_schema: An optional capsule containing an Arrow C Schema structure.

        Returns:
            A PyCapsule object named "arrow_array_stream".
        """
        ...

group = RangePattern1D
grid = RangePattern2D

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
    ...

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
    """
    ...
