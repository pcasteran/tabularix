---
title: Layout Expressions (DSL)
description: Reference guide to the Tabularix shorthand string expression language (Layex DSL).
icon: lucide/code
---

# ✍️ Layout Expressions (DSL)

To simplify the definition of 1D and 2D layout patterns for non-technical users and configuration-driven pipelines, **Tabularix** provides a shorthand string expression DSL.

This DSL allows you to declare cell matchers and repeat rules in a compact, visual format resembling grid cells, compile them via helper functions, and serialized them round-trip.

---

## 🏗️ Syntax Overview

The core building block of the DSL is a **cell rule**, represented by square brackets `[...]` which visually mimic a spreadsheet cell box.

Within a cell box, you specify the matching criteria using standard attribute-value pairs or bare keywords:

### 1. Cell Rules matching

| Matcher Type       | Full Form             | Short Form        | Example                             |
| :----------------- | :-------------------- | :---------------- | :---------------------------------- |
| **Literal Text**   | `[value: "Region"]`   | `[v: "Region"]`   | Matches exact cell value `"Region"` |
| **Regex Pattern**  | `[regex: "^Q[1-4]$"]` | `[r: "^Q[1-4]$"]` | Matches cell value with a regex     |
| **Empty Cell**     | `[empty]`             | `[e]`             | Matches blank/empty cell            |
| **Non-Empty Cell** | `[non_empty]`         | `[ne]`            | Matches any cell containing data    |
| **Wildcard / Any** | `[any]`               | `[a]`             | Matches any cell (wildcard)         |

---

## 🔄 Repetitions & Quantifiers

You can control how many times a cell rule or group repeats by appending standard regular-expression quantifiers directly after the closing bracket `]` or closing parenthesis `)`:

| Quantifier   | Matching Mode  | Description                                           | Example             |
| :----------- | :------------- | :---------------------------------------------------- | :------------------ |
| `+`          | Greedy         | Matches 1 or more times                               | `[ne]+`             |
| `+?`         | Lazy           | Matches 1 or more times (smallest span)               | `[ne]+?`            |
| `*`          | Greedy         | Matches 0 or more times                               | `[a]*`              |
| `*?`         | Lazy           | Matches 0 or more times (smallest span)               | `[a]*?`             |
| `?`          | Greedy         | Matches 0 or 1 time (optional)                        | `[e]?`              |
| `??`         | Lazy           | Matches 0 or 1 time (smallest span)                   | `[e]??`             |
| `{n}`        | Exact          | Matches exactly _n_ times                             | `[r: "^\$\\d+"]{4}` |
| `{min,max}`  | Greedy Range   | Matches between `min` and `max` times                 | `[v: "X"]{2,5}`     |
| `{min,max}?` | Lazy Range     | Matches between `min` and `max` times (smallest span) | `[v: "X"]{2,5}?`    |
| `{min,}`     | Unbounded      | Matches at least `min` times                          | `[v: "X"]{2,}`      |
| `{min,}?`    | Lazy Unbounded | Matches at least `min` times (smallest span)          | `[v: "X"]{2,}?`     |

---

## 📦 Nested Groups (Recursive Nesting)

For complex recurring sequences of cells, you can group multiple rules together using parentheses `(...)` and apply quantifiers to the entire group:

- **Syntax**: `rule1 (rule2 rule3){min,max}`
- **Example**: `[v: "Category"] ([r: "^Q[1-4]$"] [e]?){4}`
    - This matches an exact cell `"Category"` followed by 4 sequences of a quarter regex cell and an optional empty cell.

---

## 🏁 2D Grid Patterns

A 2D layout pattern (`RangePattern2D`) is defined in a single string as a sequence of parenthesized row patterns.

!!! important "Parenthesized Row Requirement"

    To ensure clarity and avoid syntax ambiguity (such as whether a quantifier belongs to the last cell rule or the entire row), **every row in a 2D pattern must be wrapped in parentheses**.

- **Syntax**: `(row_pattern1) quantifier? (row_pattern2) quantifier?`
- **Example**:
    ```text
    ([v: "Region"] [r: "^Q[1-4]$"]{4}) ([r: "^(?!Total).*$"] [ne]{4})+
    ```
    - **Row 1**: A header row containing the exact string `"Region"` followed by 4 quarters.
    - **Row 2**: One or more data rows where the first cell does not start with `"Total"` followed by 4 non-empty cells.

Newlines (`\n`, `\r`) outside of quoted strings are treated as whitespace and ignored, allowing you to format multi-row layout patterns over multiple lines inside configuration blocks:

```yaml
pattern: |
    ([v: "Region"] [r: "^Q[1-4]$"]{4})
    ([r: "^(?!Total).*$"] [ne]{4})+
```

---

## 🐍 Python API Integration

Tabularix exports two simple parser functions to compile shorthand strings into wrapper objects:

```python
from tabularix import parse_pattern_1d, parse_pattern_2d

# Compile a 1D row pattern
header_pattern = parse_pattern_1d('[v: "Category"] [e]?')

# Compile a 2D grid pattern
data_pattern = parse_pattern_2d('([v: "Region"]) ([ne])+')
```

These parsed patterns work natively with the high-level table extraction functions:

```python
from tabularix import extract_table_with_header_and_data, load_workbook

wb = load_workbook("data.xlsx")
sheet = wb.get_sheet("Sheet1")

header_pattern = parse_pattern_1d('[v: "Region"] [r: "^Q[1-4]$"]{4}')
data_pattern = parse_pattern_2d('([r: "^(?!Total).*$"] [ne]{4})+')

table = extract_table_with_header_and_data(
    sheet,
    header_pattern=header_pattern,
    data_pattern=data_pattern
)
```

---

## 🔄 Round-Trip String Conversion

Every Tabularix pattern object supports standard Python stringification protocols, allowing you to convert them to and from the DSL losslessly.

### `str()` (DSL Shorthand)

Calling `str(pattern)` produces the compact short-form DSL string, which is perfect for saving to config files or logging:

```python
pattern = group(value("Category"), empty().optional())
print(str(pattern))
# Output: [v: "Category"] [e]?
```

### `repr()` (Python Constructor Code)

Calling `repr(pattern)` returns a valid, copy-pasteable Python constructor expression for debugging:

```python
pattern = parse_pattern_1d('[v: "Category"] [e]?')
print(repr(pattern))
# Output: RangePattern1D(value('Category'), empty().optional(greedy=True))
```
