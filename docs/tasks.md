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

- [ ] **Task 2.1: Implement Rust Sheet Struct and Excel Loading**
    - **Acceptance:** Use `calamine` to parse an `.xlsx` file into an internal Rust `Sheet` struct containing the grid data. Include unit tests for loading a sample Excel file.
    - **Verify:** Run `cargo test` and ensure `Sheet` parsing tests pass.
    - **Files:** `Cargo.toml`, `src/sheet.rs`, `src/lib.rs`.

- [ ] **Task 2.2: Expose `load_workbook` to Python**
    - **Acceptance:** Create a Python API `tabularix.load_workbook(path)` that returns a Python `Sheet` object (wrapping the Rust struct). Add Robot acceptance tests for loading.
    - **Verify:** Run `just test-acceptance` and ensure the Robot test can load an Excel file.
    - **Files:** `src/lib.rs`, `python/tabularix/__init__.py`, `tests/load.robot`, `docs/api.md`.

## Step 3: Active Mutator API

- [ ] **Task 3.1: Implement `unmerge_cells` in Rust**
    - **Acceptance:** `Sheet` struct has an `unmerge_cells` method that fills merged cells with their parent values. Include Rust unit tests.
    - **Verify:** `cargo test` passes.
    - **Files:** `src/sheet.rs`.

- [ ] **Task 3.2: Implement `search_and_crop_before` in Rust**
    - **Acceptance:** `Sheet` struct has a method to crop rows before a marker text. Include Rust unit tests.
    - **Verify:** `cargo test` passes.
    - **Files:** `src/sheet.rs`.

- [ ] **Task 3.3: Expose Active Mutators to Python**
    - **Acceptance:** PyO3 bindings for `unmerge_cells` and `search_and_crop_before` are exposed. Robot tests added.
    - **Verify:** `just test-acceptance` passes.
    - **Files:** `src/lib.rs`, `python/tabularix/__init__.py`, `tests/mutators.robot`, `docs/api.md`.
