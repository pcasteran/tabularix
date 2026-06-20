*** Settings ***
Documentation       Acceptance tests for Sheet.search_row_group and Sheet.search_row_group_relative.

Library             Collections


*** Test Cases ***
Verify Absolute Row Group Search Basic
    [Documentation]    Test Sheet.search_row_group with default coordinates.
    ${sheet}=    Load Simple Sheet
    ${hdr_p}=    Evaluate    tabularix.non_empty().any().any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($hdr_p)    modules=tabularix

    ${group}=    Evaluate    $sheet.search_row_group($matcher)
    Should Not Be Equal    ${group}    ${None}
    Verify Group Coordinates    ${group}    0    0    0    2

    ${repr}=    Evaluate    repr($group)
    Should Be Equal    ${repr}    <RowGroup rows=0..0, cols=0..2>

Verify Absolute Row Group Search Sliced
    [Documentation]    Test Sheet.search_row_group within a sliced window.
    ${sheet}=    Load Simple Sheet
    ${hdr_p}=    Evaluate    tabularix.non_empty().any().any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($hdr_p)    modules=tabularix

    ${group_sliced}=    Evaluate    $sheet.search_row_group($matcher, start_row=1)
    Should Not Be Equal    ${group_sliced}    ${None}
    Verify Group Coordinates    ${group_sliced}    1    1    0    2

Verify Absolute Row Group Search Bounds and Errors
    [Documentation]    Test negative and out of bounds errors in search_row_group.
    ${sheet}=    Load Simple Sheet
    ${p}=    Evaluate    tabularix.any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($p)    modules=tabularix

    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_row_group($matcher, start_row=-1)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_row_group($matcher, end_row=100)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_row_group($matcher, start_col=-5)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_row_group($matcher, end_col=10)

Verify Absolute Row Group Search Inverted Bounds
    [Documentation]    Test inverted range errors in search_row_group.
    ${sheet}=    Load Simple Sheet
    ${p}=    Evaluate    tabularix.any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($p)    modules=tabularix

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_row_group($matcher, start_row=3, end_row=1)
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_row_group($matcher, start_col=2, end_col=1)

Verify Relative Row Group Search Below
    [Documentation]    Test search_row_group_relative below a matched group.
    ${sheet}=    Load Simple Sheet
    ${header_group}=    Find Group By Value    ${sheet}    Header #1

    ${abc_p}=    Evaluate    tabularix.value("ABC").any().any()    modules=tabularix
    ${abc_m}=    Evaluate    tabularix.RowGroupMatcher().row($abc_p)    modules=tabularix
    ${abc_group}=    Evaluate    $sheet.search_row_group_relative($abc_m, below=$header_group)
    Should Not Be Equal    ${abc_group}    ${None}
    Verify Group Coordinates    ${abc_group}    1    1    0    2

Verify Relative Row Group Search Below and Above
    [Documentation]    Test search_row_group_relative bounded vertically by below and above.
    ${sheet}=    Load Simple Sheet
    ${header_group}=    Find Group By Value    ${sheet}    Header #1
    ${def_group}=    Find Group By Value    ${sheet}    DEF

    ${abc_p}=    Evaluate    tabularix.value("ABC").any().any()    modules=tabularix
    ${abc_m}=    Evaluate    tabularix.RowGroupMatcher().row($abc_p)    modules=tabularix
    ${abc_group_both}=    Evaluate    $sheet.search_row_group_relative($abc_m, below=$header_group, above=$def_group)
    Should Not Be Equal    ${abc_group_both}    ${None}
    Verify Group Coordinates    ${abc_group_both}    1    1    0    2

Verify Relative Row Group Search Vertical Conflicts
    [Documentation]    Test vertical boundary conflicts.
    ${sheet}=    Load Simple Sheet
    ${header_group}=    Find Group By Value    ${sheet}    Header #1
    ${def_group}=    Find Group By Value    ${sheet}    DEF

    ${abc_p}=    Evaluate    tabularix.value("ABC").any().any()    modules=tabularix
    ${abc_m}=    Evaluate    tabularix.RowGroupMatcher().row($abc_p)    modules=tabularix

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_row_group_relative($abc_m, below=$def_group, above=$header_group)

    ${rg1}=    Evaluate    tabularix.RowGroup(1, 1, 0, 2)    modules=tabularix
    ${rg2}=    Evaluate    tabularix.RowGroup(3, 3, 0, 1)    modules=tabularix
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_row_group_relative($abc_m, below=$rg1, above=$rg2)

Verify Relative Row Group Search Horizontal Bounds
    [Documentation]    Test search_row_group_relative with horizontal boundaries (left, right).
    ${sheet}=    Load Simple Sheet
    ${right_rg}=    Evaluate    tabularix.RowGroup(1, 2, 0, 0)    modules=tabularix
    ${left_rg}=    Evaluate    tabularix.RowGroup(1, 2, 2, 2)    modules=tabularix

    ${val_p}=    Evaluate    tabularix.non_empty()    modules=tabularix
    ${val_m}=    Evaluate    tabularix.RowGroupMatcher().row($val_p)    modules=tabularix

    ${group}=    Evaluate    $sheet.search_row_group_relative($val_m, right=$right_rg, left=$left_rg)
    Should Not Be Equal    ${group}    ${None}
    Verify Group Coordinates    ${group}    1    1    1    1

Verify Relative Row Group Search Horizontal Conflicts
    [Documentation]    Test horizontal boundary conflicts.
    ${sheet}=    Load Simple Sheet
    ${right_rg}=    Evaluate    tabularix.RowGroup(1, 2, 0, 0)    modules=tabularix
    ${left_rg}=    Evaluate    tabularix.RowGroup(1, 2, 2, 2)    modules=tabularix

    ${val_p}=    Evaluate    tabularix.non_empty()    modules=tabularix
    ${val_m}=    Evaluate    tabularix.RowGroupMatcher().row($val_p)    modules=tabularix

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_row_group_relative($val_m, right=$left_rg, left=$right_rg)

    ${rg_row1}=    Evaluate    tabularix.RowGroup(1, 1, 0, 0)    modules=tabularix
    ${rg_row2}=    Evaluate    tabularix.RowGroup(2, 2, 2, 2)    modules=tabularix
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_row_group_relative($val_m, right=$rg_row1, left=$rg_row2)


*** Keywords ***
Load Simple Sheet
    [Documentation]    Loads the 'simple' worksheet from the test data sample workbook.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
    RETURN    ${sheet}

Verify Group Coordinates
    [Documentation]    Verifies that the given RowGroup matches the expected start/end row/col coordinates.
    [Arguments]    ${group}    ${s_row}    ${e_row}    ${s_col}    ${e_col}
    ${start_row}=    Evaluate    $group.start_row
    ${end_row}=    Evaluate    $group.end_row
    ${start_col}=    Evaluate    $group.start_col
    ${end_col}=    Evaluate    $group.end_col
    Should Be Equal As Integers    ${start_row}    ${s_row}
    Should Be Equal As Integers    ${end_row}    ${e_row}
    Should Be Equal As Integers    ${start_col}    ${s_col}
    Should Be Equal As Integers    ${end_col}    ${e_col}

Find Group By Value
    [Documentation]    Searches for a row pattern starting with the cell value.
    [Arguments]    ${sheet}    ${val}
    ${p}=    Evaluate    tabularix.value($val).any().any()    modules=tabularix
    ${m}=    Evaluate    tabularix.RowGroupMatcher().row($p)    modules=tabularix
    ${group}=    Evaluate    $sheet.search_row_group($m)
    RETURN    ${group}
