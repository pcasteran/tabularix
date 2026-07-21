from __future__ import annotations

import re
from typing import Any

TOKEN_SPEC = [
    ("QUANTIFIER", r"\+\??|\*\??|\?\??|\{\d+,\d+\}\??|\{\d+,\}\??|\{\d+\}\??"),
    ("STRING", r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\''),
    ("LBRACKET", r"\["),
    ("RBRACKET", r"\]"),
    ("COLON", r":"),
    ("LPAREN", r"\("),
    ("RPAREN", r"\)"),
    ("IDENTIFIER", r"[a-zA-Z_][a-zA-Z0-9_]*"),
    ("WS", r"\s+"),
    ("MISMATCH", r"."),
]

TOKEN_REGEX = re.compile("|".join(f"(?P<{name}>{pattern})" for name, pattern in TOKEN_SPEC))


class Token:
    """Represents a lexical token produced by the Layex DSL lexer."""

    def __init__(self, type_: str, value: str, pos: int):
        """Initializes a new Token instance."""
        self.type = type_
        self.value = value
        self.pos = pos

    def __repr__(self) -> str:
        """Returns the string representation of the token."""
        return f"Token({self.type}, {self.value!r}, at {self.pos})"


class ParseError(ValueError):
    """Raised when parsing layout patterns encounters syntax or token errors."""

    def __init__(self, message: str, text: str, pos: int):
        """Initializes a ParseError with contextual position formatting."""
        self.message = message
        self.text = text
        self.pos = pos
        super().__init__(self._format_message())

    def _format_message(self) -> str:
        lines = self.text.splitlines()
        current_pos = 0
        line_num = 0
        col_num = 0
        for i, line in enumerate(lines):
            line_len = len(line) + 1  # +1 for newline character
            if current_pos <= self.pos < current_pos + line_len:
                line_num = i
                col_num = self.pos - current_pos
                break
            current_pos += line_len
        else:
            line_num = len(lines) - 1
            col_num = len(lines[-1]) if lines else 0

        line_str = lines[line_num] if line_num < len(lines) else ""
        pointer = " " * col_num + "^"
        return f"Parse error at line {line_num + 1}, column {col_num + 1}: {self.message}\n  {line_str}\n  {pointer}"


def lex(text: str) -> list[Token]:
    """Tokenizes a Layex DSL pattern string into a sequence of tokens."""
    tokens = []
    for mo in TOKEN_REGEX.finditer(text):
        kind = mo.lastgroup
        if kind is None:
            continue
        value = mo.group(kind)
        pos = mo.start()
        if kind == "WS":
            continue
        elif kind == "MISMATCH":
            raise ParseError(f"Unexpected character {value!r}", text, pos)
        else:
            tokens.append(Token(kind, value, pos))
    return tokens


def parse_quantifier_value(q_str: str, text: str = "", pos: int = 0) -> tuple[int, int | None, bool]:
    """Parses a quantifier string like '+', '+?', '{1,4}', '{1,4}?' and returns (min, max, greedy)."""
    greedy = True
    raw_q_str = q_str
    if q_str.endswith("?"):
        if q_str in ("+?", "*?", "??") or q_str.endswith("}?"):
            greedy = False
            q_str = q_str[:-1]

    if q_str == "+":
        return 1, None, greedy
    elif q_str == "*":
        return 0, None, greedy
    elif q_str == "?":
        return 0, 1, greedy
    elif q_str.startswith("{") and q_str.endswith("}"):
        inner = q_str[1:-1]
        if "," in inner:
            parts = inner.split(",")
            min_val = int(parts[0].strip())
            max_val_str = parts[1].strip()
            max_val = int(max_val_str) if max_val_str else None
            if not greedy and min_val == max_val:
                raise ParseError(f"Exact count repetition '{raw_q_str}' cannot be lazy (?)", text, pos)
            return min_val, max_val, greedy
        else:
            if not greedy:
                raise ParseError(f"Exact count repetition '{raw_q_str}' cannot be lazy (?)", text, pos)
            val = int(inner.strip())
            return val, val, greedy
    raise ValueError(f"Invalid quantifier string: {q_str}")


def _get_types() -> tuple[Any, Any, Any]:
    import tabularix

    return tabularix.CellRule, tabularix.RangePattern1D, tabularix.RangePattern2D


