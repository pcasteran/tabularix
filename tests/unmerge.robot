*** Settings ***
Documentation       Acceptance tests for unmerge_cells API on worksheets.

Library             Collections


*** Test Cases ***
Verify Default Unmerge Initial State
    [Documentation]    Verify merged region initial cell values before unmerging.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${val_3_0}=    Evaluate    $sheet.get_cell_value(3, 0)
    Should Be Equal As Strings    ${val_3_0}    Merged value
    Verify Cell Is None    ${sheet}    3    1
    Verify Cell Is None    ${sheet}    4    0
    Verify Cell Is None    ${sheet}    4    1

Verify Default Unmerge Execution
    [Documentation]    Verify unmerge_cells with default parameters fills all merged cells.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    Evaluate    $sheet.unmerge_cells()
    Verify All Region Cells Equal    ${sheet}    Merged value

Verify Unmerge Non Overlapping Target Range
    [Documentation]    Verify non-overlapping target_ranges string leaves region untouched.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    Evaluate    $sheet.unmerge_cells(target_ranges="A1:C2")
    Verify Cell Is None    ${sheet}    3    1

Verify Unmerge Target Ranges List
    [Documentation]    Verify target_ranges list unmerges intersecting region.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    Evaluate    $sheet.unmerge_cells(target_ranges=["X1:Y2", "A4:A5"])
    Verify All Region Cells Equal    ${sheet}    Merged value

Verify Unmerge Range Objects List
    [Documentation]    Verify target_ranges using Range instances.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${r1}=    Evaluate    tabularix.Range(0, 1, 0, 1)    modules=tabularix
    ${r2}=    Evaluate    tabularix.Range(3, 4, 0, 1)    modules=tabularix
    Evaluate    $sheet.unmerge_cells(target_ranges=[$r1, $r2])
    ${val_4_1}=    Evaluate    $sheet.get_cell_value(4, 1)
    Should Be Equal As Strings    ${val_4_1}    Merged value

Verify Unmerge Fill Direction Bottom
    [Documentation]    Verify fill_direction="bottom" behavior.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    Evaluate    $sheet.unmerge_cells(fill_direction="bottom")
    ${val_3_0}=    Evaluate    $sheet.get_cell_value(3, 0)
    Should Be Equal As Strings    ${val_3_0}    Merged value
    ${val_4_0}=    Evaluate    $sheet.get_cell_value(4, 0)
    Should Be Equal As Strings    ${val_4_0}    Merged value
    Verify Cell Is None    ${sheet}    3    1
    Verify Cell Is None    ${sheet}    4    1

Verify Unmerge Fill Direction Right
    [Documentation]    Verify fill_direction="right" behavior.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    Evaluate    $sheet.unmerge_cells(fill_direction="right")
    ${val_3_0}=    Evaluate    $sheet.get_cell_value(3, 0)
    Should Be Equal As Strings    ${val_3_0}    Merged value
    ${val_3_1}=    Evaluate    $sheet.get_cell_value(3, 1)
    Should Be Equal As Strings    ${val_3_1}    Merged value
    Verify Cell Is None    ${sheet}    4    0
    Verify Cell Is None    ${sheet}    4    1

Verify Unmerge Cells Error Validation
    [Documentation]    Verify exception handling for invalid parameters.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()

    # Invalid fill_direction
    Run Keyword And Expect Error    *ValueError:*    Evaluate    $sheet.unmerge_cells(fill_direction="invalid_dir")

    # Invalid A1 notation in single string
    Run Keyword And Expect Error    *ValueError:*    Evaluate    $sheet.unmerge_cells(target_ranges="INVALID_A1")

    # Invalid A1 notation inside list
    Run Keyword And Expect Error    *ValueError:*    Evaluate    $sheet.unmerge_cells(target_ranges=["INVALID_A1"])

    # Invalid target_ranges element type inside list
    Run Keyword And Expect Error    *TypeError:*    Evaluate    $sheet.unmerge_cells(target_ranges=[123])

    # Invalid target_ranges top-level type
    Run Keyword And Expect Error    *TypeError:*    Evaluate    $sheet.unmerge_cells(target_ranges=123)


*** Keywords ***
Verify Cell Is None
    [Documentation]    Helper keyword to verify a cell at (row, col) returns None.
    [Arguments]    ${sheet}    ${row}    ${col}
    ${val}=    Evaluate    $sheet.get_cell_value(${row}, ${col})
    Should Be Equal    ${val}    ${None}

Verify All Region Cells Equal
    [Documentation]    Helper keyword to verify all 4 cells in A4:B5 hold expected_val.
    [Arguments]    ${sheet}    ${expected_val}
    ${val_3_0}=    Evaluate    $sheet.get_cell_value(3, 0)
    Should Be Equal As Strings    ${val_3_0}    ${expected_val}
    ${val_3_1}=    Evaluate    $sheet.get_cell_value(3, 1)
    Should Be Equal As Strings    ${val_3_1}    ${expected_val}
    ${val_4_0}=    Evaluate    $sheet.get_cell_value(4, 0)
    Should Be Equal As Strings    ${val_4_0}    ${expected_val}
    ${val_4_1}=    Evaluate    $sheet.get_cell_value(4, 1)
    Should Be Equal As Strings    ${val_4_1}    ${expected_val}
