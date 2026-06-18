# Technical Implementation Plan: Tabularix

## 1. Major Components & Dependencies

1. **Project Skeleton (Rust/Python)**: `maturin`, `pyo3`, basic package structure.
2. **Core Data Structure & Parser (Rust)**: `calamine` for loading `.xlsx` files into an internal, mutable `Sheet` memory representation.
3. **Active Mutator API (Rust)**: `Sheet` manipulation algorithms (`unmerge_cells`, `search_and_drop_before`).
4. **Layex Pattern Engine (Rust)**: Parser for the String DSL and matching logic (entities, types, regex cardinality).
5. **Table Assembler (Rust)**: Converts `Sheet` blocks into an internal `Table` type, which supports exporting to Arrow, Parquet, Polars, and Pandas.
6. **Python Frontend (Python)**: Ergonomic classes (`Sheet`, `RowGroupMatcher`, `Table`, `TabularixError`) that wrap the PyO3 extensions with PEP8 style and type hints.
7. **Acceptance Testing Suite (Python/Robot)**: Robot Framework setup for the Product Owner to define behavior.

## 2. Implementation Order

- **Step 1: Scaffolding & CI Pipeline**: Initialize `maturin` project, `Cargo.toml`, `pyproject.toml`, and `justfile`. Set up basic PyO3 bindings and Robot framework.
- **Step 2: Excel Parsing & Sheet Core**: Implement Rust `calamine` wrapper to load an Excel sheet into a custom `Sheet` struct. Expose `load_workbook` to Python.
- **Step 3: Active Mutator API**: Implement `unmerge_cells` and `search_and_drop_before` on the `Sheet`. Expose to Python.
- **Step 4: Layex Engine MVP**: Build the pattern matching core (matching simple types and strings without complex cardinality yet). Expose `RowGroupMatcher`.
- **Step 5: Advanced Layex & Search**: Add cardinality (`*`, `+`) to the engine. Implement `search_row_group` and `extract_rows_between`.
- **Step 6: Table API & Data Export**: Implement `build_table_from_row_groups` returning an internal `Table` instance. Add methods to the `Table` to export to Parquet files, Arrow tables, Pandas, and Polars DataFrames.

## 3. General Implementation Rules

- **Documentation & Tests**: Every story implementation must include updating the project documentation if relevant, as well as updating/adding the tests.

## 4. Risks & Mitigations

- **Risk**: _Performance overhead crossing the Rust/Python boundary._
  **Mitigation**: The `Sheet` structure will remain entirely in Rust memory. Python will only hold pointer/handle references to it. Data only crosses the boundary at the very end when generating the final tables (which is zero-copy where possible).
- **Risk**: _Layex Engine Complexity (Regex for tabular data)._
  **Mitigation**: Start simple. MVP will only support basic exact string matches and primitive types (`[type:string]`, `[type:numeric]`). Cardinality and grouping will be added incrementally with strict unit tests.

## 5. Parallel vs Sequential Work

- **Sequential**: Steps 1 -> 2 -> 3 must be built sequentially. We cannot mutate a sheet if we cannot load it.
- **Parallel**:
    - The **Layex Pattern Engine** (Step 4 & 5) can be developed purely in Rust entirely in parallel to Step 3 (Active Mutator), as it just needs to operate on row slices.
    - **Robot Framework Acceptance Tests** can be written by the User (Product Owner) immediately in parallel, based on the expected API defined in the design spec.

## 6. Verification Checkpoints

1. **Checkpoint 1 (Post-Step 1)**: `just build` succeeds, PyO3 "Hello World" works, Robot Framework can execute a dummy test.
2. **Checkpoint 2 (Post-Step 3)**: Python can load `.xlsx` and run `sheet.unmerge_cells()`. Rust unit tests confirm sheet mutation is correct.
3. **Checkpoint 3 (Post-Step 5)**: The Layex DSL accurately finds headers and footers in sample data in Rust unit tests.
4. **Checkpoint 4 (Post-Step 6)**: The full pipeline runs from end-to-end, and Robot Framework acceptance tests pass for the new `Table` export methods.
