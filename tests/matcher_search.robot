*** Settings ***
Documentation       Acceptance tests for Sheet.search_row_group and Sheet.search_row_group_relative.

Library             Collections


*** Test Cases ***
Verify Absolute Row Group Search
    [Documentation]    Test Sheet.search_row_group with absolute coordinates.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    # Set up matcher for Header row: non-empty cell followed by any cells
    ${hdr_p}=    Evaluate    tabularix.non_empty().any().any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($hdr_p)    modules=tabularix

    # Search with defaults
    ${group}=    Evaluate    $sheet.search_row_group($matcher)
    Should Not Be Equal    ${group}    ${None}
    ${start_row}=    Evaluate    $group.start_row
    ${end_row}=    Evaluate    $group.end_row
    ${start_col}=    Evaluate    $group.start_col
    ${end_col}=    Evaluate    $group.end_col
    Should Be Equal As Integers    ${start_row}    0
    Should Be Equal As Integers    ${end_row}      0
    Should Be Equal As Integers    ${start_col}    0
    Should Be Equal As Integers    ${end_col}      2

    # String representation check
    ${repr}=    Evaluate    repr($group)
    Should Be Equal    ${repr}    <RowGroup rows=0..0, cols=0..2>

    # Search within a sliced window that excludes row 0
    ${group_sliced}=    Evaluate    $sheet.search_row_group($matcher, start_row=1)
    # Row 1 is: ABC, 123.45, Alice. Row 1 matches non_empty().any().any(). So it should return row 1.
    Should Not Be Equal    ${group_sliced}    ${None}
    ${start_row_s}=    Evaluate    $group_sliced.start_row
    Should Be Equal As Integers    ${start_row_s}    1

Verify Absolute Row Group Search Bounds and Errors
    [Documentation]    Test boundaries and expected errors in search_row_group.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    ${p}=    Evaluate    tabularix.any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($p)    modules=tabularix

    # Negative bounds error (IndexError)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_row_group($matcher, start_row=-1)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_row_group($matcher, end_row=100)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_row_group($matcher, start_col=-5)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_row_group($matcher, end_col=10)

    # Inverted ranges error (ValueError)
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.search_row_group($matcher, start_row=3, end_row=1)
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.search_row_group($matcher, start_col=2, end_col=1)

Verify Relative Row Group Search Vertical
    [Documentation]    Test search_row_group_relative with vertical boundaries (below, above).
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    # Match row 0: Header
    ${hdr_p}=    Evaluate    tabularix.value("Header #1").any().any()    modules=tabularix
    ${hdr_m}=    Evaluate    tabularix.RowGroupMatcher().row($hdr_p)    modules=tabularix
    ${header_group}=    Evaluate    $sheet.search_row_group($hdr_m)

    # Match row 2: DEF data row
    ${def_p}=    Evaluate    tabularix.value("DEF").any().any()    modules=tabularix
    ${def_m}=    Evaluate    tabularix.RowGroupMatcher().row($def_p)    modules=tabularix
    ${def_group}=    Evaluate    $sheet.search_row_group($def_m)

    # Now search for "ABC" below header_group
    ${abc_p}=    Evaluate    tabularix.value("ABC").any().any()    modules=tabularix
    ${abc_m}=    Evaluate    tabularix.RowGroupMatcher().row($abc_p)    modules=tabularix
    ${abc_group}=    Evaluate    $sheet.search_row_group_relative($abc_m, below=$header_group)
    Should Not Be Equal    ${abc_group}    ${None}
    ${abc_start}=    Evaluate    $abc_group.start_row
    Should Be Equal As Integers    ${abc_start}    1

    # Search for "ABC" below header_group and above def_group
    ${abc_group_both}=    Evaluate    $sheet.search_row_group_relative($abc_m, below=$header_group, above=$def_group)
    Should Not Be Equal    ${abc_group_both}    ${None}
    ${abc_start_b}=    Evaluate    $abc_group_both.start_row
    Should Be Equal As Integers    ${abc_start_b}    1

    # Conflict: above is below below
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.search_row_group_relative($abc_m, below=$def_group, above=$header_group)

    # Conflict: non-aligned column spans
    # Create custom RowGroup objects with different column spans
    ${rg1}=    Evaluate    tabularix.RowGroup(1, 1, 0, 2)    modules=tabularix
    ${rg2}=    Evaluate    tabularix.RowGroup(3, 3, 0, 1)    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.search_row_group_relative($abc_m, below=$rg1, above=$rg2)

Verify Relative Row Group Search Horizontal
    [Documentation]    Test search_row_group_relative with horizontal boundaries (left, right).
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    # Let's create dummy left/right groups to test the logic
    # Right boundary at col 0, left boundary at col 2
    # So searching in between (col 1)
    ${right_rg}=    Evaluate    tabularix.RowGroup(1, 2, 0, 0)    modules=tabularix
    ${left_rg}=     Evaluate    tabularix.RowGroup(1, 2, 2, 2)    modules=tabularix

    # Row 1 and 2 in col 1 are: 123.45, 678.0.
    # Pattern expecting 1 cell
    ${val_p}=    Evaluate    tabularix.non_empty()    modules=tabularix
    ${val_m}=    Evaluate    tabularix.RowGroupMatcher().row($val_p)    modules=tabularix

    ${group}=    Evaluate    $sheet.search_row_group_relative($val_m, right=$right_rg, left=$left_rg)
    Should Not Be Equal    ${group}    ${None}
    ${s_col}=    Evaluate    $group.start_col
    ${e_col}=    Evaluate    $group.end_col
    Should Be Equal As Integers    ${s_col}    1
    Should Be Equal As Integers    ${e_col}    1

    # Conflict: right is to the left of left (overlapping columns)
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.search_row_group_relative($val_m, right=$left_rg, left=$right_rg)

    # Conflict: non-aligned row spans
    ${rg_row1}=    Evaluate    tabularix.RowGroup(1, 1, 0, 0)    modules=tabularix
    ${rg_row2}=    Evaluate    tabularix.RowGroup(2, 2, 2, 2)    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.search_row_group_relative($val_m, right=$rg_row1, left=$rg_row2)
