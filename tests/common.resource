*** Settings ***
Documentation       Shared keywords and helpers for acceptance tests.


*** Keywords ***
Load Simple Sheet
    [Documentation]    Loads the 'simple' worksheet from the test data sample workbook.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    RETURN    ${sheet}

Load Complex Sheet
    [Documentation]    Loads the 'complex' worksheet from the test data sample workbook.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("complex")
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
