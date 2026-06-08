---
title: Welcome to Tabularix
description: A high-performance Excel parser and evaluator built in Rust with Python bindings designed to solve the hidden data problem.
icon: lucide/table
---

# ⚙️ Welcome to Tabularix

**Tabularix** is a fast, reliable, and memory-efficient framework for parsing, cleaning, and extracting data from messy Excel spreadsheets. Built from the ground up in **Rust** with seamless **Python bindings**, it brings systems-level performance to your data pipelines while allowing you to script extraction logic in your favorite ecosystem.

---

## 🚨 The Hidden Data Problem

In most organizations, mission-critical data exists in a state of high entropy, trapped in fragmented Excel files. This "hidden data" is the primary barrier to meaningful AI adoption and advanced analytics.

Spreadsheets are raw, non-relational visual grids. In the real world, this results in extreme layout variability:

- **Inconsistent boundaries:** Tables lack fixed coordinates and must be found using visual anchors.
- **Variable headers:** Simple, multi-row, or missing headers are common.
- **Merged cells:** They obscure underlying data columns and complicate extraction.
- **Extraneous elements:** Subtotals, complex footers, and scattered metadata disrupt table continuity.
- **Multiple tables:** Independent sub-tables are often scattered across the same sheet.

Furthermore, these files frequently contain private, strategic, or sensitive company data. You cannot simply send raw Excel contents to external AI models or third-party extraction APIs without compromising data privacy.

---

## 💡 The Tabularix Solution

We built Tabularix to navigate this layout chaos natively and securely. It operates as a local, privacy-first library that empowers you to write **Smart Data Extraction** scripts in Python.

Our approach allows you to implement distinct data extraction strategies that can identify, clean, and organize hidden data into structured formats ready for downstream use.

!!! tip "Configuration as Code"
    By writing your extraction "recipes" in Python, you get all the benefits of modern software engineering: version control, linters, CI/CD, and local debugging. The heavy lifting is safely offloaded to Rust.

---

## 🌟 Key Features

- **Blazing Fast**: Leverages Rust's zero-cost abstractions for lightning-fast workbook parsing and manipulation.
- **Privacy-First**: Everything runs locally on your machine or within your private cloud. No data is sent to external APIs.
- **Python Native**: Scripting extraction logic in Python offers incredible ergonomics and fits perfectly into standard data engineering pipelines.
- **Interoperability**: (Planned) Direct exports of extracted tables to [Arrow](https://arrow.apache.org/), [Pandas](https://pandas.pydata.org/), and [Polars](https://pola.rs/).

## 📦 Getting Started

Ready to dive in? Head over to the [Quickstart Guide](quickstart.md) to install Tabularix and write your first script!
