from typing import Any

from ._tabularix import (  # ty: ignore[unresolved-import]
    RowGroupMatcher,
    RowPattern,
    Sheet,
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


__all__ = [
    "load_workbook",
    "Sheet",
    "Workbook",
    "RowPattern",
    "RowGroupMatcher",
    "value",
    "regex",
    "empty",
    "non_empty",
    "any",
]
