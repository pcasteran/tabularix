*** Settings ***
Documentation       Acceptance tests for Sheet.get_range_between.

Library             Collections


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
    ${left}=    Evaluate    tabularix.Range(1, 3, 0, 0)    modules=tabularix
    ${right}=    Evaluate    tabularix.Range(1, 3, 2, 2)    modules=tabularix
    ${res}=    Evaluate    $sheet.get_range_between($left, $right)
    Verify Range Coordinates    ${res}    1    3    1    1

Verify Range Between Errors
    [Documentation]    Test that invalid separation orientations and alignments raise ValueError.
    ${sheet}=    Load Simple Sheet
    ${r1}=    Evaluate    tabularix.Range(0, 0, 0, 2)    modules=tabularix
    ${r2_mismatch}=    Evaluate    tabularix.Range(4, 4, 0, 1)    modules=tabularix
    # Vertical mismatch
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.get_range_between($r1, $r2_mismatch)
    # Diagonal separation
    ${diag1}=    Evaluate    tabularix.Range(0, 0, 0, 0)    modules=tabularix
    ${diag2}=    Evaluate    tabularix.Range(2, 2, 2, 2)    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.get_range_between($diag1, $diag2)
    # Overlap
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.get_range_between($r1, $r1)


*** Keywords ***
Load Simple Sheet
    [Documentation]    Loads the 'simple' worksheet from the test data sample workbook.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    RETURN    ${sheet}

Verify Range Coordinates
    [Documentation]    Verifies that the given Range matches the expected start/end row/col coordinates.
    [Arguments]    ${range}    ${s_row}    ${e_row}    ${s_col}    ${e_col}
    ${start_row}=    Evaluate    $range.start_row
    ${end_row}=    Evaluate    $range.end_row
    ${start_col}=    Evaluate    $range.start_col
    ${end_col}=    Evaluate    $range.end_col
    Should Be Equal As Integers    ${start_row}    ${s_row}
    Should Be Equal As Integers    ${end_row}    ${e_row}
    Should Be Equal As Integers    ${start_col}    ${s_col}
    Should Be Equal As Integers    ${end_col}    ${e_col}

Find Range By Value
    [Documentation]    Searches for a row pattern starting with the cell value.
    [Arguments]    ${sheet}    ${val}
    ${p}=    Evaluate    tabularix.value($val).any().any()    modules=tabularix
    ${m}=    Evaluate    tabularix.RangeMatcher().row($p)    modules=tabularix
    ${range}=    Evaluate    $sheet.search_range($m)
    RETURN    ${range}
