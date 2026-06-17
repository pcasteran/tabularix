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
            The cell's value (None, str, float, int, bool, or an error string).

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

    def to_svg(self, path: str, zero_based_indices: bool = True) -> None:
        """Renders the worksheet to a beautifully styled SVG file.

        Args:
            path: Target file path where the SVG should be saved.
            zero_based_indices: If True, uses 0-based indexing for headers (default); otherwise 1-based.

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
