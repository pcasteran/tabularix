# Spec: High-Level API Simplification for Tabularix

## Objective

Tabularix has a highly expressive but low-level API. To reduce boilerplate and simplify common extraction scenarios (such as vertical and horizontal tables, and extracting tables bounded by a header/footer), we want to introduce a higher-level Python-only API that abstracts matcher compiling, search directions, and relative location searches.

## Tech Stack

- **Language:** Python 3.12+
- **Dependencies:** PyArrow (existing dependency)
- **Framework Interface:** PyO3 bindings for Rust core

## Commands

- Compile / build bindings: `just build`
- Run acceptance tests: `just acceptance-test`
- Code quality / lints: `just prek`

## Project Structure

We will modify:

- [python/tabularix/**init**.py](file:///workspaces/tabularix/python/tabularix/__init__.py) -> To add aliases, modify constructors to accept varargs, and add function implementations.
- [python/tabularix/**init**.pyi](file:///workspaces/tabularix/python/tabularix/__init__.pyi) -> To add type stubs.
- [tests/](file:///workspaces/tabularix/tests) -> To add Robot Framework acceptance tests for the new API and refactor existing tests to use varargs.

## Code Style

- Follow PEP 8 formatting.
- Use strict type hinting with the native union operator `|` (PEP 604) rather than `typing.Union`.
- Docstrings must conform to Google style.
- Use type aliases `group` and `grid` for pattern declarations and type hints.

Example usage comparison:

```python
# Old Low-Level API
header_pattern = RangePattern1D([value("Region"), regex(r"^Q[1-4]$").repeat(4, 4)])
header_matcher = header_pattern.to_matcher(direction="LR")
header_range = sheet.search_range(header_matcher)

data_pattern = RangePattern2D([
    RangePattern1D([regex(r"^(?!Total).*$"), non_empty().repeat(4, 4)]).one_or_more()
])
data_matcher = data_pattern.to_matcher(outer_direction="TB", inner_direction="LR")
data_range = sheet.search_range_relative(data_matcher, below=header_range)
table = sheet.extract_table(data_range, header_range)

# New High-Level API (with clean varargs and aliases)
header = group(value("Region"), regex(r"^Q[1-4]$").repeat(4, 4))
data = grid(group(regex(r"^(?!Total).*$"), non_empty().repeat(4, 4)).one_or_more())

table = extract_table_with_header_and_data(sheet, header, data)
```

## Testing Strategy

We will add a new Robot Framework file `tests/high_level_api.robot` that verifies:

1. Extraction of vertical tables using `extract_table_with_header_and_data`.
2. Extraction of horizontal tables using `extract_table_with_header_and_data`.
3. Extraction of tables between a header and footer using `extract_table_between_header_and_footer`.

We will also update all existing tests to remove list brackets `[]` from `RangePattern1D` and `RangePattern2D` instantiations.

## Boundaries

- **Always do:** Compile with `just build` before testing. Keep Python and type stub files strictly synchronized.
- **Ask first:** Modifying Rust-level code.
- **Never do:** Use `typing.Union` in new or refactored signatures; use `|` instead.

## Success Criteria

1. The type checker (`ty` or `mypy`) passes for all Python modules, including stubs.
2. The `just prek` checks pass.
3. All Robot tests pass successfully when run via `just acceptance-test`.
4. Examples correctly load and extract tables using the new functions.

## Open Questions

- None.
