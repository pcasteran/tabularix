---
title: Quickstart
description: Get up and running with Tabularix in minutes.
icon: lucide/rocket
---

# ⚡ Quickstart

Getting started with Tabularix is easy. Since the core logic is implemented in Rust, you get the performance of a systems language directly within your Python environment.

## Installation

Currently, Tabularix is in active development. You can build it from source using `uv` and `maturin`.

```bash
# Clone the repository
git clone https://github.com/pcasteran/tabularix.git
cd tabularix

# Create a virtual environment and build the extension
uv venv
uv run maturin develop
```

## First Steps

Once installed, you can import Tabularix just like any other Python module.

!!! example "Basic Usage"
    Here is a quick example of how to interact with the Tabularix core from Python.

```python
import tabularix

# Call the Rust-backed hello_world function
response = tabularix.hello_world()
print(response)
```

!!! success "Expected Output"
    ```text
    Hello from Tabularix Rust Core!
    ```

## Next Steps

Check out the [Project Specification](spec.md) to see our roadmap and architectural design!
