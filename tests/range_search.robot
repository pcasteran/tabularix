*** Settings ***
Documentation       Acceptance tests for Sheet.search_range and Sheet.search_range_relative.

Library             Collections


*** Test Cases ***
Verify Absolute Range Search Basic
    [Documentation]    Test Sheet.search_range with default coordinates.
    ${sheet}=    Load Simple Sheet
    ${hdr_p}=    Evaluate    tabularix.non_empty().any().any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RangeMatcher().row($hdr_p)    modules=tabularix

    ${range}=    Evaluate    $sheet.search_range($matcher)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    0    0    0    2

    ${repr}=    Evaluate    repr($range)
    Should Be Equal    ${repr}    <Range rows=0..0, cols=0..2>

Verify Absolute Range Search Sliced
    [Documentation]    Test Sheet.search_range within a sliced window.
    ${sheet}=    Load Simple Sheet
    ${hdr_p}=    Evaluate    tabularix.non_empty().any().any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RangeMatcher().row($hdr_p)    modules=tabularix

    ${range_sliced}=    Evaluate    $sheet.search_range($matcher, start_row=1)
    Should Not Be Equal    ${range_sliced}    ${None}
    Verify Range Coordinates    ${range_sliced}    1    1    0    2

Verify Absolute Range Search Bounds and Errors
    [Documentation]    Test negative and out of bounds errors in search_range.
    ${sheet}=    Load Simple Sheet
    ${p}=    Evaluate    tabularix.any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RangeMatcher().row($p)    modules=tabularix

    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_range($matcher, start_row=-1)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_range($matcher, end_row=100)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_range($matcher, start_col=-5)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_range($matcher, end_col=10)

Verify Absolute Range Search Inverted Bounds
    [Documentation]    Test inverted range errors in search_range.
    ${sheet}=    Load Simple Sheet
    ${p}=    Evaluate    tabularix.any()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RangeMatcher().row($p)    modules=tabularix

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range($matcher, start_row=3, end_row=1)
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range($matcher, start_col=2, end_col=1)

Verify Relative Range Search Below
    [Documentation]    Test search_range_relative below a matched range.
    ${sheet}=    Load Simple Sheet
    ${header_range}=    Find Range By Value    ${sheet}    Header #1

    ${abc_p}=    Evaluate    tabularix.value("ABC").any().any()    modules=tabularix
    ${abc_m}=    Evaluate    tabularix.RangeMatcher().row($abc_p)    modules=tabularix
    ${abc_range}=    Evaluate    $sheet.search_range_relative($abc_m, below=$header_range)
    Should Not Be Equal    ${abc_range}    ${None}
    Verify Range Coordinates    ${abc_range}    1    1    0    2

Verify Relative Range Search Below and Above
    [Documentation]    Test search_range_relative bounded vertically by below and above.
    ${sheet}=    Load Simple Sheet
    ${header_range}=    Find Range By Value    ${sheet}    Header #1
    ${def_range}=    Find Range By Value    ${sheet}    DEF

    ${abc_p}=    Evaluate    tabularix.value("ABC").any().any()    modules=tabularix
    ${abc_m}=    Evaluate    tabularix.RangeMatcher().row($abc_p)    modules=tabularix
    ${abc_range_both}=    Evaluate    $sheet.search_range_relative($abc_m, below=$header_range, above=$def_range)
    Should Not Be Equal    ${abc_range_both}    ${None}
    Verify Range Coordinates    ${abc_range_both}    1    1    0    2

Verify Relative Range Search Vertical Conflicts
    [Documentation]    Test vertical boundary conflicts.
    ${sheet}=    Load Simple Sheet
    ${header_range}=    Find Range By Value    ${sheet}    Header #1
    ${def_range}=    Find Range By Value    ${sheet}    DEF

    ${abc_p}=    Evaluate    tabularix.value("ABC").any().any()    modules=tabularix
    ${abc_m}=    Evaluate    tabularix.RangeMatcher().row($abc_p)    modules=tabularix

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range_relative($abc_m, below=$def_range, above=$header_range)

    ${rg1}=    Evaluate    tabularix.Range(1, 1, 0, 2)    modules=tabularix
    ${rg2}=    Evaluate    tabularix.Range(3, 3, 0, 1)    modules=tabularix
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range_relative($abc_m, below=$rg1, above=$rg2)

Verify Relative Range Search Horizontal Bounds
    [Documentation]    Test search_range_relative with horizontal boundaries (left, right).
    ${sheet}=    Load Simple Sheet
    ${right_rg}=    Evaluate    tabularix.Range(1, 2, 0, 0)    modules=tabularix
    ${left_rg}=    Evaluate    tabularix.Range(1, 2, 2, 2)    modules=tabularix

    ${val_p}=    Evaluate    tabularix.non_empty()    modules=tabularix
    ${val_m}=    Evaluate    tabularix.RangeMatcher().row($val_p)    modules=tabularix

    ${range}=    Evaluate    $sheet.search_range_relative($val_m, right=$right_rg, left=$left_rg)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    1    1    1    1

Verify Relative Range Search Horizontal Conflicts
    [Documentation]    Test horizontal boundary conflicts.
    ${sheet}=    Load Simple Sheet
    ${right_rg}=    Evaluate    tabularix.Range(1, 2, 0, 0)    modules=tabularix
    ${left_rg}=    Evaluate    tabularix.Range(1, 2, 2, 2)    modules=tabularix

    ${val_p}=    Evaluate    tabularix.non_empty()    modules=tabularix
    ${val_m}=    Evaluate    tabularix.RangeMatcher().row($val_p)    modules=tabularix

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range_relative($val_m, right=$left_rg, left=$right_rg)

    ${rg_row1}=    Evaluate    tabularix.Range(1, 1, 0, 0)    modules=tabularix
    ${rg_row2}=    Evaluate    tabularix.Range(2, 2, 2, 2)    modules=tabularix
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range_relative($val_m, right=$rg_row1, left=$rg_row2)


*** Keywords ***
Load Simple Sheet
    [Documentation]    Loads the 'simple' worksheet from the test data sample workbook.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")
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
