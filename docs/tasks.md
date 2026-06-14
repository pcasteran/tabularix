# Implementation Tasks: Tabularix

These tasks follow the phased Implementation Plan. Each task is designed to be completable in a single session and includes explicit acceptance criteria and verification steps.

## Step 1: Scaffolding & CI Pipeline

- [x] **Task 1.1: Initialize Maturin and PyO3 Project**
    - **Acceptance:** `mise.toml` is updated with `rust`, `python`, and `maturin` toolchains. Rust and Python are linked via a `maturin` project. A dummy `hello_world` function can be called from Python.
    - **Verify:** Run `mise install`, then `maturin develop` and verify in a Python REPL that `import tabularix` and `tabularix.hello_world()` work.
    - **Files:** `mise.toml`, `Cargo.toml`, `pyproject.toml`, `src/lib.rs`, `python/tabularix/__init__.py`.

- [x] **Task 1.2: Configure Task Runner and Linting**
    - **Acceptance:** `justfile` has recipes for building, testing, linting, and formatting. Rust `clippy` is configured with strict rules. The `prek` pre-commit setup is integrated.
    - **Verify:** Run `just build`, `just prek` (which runs pre-commit hooks), and `just test`. They should all pass cleanly.
    - **Files:** `justfile`, `Cargo.toml`, `.pre-commit-config.yaml`.

- [x] **Task 1.3: Setup Robot Framework**
    - **Acceptance:** A dummy Robot Framework test exists and can be executed via a `just` command, testing the dummy PyO3 `hello_world` output.
    - **Verify:** Run `just test-acceptance` and see the Robot Framework output `PASS`.
    - **Files:** `justfile`, `tests/hello.robot`, `tests/requirements.txt` (or similar for Python deps).

- [x] **Task 1.4: Initialize Project Documentation**
    - **Acceptance:** Use Zensical to initialize the documentation site structure. Add `just` recipes to serve and build the docs.
    - **Verify:** Run `just docs-serve` to see the local documentation site running.
    - **Files:** `justfile`, `zensical` config files, `docs/index.md`.

## Step 2: Excel Parsing & Sheet Core

- [x] **Task 2.1: Implement Rust Sheet Struct and Excel Loading**
    - **Acceptance:** Use `calamine` to parse an `.xlsx` file into an internal Rust `Sheet` struct containing the grid data. Include unit tests for loading a sample Excel file. Clean up and remove the temporary `hello_world` test function.
    - **Verify:** Run `cargo test` and ensure `Sheet` parsing tests pass (and `hello_world` has been removed).
    - **Files:** `Cargo.toml`, `src/sheet.rs`, `src/lib.rs`.

- [x] **Task 2.2: Expose `load_workbook` to Python**
    - **Acceptance:** Create a Python API `tabularix.load_workbook(path)` that returns a Python `Sheet` object (wrapping the Rust struct). Add Robot acceptance tests for loading.
    - **Verify:** Run `just test-acceptance` and ensure the Robot test can load an Excel file.
    - **Files:** `src/lib.rs`, `python/tabularix/__init__.py`, `tests/load.robot`, `docs/api.md`.

- [x] **Task 2.3: Implement SVG Sheet Export**
    - **Acceptance:** `Sheet` has a method `to_svg(path: str)` that renders the cell grid into a beautifully styled SVG file, handling cell types and merged cells correctly.
    - **Verify:** `just acceptance-test` runs a test that loads `sample.xlsx`, exports to SVG, and verifies the SVG file is created.
    - **Files:** `src/sheet.rs`, `python/tabularix/__init__.py`, `tests/svg.robot`.

## Step 3: Active Mutator API

- [ ] **Task 3.1: Implement and Expose `unmerge_cells`**
    - **Acceptance:** `Sheet` has an `unmerge_cells` method that fills merged cells with their parent values. The method is exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated with the new signatures, types, and docstrings. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/unmerge.robot`, `docs/api.md`.

- [ ] **Task 3.2: Implement and Expose `search_and_crop_before`**
    - **Acceptance:** `Sheet` has a `search_and_crop_before` method to crop rows before a marker text. The method is exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated with the new signatures, types, and docstrings. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/crop.robot`, `docs/api.md`.

- [x] **Task 3.3: Implement and Expose `get_cell_value` / `set_cell_value`**
    - **Acceptance:** `Sheet` has `get_cell_value(row, col)` and `set_cell_value(row, col, value)` methods to read and write cell values. The methods are exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated with signatures and docstrings. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/cell_values.robot`, `docs/api.md`.

- [ ] **Task 3.4: Implement and Expose `drop_row` / `drop_column`**
    - **Acceptance:** `Sheet` has `drop_row(row_idx)` and `drop_column(col_idx)` methods to delete specific rows or columns. The methods are exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated with signatures and docstrings. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/drop.robot`, `docs/api.md`.

- [ ] **Task 3.5: Implement and Expose `drop_rows_when_fill_ratio_less_than` / `drop_columns_when_fill_ratio_less_than`**
    - **Acceptance:** `Sheet` has methods to drop rows/columns where the proportion of non-empty cells is below a specified threshold. The methods are exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/fill_ratio.robot`, `docs/api.md`.

- [ ] **Task 3.6: Implement and Expose `drop_rows_when_entropy_less_than` / `drop_columns_when_entropy_less_than`**
    - **Acceptance:** `Sheet` has methods to drop rows/columns where the information entropy (data variability/uniqueness) is below a specified threshold. The methods are exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/entropy.robot`, `docs/api.md`.

- [ ] **Task 3.7: Implement and Expose `swap_rows` / `swap_columns`**
    - **Acceptance:** `Sheet` has `swap_rows(i, j)` and `swap_columns(i, j)` methods to reorder rows/columns. The methods are exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/swap.robot`, `docs/api.md`.

- [ ] **Task 3.8: Implement and Expose `repeat_row_cell` / `repeat_column_cell`**
    - **Acceptance:** `Sheet` has methods to fill adjacent empty cells in a row/column with the value of the nearest preceding non-empty cell. The methods are exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/repeat.robot`, `docs/api.md`.

- [ ] **Task 3.9: Implement and Expose `search_first_value` / `search_nth_value`**
    - **Acceptance:** `Sheet` has methods to search for values matching a query string and return their 0-based cell coordinates. The methods are exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/search.robot`, `docs/api.md`.

- [ ] **Task 3.10: Implement and Expose `crop_all`**
    - **Acceptance:** `Sheet` has a `crop_all` method to automatically trim empty rows and columns from all edges of the grid. The method is exposed to Python via PyO3. Type stubs in `python/tabularix/__init__.pyi` and public API documentation in `docs/api.md` are updated. Add Rust unit tests and Robot acceptance tests.
    - **Verify:** `cargo test` and `just acceptance-test` both pass.
    - **Files:** `src/sheet.rs`, `src/lib.rs`, `python/tabularix/__init__.py`, `python/tabularix/__init__.pyi`, `tests/crop_all.robot`, `docs/api.md`.
