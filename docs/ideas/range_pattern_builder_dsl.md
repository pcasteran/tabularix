# RangePattern Builder DSL

## Problem Statement

How Might We enable non-technical users to define and compose complex spreadsheet layout patterns without writing verbose Python code?

---

## Recommended Direction

We have designed a compact, visual, and highly consistent shorthand string DSL for defining `RangePattern1D` (groups) and `RangePattern2D` (grids).

By wrapping cell matching rules in square brackets `[...]`, the syntax visually represents a "grid cell", which is extremely intuitive for spreadsheet users. The syntax supports key-value attribute matching with a colon (`:`) separator, and provides both descriptive full forms (for self-documentation) and short forms (for rapid typing).

---

## Key Assumptions to Validate

- [ ] The custom parser correctly handles semicolons inside quoted regex strings without triggering row splitting.
- [ ] Error messages are clear and helpful when a user attempts to define a 2D pattern without wrapping rows in parentheses.
- [ ] The performance overhead of the parsing layer is negligible during sheet extraction.

---

## Architecture: Why Implement in the Python Layer vs. Rust Core?

We have chosen to implement the DSL parsing layer entirely in the **Python layer** rather than the **Rust core** for the following reasons:

1. **Alignment with Python Wrapper's Role**:
   The existing codebase is structured such that PyO3 Rust classes (`_RangePattern1D`, `Workbook`, `Sheet`, etc.) act as raw, low-level execution engines. The high-level Python layer (`python/tabularix/__init__.py`) wraps these raw classes and exposes developer-friendly classes (`RangePattern1D`, `RangePattern2D`, `CellRule`), method aliases (like `group` and `grid`), and custom Python exceptions. A parser that constructs these Python wrappers fits naturally in the Python wrapper layer.
2. **Error Diagnostics and Developer UX**:
   A primary goal of the DSL is to be user-friendly. Providing high-quality syntax error tracebacks (e.g. indicating character offsets and pointing to unmatched parentheses or brackets) is significantly easier to write, format, and raise in Python using Python's native string processing and exception systems, rather than passing syntax errors across the PyO3 binary boundary.
3. **Keep Rust Core Slim and Fast to Compile**:
   Implementing parsing in Rust would require adding parser-generator dependencies (like `nom`, `pest`, or `lalrpop`) to `Cargo.toml`, increasing compilation overhead and dependency footprints. Writing a clean, zero-dependency recursive-descent lexer and parser in Python avoids compile-time overhead and simplifies the codebase.
4. **Performance Overhead is Negligible**:
   Pattern parsing is a one-off compilation step that happens once per layout declaration at script initialization. The CPU time spent parsing a ~100-character DSL string is a few microseconds in Python, which is completely negligible compared to sheet loading, layout evaluation, and Arrow table extraction.

---

## MVP Scope

### In Scope

1. **Parser & Lexer (`python/tabularix/parser.py`)**:
    - Zero-dependency recursive-descent lexer and parser.
    - Grammars for cell rules inside `[...]`:
        - Literal matches: `[value: "Text"]` or `[v: "Text"]`
        - Regex matches: `[regex: "^Q[1-4]$"]` or `[r: "^Q[1-4]$"]`
        - Cell states: `[empty]` / `[e]`, `[non_empty]` / `[ne]`, `[any]` / `[a]`
    - Suffix quantifiers: `+`, `+?`, `*`, `*?`, `?`, `??`, `{n}`, `{n}?`, `{min,max}`, `{min,max}?`.
    - Parenthesized groups `(...)` for nested structures.
    - Semicolon-delimited rows for 2D patterns, requiring each row to be parenthesized: `(row1) ; (row2)+`.
2. **Public Helper Entrypoints**:
    - `parse_pattern_1d(pattern_str: str) -> RangePattern1D`
    - `parse_pattern_2d(pattern_str: str) -> RangePattern2D`
3. **Stringification (`__str__` / `__repr__`)**:
    - `__str__` on rules and patterns produces the clean short-form DSL for round-tripping.
    - `__repr__` produces valid copy-pasteable Python code constructors.
4. **Documentation**:
    - Create a dedicated user documentation page `docs/dsl.md` detailing the DSL syntax, rules, short forms, and 2D pattern structure.
    - Register the new page in `zensical.toml`.

### Out of Scope / Not Doing

- **Visual DSL Builder**: Not building a GUI builder; the string-based DSL in YAML/TOML/Python is sufficient for business analysts.
- **Config Loader (`extract_table_from_config`)**: Decoupled configuration loading from the core package to keep the extraction API flexible.

---

## Proposed Technical Changes

### 1. `python/tabularix/parser.py` [NEW]

Implement the custom lexer and parser.

### 2. `python/tabularix/__init__.py` [MODIFY]

- Inject `__str__` and `__repr__` methods onto:
    - `CellRule`
    - `RangePattern1D`
    - `RangePattern2D`
- Expose public APIs `parse_pattern_1d` and `parse_pattern_2d`.

### 3. `python/tabularix/__init__.pyi` [MODIFY]

Declare types and functions for static checking.

### 4. `docs/dsl.md` [NEW]

Dedicated documentation page explaining DSL syntax, formatting examples, and usage.

### 5. `zensical.toml` [MODIFY]

Register the new Layout Expressions DSL page under the main navigation.

### 6. `tests/test_parser.py` [NEW]

Add unit tests for the lexer, parser, and stringifier.

### 7. `tests/high_level_api.robot` [MODIFY]

Add acceptance test cases validating parsed patterns in sheet table extraction.

---

## Verification Plan

### Automated Tests

Run tests with maturin build:

```bash
just unit-test
just acceptance-test
```

### Static Analysis

Run formatting and quality gates:

```bash
just prek
```
