*** Settings ***
Documentation       Acceptance tests for drop_row and drop_column APIs.

Library             Collections


*** Test Cases ***
Verify drop_row Shape Update
    [Documentation]    Verify drop_row updates the shape of the sheet.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    ${shape_before}=    Evaluate    $sheet.shape
    ${expected_before}=    Evaluate    (5, 3)
    Should Be Equal    ${shape_before}    ${expected_before}

    Evaluate    $sheet.drop_row(1)

    ${shape_after}=    Evaluate    $sheet.shape
    ${expected_after}=    Evaluate    (4, 3)
    Should Be Equal    ${shape_after}    ${expected_after}

Verify drop_row Values Shifting
    [Documentation]    Verify drop_row shifts remaining cell values.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    ${val_before}=    Evaluate    $sheet.get_cell_value(1, 0)
    Should Be Equal As Strings    ${val_before}    ABC

    Evaluate    $sheet.drop_row(1)

    ${val_after}=    Evaluate    $sheet.get_cell_value(1, 0)
    Should Be Equal As Strings    ${val_after}    DEF

Verify drop_column Shape Update
    [Documentation]    Verify drop_column updates the shape of the sheet.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    ${shape_before}=    Evaluate    $sheet.shape
    ${expected_before}=    Evaluate    (5, 3)
    Should Be Equal    ${shape_before}    ${expected_before}

    Evaluate    $sheet.drop_column(1)

    ${shape_after}=    Evaluate    $sheet.shape
    ${expected_after}=    Evaluate    (5, 2)
    Should Be Equal    ${shape_after}    ${expected_after}

Verify drop_column Values Shifting
    [Documentation]    Verify drop_column shifts remaining cell values and removes out of bounds columns.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    ${val_before}=    Evaluate    $sheet.get_cell_value(0, 1)
    Should Be Equal As Strings    ${val_before}    Header #2

    Evaluate    $sheet.drop_column(1)

    ${val_after}=    Evaluate    $sheet.get_cell_value(0, 1)
    Should Be Equal As Strings    ${val_after}    Header #3
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.get_cell_value(0, 2)

Verify drop Out Of Bounds
    [Documentation]    Verify drop_row and drop_column with out of bounds index raise IndexError.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.drop_row(5)
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.drop_column(3)

Verify drop Negative Index
    [Documentation]    Verify drop_row and drop_column with negative index raise IndexError.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.drop_row(-1)
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.drop_column(-1)

Verify drop Empty Sheet
    [Documentation]    Verify dropping all rows makes the sheet empty, and dropping on empty sheet raises IndexError.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    Repeat Keyword    5 times    Evaluate    $sheet.drop_row(0)

    ${shape}=    Evaluate    $sheet.shape
    ${expected_shape}=    Evaluate    (0, 0)
    Should Be Equal    ${shape}    ${expected_shape}

    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.drop_row(0)
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.drop_column(0)
