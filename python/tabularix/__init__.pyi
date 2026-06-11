from typing import Any

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

def index_to_a1(row: int, col: int) -> str:
    """Converts 0-based row and column indices to Excel A1 cell notation.

    Args:
        row: The 0-based row index (e.g., 0 for row 1).
        col: The 0-based column index (e.g., 0 for column A).

    Returns:
        The A1 notation string (e.g., "A1", "B2", "AA10").

    Raises:
        ValueError: If row or column index is negative.
    """
    ...

def a1_to_index(a1: str) -> tuple[int, int]:
    """Converts an Excel A1 cell notation string to 0-based row and column indices.

    Args:
        a1: The A1 notation string (e.g., "A1", "B2", "AA10").

    Returns:
        A tuple of (row_index, col_index).

    Raises:
        ValueError: If the A1 notation is invalid (e.g. empty, missing row number, or row index is 0).
    """
    ...

class Sheet:
    """Represents an Excel worksheet containing a grid of cells."""

    @property
    def name(self) -> str:
        """The name of the worksheet."""
        ...

    @property
    def shape(self) -> tuple[int, int]:
        """A tuple of (rows, columns) representing the size of the cell grid."""
        ...

    def cell(self, row: int, col: int) -> Any:
        """Retrieves the value of a cell at the given 0-based row and column indices.

        Args:
            row: 0-based row index.
            col: 0-based column index.

        Returns:
            The cell's value (None, str, float, int, bool, or an error string).

        Raises:
            IndexError: If the indices are out of bounds.
        """
        ...

    def to_svg(self, path: str) -> None:
        """Renders the worksheet to a beautifully styled SVG file.

        Args:
            path: Target file path where the SVG should be saved.

        Raises:
            IOError: If writing to the destination path fails.
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
