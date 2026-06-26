*** Settings ***
Documentation       Acceptance tests for loading Excel workbooks and sheets.

Library             Collections


*** Test Cases ***
Verify Workbook Sheet Names
    [Documentation]    Verify sheet names in the workbook match expectations.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${names}=    Evaluate    $wb.sheet_names()
    Sort List    ${names}
    VAR    @{expected_names}=    complex    multi-tables    simple
    Lists Should Be Equal    ${names}    ${expected_names}

Verify Active Sheet Metadata
    [Documentation]    Verify retrieving active sheet and checking its name and shape.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${name}=    Evaluate    $sheet.name
    Should Be Equal As Strings    ${name}    simple
    ${shape}=    Evaluate    $sheet.shape
    ${expected_shape}=    Evaluate    (5, 3)
    Should Be Equal    ${shape}    ${expected_shape}

Verify Sheet Cell Values Row 1 to 3
    [Documentation]    Verify checking cell values in first three rows.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${val1}=    Evaluate    $sheet.get_cell_value(0, 0)
    Should Be Equal As Strings    ${val1}    Header #1
    ${val2}=    Evaluate    $sheet.get_cell_value(1, 1)
    ${expected_val2}=    Evaluate    123.45
    Should Be Equal    ${val2}    ${expected_val2}
    ${val3}=    Evaluate    $sheet.get_cell_value(1, 2)
    Should Be Equal As Strings    ${val3}    Alice

Verify Sheet Cell Values Row 4 to 5
    [Documentation]    Verify checking cell values in rows four and five.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${val4}=    Evaluate    $sheet.get_cell_value(3, 2)
    Should Be Equal As Strings    ${val4}    Charlie
    ${val5}=    Evaluate    $sheet.get_cell_value(4, 2)
    Should Be Equal As Strings    ${val5}    David
