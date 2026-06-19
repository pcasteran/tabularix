*** Settings ***
Documentation       Acceptance tests for RowGroupMatcher cell and row cardinalities.

Library             Collections


*** Test Cases ***
Verify Cell Cardinality One Or More
    [Documentation]    Verify cell rule one_or_more (+) matches 1 or more cells.
    ${pattern}=    Evaluate    tabularix.value("Data").non_empty().one_or_more()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($pattern)    modules=tabularix

    # Matches with 1 non-empty cell
    ${res1}=    Evaluate    $matcher.matches_row_group([["Data", "A"]])
    Should Be True    ${res1}

    # Matches with 3 non-empty cells
    ${res2}=    Evaluate    $matcher.matches_row_group([["Data", "A", "B", "C"]])
    Should Be True    ${res2}

    # Does not match with 0 non-empty cells
    ${res3}=    Evaluate    $matcher.matches_row_group([["Data"]])
    Should Be Equal    ${res3}    ${False}

Verify Cell Cardinality Zero Or More
    [Documentation]    Verify cell rule zero_or_more (*) matches 0 or more cells.
    ${pattern}=    Evaluate    tabularix.value("Data").empty().zero_or_more()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($pattern)    modules=tabularix

    # Matches with 0 empty cells
    ${res1}=    Evaluate    $matcher.matches_row_group([["Data"]])
    Should Be True    ${res1}

    # Matches with 2 empty cells
    ${res2}=    Evaluate    $matcher.matches_row_group([["Data", None, None]])
    Should Be True    ${res2}

Verify Cell Cardinality Optional
    [Documentation]    Verify cell rule optional (?) matches 0 or 1 cell.
    ${pattern}=    Evaluate    tabularix.value("Data").value("Extra").optional()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($pattern)    modules=tabularix

    # Matches with 0 "Extra" cells
    ${res1}=    Evaluate    $matcher.matches_row_group([["Data"]])
    Should Be True    ${res1}

    # Matches with 1 "Extra" cell
    ${res2}=    Evaluate    $matcher.matches_row_group([["Data", "Extra"]])
    Should Be True    ${res2}

    # Does not match with 2 "Extra" cells
    ${res3}=    Evaluate    $matcher.matches_row_group([["Data", "Extra", "Extra"]])
    Should Be Equal    ${res3}    ${False}

Verify Cell Cardinality Repeat Custom Range Exact
    [Documentation]    Verify cell rule repeat matches exact custom count.
    ${pattern}=    Evaluate    tabularix.non_empty().repeat(3)    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($pattern)    modules=tabularix

    ${res1}=    Evaluate    $matcher.matches_row_group([["A", "B", "C"]])
    Should Be True    ${res1}

    ${res2}=    Evaluate    $matcher.matches_row_group([["A", "B"]])
    Should Be Equal    ${res2}    ${False}

Verify Cell Cardinality Repeat Custom Range Interval
    [Documentation]    Verify cell rule repeat matches custom interval range.
    ${pattern2}=    Evaluate    tabularix.empty().repeat(2, 4)    modules=tabularix
    ${matcher2}=    Evaluate    tabularix.RowGroupMatcher().row($pattern2)    modules=tabularix

    ${res3}=    Evaluate    $matcher2.matches_row_group([[None, None, None]])
    Should Be True    ${res3}

    ${res4}=    Evaluate    $matcher2.matches_row_group([[None]])
    Should Be Equal    ${res4}    ${False}

Verify Row Cardinality Repeating
    [Documentation]    Verify row repetitions (one_or_more, zero_or_more, optional) work correctly.
    ${pattern}=    Evaluate    tabularix.non_empty().repeat(2)    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($pattern).one_or_more()    modules=tabularix

    # Matches 1 row
    ${res1}=    Evaluate    $matcher.matches_row_group([["A", "B"]])
    Should Be True    ${res1}

    # Matches 3 rows
    ${res2}=    Evaluate    $matcher.matches_row_group([["A", "B"], ["C", "D"], ["E", "F"]])
    Should Be True    ${res2}
