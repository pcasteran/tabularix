*** Settings ***
Documentation       Acceptance tests for loading Excel workbooks and sheets.

Library             Collections


*** Test Cases ***
Verify Workbook Sheet Names
    [Documentation]    Verify sheet names in the workbook match expectations.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${names}=    Evaluate    $wb.sheet_names()
    Sort List    ${names}
    VAR    @{expected_names}=    complex    simple
    Lists Should Be Equal    ${names}    ${expected_names}

Verify Active Sheet Metadata
    [Documentation]    Verify retrieving active sheet and checking its name and shape.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${name}=    Evaluate    $sheet.name
    Should Be Equal As Strings    ${name}    simple
    ${shape}=    Evaluate    $sheet.shape
    ${expected_shape}=    Evaluate    (5, 2)
    Should Be Equal    ${shape}    ${expected_shape}

Verify Sheet Cell Values
    [Documentation]    Verify checking individual cell values.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${val1}=    Evaluate    $sheet.get_cell_value(0, 0)
    Should Be Equal As Strings    ${val1}    Header1
    ${val2}=    Evaluate    $sheet.get_cell_value(1, 1)
    ${expected_val2}=    Evaluate    123.45
    Should Be Equal    ${val2}    ${expected_val2}
