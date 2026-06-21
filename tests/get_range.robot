*** Settings ***
Documentation       Acceptance tests for Sheet.get_range_between.

Library             Collections
Resource            common.resource


*** Test Cases ***
Verify Range Between Vertically
    [Documentation]    Test computing range between two vertically separated ranges.
    ${sheet}=    Load Simple Sheet
    ${header}=    Find Range By Value    ${sheet}    Header #1
    ${footer}=    Find Range By Value    ${sheet}    Merged value
    ${res}=    Evaluate    $sheet.get_range_between($header, $footer)
    Verify Range Coordinates    ${res}    1    2    0    2

Verify Range Between Horizontally
    [Documentation]    Test computing range between two horizontally separated ranges.
    ${sheet}=    Load Simple Sheet
    ${left}=    Evaluate    tabularix.Range.from_a1("A2:A4")    modules=tabularix
    ${right}=    Evaluate    tabularix.Range.from_a1("C2:C4")    modules=tabularix
    ${res}=    Evaluate    $sheet.get_range_between($left, $right)
    Verify Range Coordinates    ${res}    1    3    1    1

Verify Range Between Errors
    [Documentation]    Test that invalid separation orientations and alignments raise ValueError.
    ${sheet}=    Load Simple Sheet
    ${r1}=    Evaluate    tabularix.Range.from_a1("A1:C1")    modules=tabularix
    ${r2_mismatch}=    Evaluate    tabularix.Range.from_a1("A5:B5")    modules=tabularix
    # Vertical mismatch
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.get_range_between($r1, $r2_mismatch)
    # Diagonal separation
    ${diag1}=    Evaluate    tabularix.Range.from_a1("A1")    modules=tabularix
    ${diag2}=    Evaluate    tabularix.Range.from_a1("C3")    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.get_range_between($diag1, $diag2)
    # Overlap
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.get_range_between($r1, $r1)
