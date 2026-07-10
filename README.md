<div align="center">
  <img src="docs/assets/logo_2.png" alt="Tabularix Logo" width="250"/>
  <p><strong>Smart, layout-resilient data extraction from Excel documents.</strong></p>
</div>

---

# Tabularix

Tabularix is a high-performance framework designed to identify, extract, and organize "hidden data" trapped in fragmented, messy Excel files. It transforms highly variable visual spreadsheets into clean, structured formats (such as Apache Arrow tables, Pandas DataFrames, or Polars DataFrames) at native speeds.

Developed as a direct continuation of the [Archery](https://github.com/RomualdRousseau/Archery) framework, it combines a blazing-fast **Rust core engine** with ergonomic **Python scripting bindings** to enable powerful "Configuration as Code" recipes.

---

## 🚀 Installation

Install the stable package directly from PyPI:

```bash
pip install tabularix
```

---

## 💡 Quick Example

### The Scenario

Suppose we have a spreadsheet containing a sales report table surrounded by empty rows, headers, and metadata, as shown in the layout analysis below:

[<img src="docs/assets/sheet_complex.svg" alt="Multi-tables Worksheet Layout" width="600" />](docs/assets/sheet_complex.svg)

Instead of hardcoding static cell coordinates (e.g., `A3:E8`) which break when columns or rows are added, deleted, or shifted, Tabularix uses **Range Matchers** to dynamically locate table boundaries relative to their visual markers.

The following example shows how to define the header and data patterns, locate the table, and export it:

```python
import polars as pl
from tabularix import (
    extract_table_with_header_and_data,
    grid,
    group,
    load_workbook,
    non_empty,
    regex,
    value,
)

# 1. Load the workbook and get the target worksheet.
workbook = load_workbook("tests/data/sample.xlsx")
sheet = workbook.get_sheet("complex")

# 2. Define the header row pattern (starts with "Region", then 4 Quarters matching Q1-Q4 regex).
header_pattern = group(
    value("Region"),
    regex(r"^Q[1-4]$").repeat(min=4, max=4)
)

# 3. Define the data row pattern (region name, then 4 non-empty numeric quarter cells).
data_pattern = grid(
    group(
        regex(r"^(?!Total).*$"),  # Match any string except "Total" (the footer marker).
        non_empty().repeat(min=4, max=4)
    ).one_or_more()
)

# 4. Extract the structured Table with dynamic coordinate scanning.
table = extract_table_with_header_and_data(
    sheet,
    header_pattern,
    data_pattern,
    clean_names=True
)

# 5. Export zero-copy to a Polars or Pandas DataFrame.
df = pl.from_arrow(table.to_arrow())
print(df)
```

---

## 📖 Documentation

For full guides, detailed tutorials, and API reference, please visit our **[Official Documentation Site](https://pcasteran.github.io/tabularix)**.

---

## ⚡ Core Features

- **High-Performance Rust Core**: Performs CPU-heavy Excel manipulation, boundary scanning, and cell matching at native speeds.
- **Privacy-First & Secure**: Runs entirely locally on your hardware. No external APIs or third-party servers are queried.
- **Python Ergonomics**: Natural integration with standard Python tools, dynamic typing support, and full PEP 8 compliance.
- **Zero-Copy FFI**: Seamless exports to Apache Arrow tables, Pandas, Polars, and DuckDB.

---

## 🤝 Contributing

Contributions are welcome! Please read our **[Development Guidelines](docs/contributing.md)** (or the root [CONTRIBUTING.md](CONTRIBUTING.md)) for details on local environment setup, testing, formatting checks, and repository workflows.

---

## ⚖️ License

Tabularix is dual-licensed under the Apache 2.0 and MIT licenses.

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Tabularix by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any additional terms or conditions.
