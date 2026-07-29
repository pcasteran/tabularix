*** Settings ***
Documentation       Acceptance tests for search_and_drop API on worksheets.

Library             Collections


*** Test Cases ***
Verify Search And Drop Top
    [Documentation]    Verify exact string match with drop direction top.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${coords}=    Evaluate    $sheet.search_and_drop("DEF", "top")
    ${expected_coords}=    Evaluate    ((2, 0), (0, 0))
    ${expected_shape}=    Evaluate    (3, 3)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    0    0    DEF

Verify Search And Drop Bottom
    [Documentation]    Verify exact string match with drop direction bottom.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${coords}=    Evaluate    $sheet.search_and_drop("DEF", "bottom")
    ${expected_coords}=    Evaluate    ((2, 0), (2, 0))
    ${expected_shape}=    Evaluate    (3, 3)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    2    0    DEF

Verify Search And Drop Left
    [Documentation]    Verify exact string match with drop direction left.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${coords}=    Evaluate    $sheet.search_and_drop("Header #2", "left")
    ${expected_coords}=    Evaluate    ((0, 1), (0, 0))
    ${expected_shape}=    Evaluate    (5, 2)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    0    0    Header #2

Verify Search And Drop Right
    [Documentation]    Verify exact string match with drop direction right.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${coords}=    Evaluate    $sheet.search_and_drop("Header #2", "right")
    ${expected_coords}=    Evaluate    ((0, 1), (0, 1))
    ${expected_shape}=    Evaluate    (5, 2)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    0    1    Header #2

Verify Search And Drop Top Left
    [Documentation]    Verify exact string match with drop direction top_left.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${coords}=    Evaluate    $sheet.search_and_drop("Alice", "top_left")
    ${expected_coords}=    Evaluate    ((1, 2), (0, 0))
    ${expected_shape}=    Evaluate    (4, 1)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    0    0    Alice

Verify Regex Search And Drop
    [Documentation]    Verify using a compiled Python regex pattern.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${pattern}=    Evaluate    re.compile("^[D-F]{3}$")    modules=re
    ${coords}=    Evaluate    $sheet.search_and_drop($pattern, "top")
    ${expected_coords}=    Evaluate    ((2, 0), (0, 0))
    ${expected_shape}=    Evaluate    (3, 3)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    0    0    DEF

Verify Search And Drop Top Right
    [Documentation]    Verify exact string match with drop direction top_right.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${coords}=    Evaluate    $sheet.search_and_drop("DEF", "top_right")
    ${expected_coords}=    Evaluate    ((2, 0), (0, 0))
    ${expected_shape}=    Evaluate    (3, 1)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    0    0    DEF

Verify Search And Drop Bottom Left
    [Documentation]    Verify exact string match with drop direction bottom_left.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${coords}=    Evaluate    $sheet.search_and_drop("Header #2", "bottom_left")
    ${expected_coords}=    Evaluate    ((0, 1), (0, 0))
    ${expected_shape}=    Evaluate    (1, 2)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    0    0    Header #2

Verify Search And Drop Bottom Right
    [Documentation]    Verify exact string match with drop direction bottom_right.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${coords}=    Evaluate    $sheet.search_and_drop("Header #2", "bottom_right")
    ${expected_coords}=    Evaluate    ((0, 1), (0, 1))
    ${expected_shape}=    Evaluate    (1, 2)
    Verify Coordinates And Shape    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Verify Cell Value    ${sheet}    0    1    Header #2

Verify Search And Drop Errors
    [Documentation]    Verify correct exception raising for invalid inputs/not found.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    # Not found -> ValueError
    Run Keyword And Expect Error    *Search term not found*    Evaluate    $sheet.search_and_drop("MISSING", "top")
    # Invalid direction -> ValueError
    Run Keyword And Expect Error
    ...    *Invalid drop direction*
    ...    Evaluate
    ...    $sheet.search_and_drop("DEF", "invalid_dir")
    # Invalid query type -> TypeError
    Run Keyword And Expect Error    *str_or_regex must be*    Evaluate    $sheet.search_and_drop(123.45, "top")


*** Keywords ***
Verify Coordinates And Shape
    [Documentation]    Verify the returned drop coordinates and the updated sheet shape.
    [Arguments]    ${coords}    ${expected_coords}    ${sheet}    ${expected_shape}
    Should Be Equal    ${coords}    ${expected_coords}
    ${shape}=    Evaluate    $sheet.shape
    Should Be Equal    ${shape}    ${expected_shape}

Verify Cell Value
    [Documentation]    Verify the sheet cell value at a given (row, col) coordinate matches expected.
    [Arguments]    ${sheet}    ${row}    ${col}    ${expected_val}
    ${val}=    Evaluate    $sheet.get_cell_value(${row}, ${col})
    Should Be Equal As Strings    ${val}    ${expected_val}
