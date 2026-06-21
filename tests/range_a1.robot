*** Settings ***
Documentation       Acceptance tests for Range.from_a1 builder.

Library             Collections
Resource            common.resource


*** Test Cases ***
Verify Standard A1 Range Construction
    [Documentation]    Test creating a Range from standard A1 notation.
    ${range}=    Evaluate    tabularix.Range.from_a1("B2:D6")    modules=tabularix
    Verify Range Coordinates    ${range}    1    5    1    3

Verify Single Cell Range Construction
    [Documentation]    Test creating a Range from a single cell string.
    ${range}=    Evaluate    tabularix.Range.from_a1("B2")    modules=tabularix
    Verify Range Coordinates    ${range}    1    1    1    1

Verify Normalization of Reverse Ranges
    [Documentation]    Test that reverse coordinates in A1 are correctly swapped/normalized.
    ${range}=    Evaluate    tabularix.Range.from_a1("D6:B2")    modules=tabularix
    Verify Range Coordinates    ${range}    1    5    1    3

Verify Range Parsing Errors
    [Documentation]    Test that invalid or unbounded formats correctly raise ValueError.
    # Unbounded columns
    Run Keyword And Expect Error    *ValueError*    Evaluate    tabularix.Range.from_a1("A:B")    modules=tabularix
    # Unbounded rows
    Run Keyword And Expect Error    *ValueError*    Evaluate    tabularix.Range.from_a1("1:2")    modules=tabularix
    # Partial unbounded
    Run Keyword And Expect Error    *ValueError*    Evaluate    tabularix.Range.from_a1("A1:B")    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    tabularix.Range.from_a1("A:B2")    modules=tabularix
    # Empty parts or invalid syntax
    Run Keyword And Expect Error    *ValueError*    Evaluate    tabularix.Range.from_a1("")    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    tabularix.Range.from_a1("A0")    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    tabularix.Range.from_a1("A1:B2:C3")    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    tabularix.Range.from_a1("A 1")    modules=tabularix