class Parser:
    """Recursive-descent parser for Layex DSL strings."""

    def __init__(self, text: str):
        """Initializes a new Parser instance for the given text."""
        self.text = text
        self.tokens = lex(text)
        self.pos = 0

    def peek(self) -> Token | None:
        """Returns the current token without consuming it."""
        if self.pos < len(self.tokens):
            return self.tokens[self.pos]
        return None

    def consume(self, expected_type: str | None = None) -> Token:
        """Consumes and returns the current token, optionally asserting its type."""
        tok = self.peek()
        if tok is None:
            raise ParseError("Unexpected end of input", self.text, len(self.text))
        if expected_type and tok.type != expected_type:
            raise ParseError(f"Expected token of type {expected_type}, got {tok.type}", self.text, tok.pos)
        self.pos += 1
        return tok

    def parse_pattern_2d(self) -> Any:
        """Parses a 2D pattern sequence of parenthesized rows."""
        _, _, RangePattern2D_cls = _get_types()
        rows = []
        rows.append(self.parse_parenthesized_row())
        while (tok := self.peek()) and tok.type == "LPAREN":
            rows.append(self.parse_parenthesized_row())

        last_tok = self.peek()
        if last_tok is not None:
            raise ParseError(f"Unexpected token {last_tok.value!r} after 2D pattern", self.text, last_tok.pos)

        return RangePattern2D_cls(*rows)

    def parse_parenthesized_row(self) -> Any:
        """Parses a row pattern wrapped in mandatory parentheses."""
        tok = self.peek()
        if tok is None or tok.type != "LPAREN":
            pos = tok.pos if tok else len(self.text)
            raise ParseError(
                "Each row in a 2D pattern must be wrapped in parentheses (e.g. '( [v: \"Header\"] )')", self.text, pos
            )
        self.consume("LPAREN")
        pattern_1d = self.parse_pattern_1d()
        self.consume("RPAREN")

        # Check for optional quantifier
        next_tok = self.peek()
        if next_tok and next_tok.type == "QUANTIFIER":
            q_tok = self.consume("QUANTIFIER")
            min_val, max_val, greedy = parse_quantifier_value(q_tok.value, self.text, q_tok.pos)
            pattern_1d.repeat(min_val, max_val, greedy=greedy)

        return pattern_1d

    def parse_pattern_1d(self) -> Any:
        """Parses a 1D sequence of elements."""
        _, RangePattern1D_cls, _ = _get_types()
        elements = []
        elements.append(self.parse_element())
        while (tok := self.peek()) and tok.type in ("LBRACKET", "LPAREN"):
            elements.append(self.parse_element())
        return RangePattern1D_cls(*elements)

    def parse_element(self) -> Any:
        """Parses a single element (cell rule or nested parenthesized group) with optional quantifier."""
        tok = self.peek()
        if tok is None:
            raise ParseError("Expected cell rule or parenthesized group", self.text, len(self.text))

        if tok.type == "LPAREN":
            self.consume("LPAREN")
            element = self.parse_pattern_1d()
            self.consume("RPAREN")
        elif tok.type == "LBRACKET":
            element = self.parse_cell_rule()
        else:
            raise ParseError(f"Unexpected token {tok.value!r}. Expected '[' or '('", self.text, tok.pos)

        # Check for optional quantifier
        next_tok = self.peek()
        if next_tok and next_tok.type == "QUANTIFIER":
            q_tok = self.consume("QUANTIFIER")
            min_val, max_val, greedy = parse_quantifier_value(q_tok.value, self.text, q_tok.pos)
            element.repeat(min_val, max_val, greedy=greedy)

        return element

    def parse_cell_rule(self) -> Any:
        """Parses a single bracketed cell rule [...] definition."""
        CellRule_cls, _, _ = _get_types()
        self.consume("LBRACKET")

        tok = self.peek()
        if tok is None:
            raise ParseError("Expected cell rule body inside brackets", self.text, len(self.text))

        if tok.type != "IDENTIFIER":
            raise ParseError(
                f"Expected rule identifier (e.g. 'v', 'r', 'e', 'ne', 'a'), got {tok.value!r}", self.text, tok.pos
            )

        ident = self.consume("IDENTIFIER").value
        next_tok = self.peek()
        if next_tok and next_tok.type == "COLON":
            self.consume("COLON")
            val_tok = self.consume("STRING")
            raw_val = val_tok.value
            quote_char = raw_val[0]
            inner_val = raw_val[1:-1]
            if quote_char == '"':
                val_str = inner_val.replace(r"\"", '"').replace(r"\\", "\\")
            else:
                val_str = inner_val.replace(r"\'", "'").replace(r"\\", "\\")

            if ident in ("value", "v"):
                rule = CellRule_cls("exact", val_str)
            elif ident in ("regex", "r"):
                rule = CellRule_cls("regex", val_str)
            else:
                raise ParseError(
                    f"Unknown attribute-value rule type {ident!r}. Expected 'v'/'value' or 'r'/'regex'.",
                    self.text,
                    tok.pos,
                )
        else:
            if ident in ("empty", "e"):
                rule = CellRule_cls("empty")
            elif ident in ("non_empty", "ne"):
                rule = CellRule_cls("non_empty")
            elif ident in ("any", "a"):
                rule = CellRule_cls("any")
            else:
                raise ParseError(
                    f"Unknown bare state rule type {ident!r}. Expected 'e'/'empty', 'ne'/'non_empty', or 'a'/'any'.",
                    self.text,
                    tok.pos,
                )

        self.consume("RBRACKET")
        return rule


def parse_pattern_1d(pattern_str: str) -> Any:
    """Parses a 1D pattern shorthand DSL string into a RangePattern1D instance.

    Args:
        pattern_str: The shorthand Layex DSL string (e.g. '[v: "Category"], [e]?').

    Returns:
        A compiled RangePattern1D instance.

    Raises:
        ParseError: If syntax or token errors are encountered during parsing.
    """
    parser = Parser(pattern_str)
    res = parser.parse_pattern_1d()
    last_tok = parser.peek()
    if last_tok is not None:
        raise ParseError(f"Unexpected token {last_tok.value!r} at end of 1D pattern", pattern_str, last_tok.pos)
    return res


def parse_pattern_2d(pattern_str: str) -> Any:
    """Parses a 2D pattern shorthand DSL string into a RangePattern2D instance.

    Args:
        pattern_str: The shorthand Layex DSL string (e.g. '([v: "Header"]) ; ([ne])+').

    Returns:
        A compiled RangePattern2D instance.

    Raises:
        ParseError: If syntax or token errors are encountered during parsing.
    """
    parser = Parser(pattern_str)
    return parser.parse_pattern_2d()
