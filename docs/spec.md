# Spec: Tabularix Core API & Engine

## Objective

Build **Tabularix**, a high-performance framework for identifying, extracting, and organizing "hidden data" from highly variable Excel files (`.xlsx`). It will provide layout-flexible APIs (Active Mutator & Pattern Matching via Layex) without requiring a proprietary configuration DSL. The framework will be a Python package backed by a native Rust core, with outputs converted seamlessly to Apache Arrow tables.

**Success Criteria:**

- Native speed extraction of 100MB+ spreadsheets without memory bloat.
- Ergonomic Python API matching the example blueprint from `tabularix_core_design.md`.
- Acceptance tests written in Robot Framework passing for defined MVP features.
- CI/CD quality gates passing (compilation, strict linting, tests).

## Tech Stack

- **Languages**: Rust (latest stable), Python (latest stable).
- **Core Dependencies (Rust)**: `pyo3` (Python bindings), `calamine` (Excel parsing), `polars` / `arrow-rs` (Arrow integration).
- **Build System**: `maturin` to compile the extension module.
- **Testing**: `cargo test` for Rust internals, `robotframework` for acceptance tests.
- **Task Runner**: `just` (with `mise` for toolchain management).

## Commands

_These will be encapsulated in the `justfile`, but the raw commands are:_

- **Setup**: `mise install`
- **Build (Dev)**: `maturin develop`
- **Build (Release)**: `maturin build --release`
- **Test (Rust Unit Tests)**: `cargo test`
- **Test (Acceptance)**: `robot tests/`
- **Lint (Rust)**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Format (Rust)**: `cargo fmt -- --check`

## Project Structure

```text
├── src/                # Rust core engine and PyO3 bindings
├── python/
│   └── tabularix/      # Python wrappers and typing stubs
├── tests/              # Robot Framework acceptance tests
│   └── data/           # Sample .xlsx files for testing
├── docs/               # Documentation (specs, ADRs, pitches)
├── Cargo.toml          # Rust dependencies and configuration
├── pyproject.toml      # Python and Maturin configuration
├── justfile            # Task runner recipes
└── mise.toml           # Toolchain manager configuration
```

## Code Style

**Rust:** Idiomatic Rust using `cargo fmt` and `clippy` (pedantic/restriction groups as specified). Avoid `.unwrap()` and `.expect()` in library code; use robust error propagation (`Result`).

```rust
// Example of expected Rust style
pub fn extract_rows_between(start: usize, end: usize) -> Result<RowGroup, TabularixError> {
    if start >= end {
        return Err(TabularixError::InvalidBounds("start must be before end".into()));
    }
    // ... implementation
}
```

**Python:** PEP8 compliant, fully type-hinted, and ergonomic.

```python
# Example of expected Python style
def search_and_crop_before(self, marker: str, direction: str = "TOP") -> "Sheet":
    """Crops the sheet relative to a specified text marker."""
    self._rust_engine.search_and_crop_before(marker, direction)
    return self
```

## Testing Strategy

Following the **Black Box Development Methodology**:

- **Rust Unit Tests (`tests/`, `src/`)**: Written by the AI developer. Tests core algorithms (parsing, cropping, pattern matching logic) natively in Rust to ensure memory safety and correctness.
- **Acceptance Tests (`tests/*.robot`)**: Written by the Product Owner (User). Focuses purely on the Python API boundary using Keyword-Driven specifications. Acts as the ultimate definition of "done".
- **Coverage**: Core logic must have high unit test coverage before acceptance tests are run.

## Boundaries

- **Always do:**
    - Ensure memory safety and zero-copy conversions to Arrow where possible.
    - Return informative errors back across the FFI boundary (PyO3).
    - Run all quality gates locally before pushing code.
- **Ask first:**
    - Adding significant Rust dependencies (e.g., async runtimes if not needed).
    - Modifying the accepted Project Structure.
- **Never do:**
    - Hardcode bypasses to satisfy Robot Framework tests.
    - Add features outside the MVP scope (e.g., cell formatting matching, LLM agents).

## Open Questions

- None at this time.
