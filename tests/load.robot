*** Settings ***
Documentation       Acceptance tests for loading Excel workbooks and sheets.

Library             Collections


*** Test Cases ***
Verify Workbook Sheet Names
    [Documentation]    Verify sheet names in the workbook match expectations.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${names}=    Evaluate    $wb.sheet_names()
    Sort List    ${names}
    VAR    @{expected_names}=    complex    multi-tables    simple    sub-sections
    Lists Should Be Equal    ${names}    ${expected_names}

Verify Active Sheet Name Metadata
    [Documentation]    Verify retrieving active sheet name.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${name}=    Evaluate    $wb.active_sheet_name()
    Should Be Equal As Strings    ${name}    simple
    ${sheet}=    Evaluate    $wb.get_sheet($name)
    ${shape}=    Evaluate    $sheet.shape
    ${expected_shape}=    Evaluate    (5, 3)
    Should Be Equal    ${shape}    ${expected_shape}

Verify Sheet Cell Values Row 1 to 3
    [Documentation]    Verify checking cell values in first three rows.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
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
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${val4}=    Evaluate    $sheet.get_cell_value(3, 2)
    Should Be Equal As Strings    ${val4}    Charlie
    ${val5}=    Evaluate    $sheet.get_cell_value(4, 2)
    Should Be Equal As Strings    ${val5}    David

Verify Sheet Unload Single
    [Documentation]    Verify is_sheet_loaded and unload_sheet on a single worksheet.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    Should Not Be True    $wb.is_sheet_loaded("simple")
    Evaluate    $wb.get_sheet("simple")
    Should Be True    $wb.is_sheet_loaded("simple")
    ${evicted1}=    Evaluate    $wb.unload_sheet("simple")
    Should Be True    $evicted1
    Should Not Be True    $wb.is_sheet_loaded("simple")
    ${evicted2}=    Evaluate    $wb.unload_sheet("simple")
    Should Not Be True    $evicted2

Verify Sheet Open Context Manager
    [Documentation]    Verify open_sheet context manager enter and exit lifecycle.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${cm}=    Evaluate    $wb.open_sheet("simple")
    Evaluate    $cm.__enter__()
    ${loaded_in_cm}=    Evaluate    $wb.is_sheet_loaded("simple")
    Should Be Equal    ${loaded_in_cm}    ${True}
    Evaluate    $cm.__exit__(None, None, None)
    ${loaded_after_cm}=    Evaluate    $wb.is_sheet_loaded("simple")
    Should Be Equal    ${loaded_after_cm}    ${False}

Verify Sheet Unload All Sheets
    [Documentation]    Verify unload_all_sheets evicts all loaded sheets.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    Evaluate    $wb.get_sheet("simple")
    Evaluate    $wb.get_sheet("complex")
    ${count}=    Evaluate    $wb.unload_all_sheets()
    Should Be Equal As Integers    ${count}    2
    ${loaded_final}=    Evaluate    $wb.is_sheet_loaded("simple")
    Should Be Equal    ${loaded_final}    ${False}
