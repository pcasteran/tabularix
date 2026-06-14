*** Settings ***
Documentation       Acceptance tests for get_cell_value and set_cell_value APIs.

Library             Collections


*** Test Cases ***
Verify get_cell_value Success
    [Documentation]    Verify get_cell_value retrieves correct cell values.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${val1}=    Evaluate    $sheet.get_cell_value(0, 0)
    Should Be Equal As Strings    ${val1}    Header1
    ${val2}=    Evaluate    $sheet.get_cell_value(1, 1)
    ${expected_val2}=    Evaluate    123.45
    Should Be Equal    ${val2}    ${expected_val2}

Verify get_cell_value Out Of Bounds
    [Documentation]    Verify get_cell_value out of bounds (including negative coordinates) raises IndexError.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.get_cell_value(5, 0)
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.get_cell_value(-1, 0)
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.get_cell_value(0, -1)

Verify set_cell_value Success
    [Documentation]    Verify set_cell_value mutates the sheet cell in-place.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    # Before update
    ${val_before}=    Evaluate    $sheet.get_cell_value(1, 0)
    Should Be Equal As Strings    ${val_before}    ABC
    # Update value
    Evaluate    $sheet.set_cell_value(1, 0, "UpdatedValue")
    # After update
    ${val_after}=    Evaluate    $sheet.get_cell_value(1, 0)
    Should Be Equal As Strings    ${val_after}    UpdatedValue

Verify set_cell_value Out Of Bounds
    [Documentation]    Verify set_cell_value out of bounds (including negative coordinates) raises IndexError.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.set_cell_value(5, 0, "ErrorVal")
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.set_cell_value(-1, 0, "ErrorVal")
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.set_cell_value(0, -1, "ErrorVal")

Verify set_cell_value Type Validation
    [Documentation]    Verify set_cell_value only accepts strings and raises TypeError for other types.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    Run Keyword And Expect Error    *TypeError:*    Evaluate    $sheet.set_cell_value(0, 0, 123)
    Run Keyword And Expect Error    *TypeError:*    Evaluate    $sheet.set_cell_value(0, 0, True)
