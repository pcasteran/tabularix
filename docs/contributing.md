---
title: Contributing
description: Guidelines for contributing and setting up the development environment.
icon: lucide/git-pull-request
---

# 🤝 Contributing & Development Guidelines

Thank you for your interest in contributing to **Tabularix**! This document provides instructions for setting up your local environment, running static analysis checks, executing test suites, and preparing packages for release.

---

## 🛠️ Environment Setup

Tabularix is a hybrid library with a high-performance **Rust core** and **Python bindings**.

### 1. Using VS Code Dev Containers (Recommended)

The easiest and recommended way to get started is by using [VS Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers). The project includes a fully configured `.devcontainer` directory containing all necessary compilers, Python versions, formatters, and linters pre-installed.

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/pcasteran/tabularix.git
    cd tabularix
    ```
2.  **Open in VS Code**: Open the folder in VS Code, and click the **"Reopen in Container"** prompt.

### 2. Manual Environment Setup (Alternative)

If you prefer to configure your environment manually, local toolchains are managed using [mise](https://mise.jdx.dev/) and recipes are run using [just](https://github.com/casey/just).

1.  **Install Global Prerequisites**: Make sure you have `mise` and `just` installed globally:
    - [Mise Installation Guide](https://mise.jdx.dev/getting-started.html)
    - [Just Installation Guide](https://github.com/casey/just#installation)
2.  **Clone the Repository**:

    ```bash
    git clone https://github.com/pcasteran/tabularix.git
    cd tabularix
    ```

3.  **Set Up Toolchains**: Install all correct compilers, runtimes, and linters defined in `mise.toml`:

    ```bash
    mise install
    ```

4.  **Build the Project**: Compile the Rust engine extension and build the Python wheel:
    ```bash
    just build
    ```

---

## 🔍 Code Style & Static Analysis

We enforce strict formatting and quality checks across all files. Always run the static analysis suite before submitting code:

```bash
# Execute all pre-commit hooks, formatting, and linters
just prek

# Update the static analysis engine and hook versions
just prek-hooks-update
```

The `just prek` command validates:

- Rust formatting (`cargo fmt`) and linting (`cargo clippy`).
- Python linting/formatting (`ruff`).
- JSON, YAML, and TOML validation.
- Markdown and spellchecking.

---

## 🧪 Testing Suites

All contributions must pass the entire test suite before merging.

```bash
# Run Rust unit tests
just unit-test

# Run Robot Framework acceptance tests
just acceptance-test
```

---

## 📖 Building Documentation

The documentation site is powered by **Zensical**.

```bash
# Run a local development documentation server
just docs-serve

# Build the static HTML site (output to site/ directory)
just docs-build
```

---

## 📦 Release Preparation

If you are preparing a new release, use the automated recipe to branch, update the changelog, and bump versions:

```bash
# Calculate next version, branch, update CHANGELOG.md, and bump versions
just prepare-release
```
