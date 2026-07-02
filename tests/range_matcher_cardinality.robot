*** Settings ***
Documentation       Acceptance tests for RangeMatcher cell and row cardinalities.

Library             Collections


*** Test Cases ***
Verify Cell Cardinality One Or More
    [Documentation]    Verify cell rule one_or_more (+) matches 1 or more cells.
    ${pattern}=    Evaluate
    ...    tabularix.RangePattern1D(tabularix.value("Data"), tabularix.non_empty().one_or_more())
    ...    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    # Matches with 1 non-empty cell
    ${res1}=    Evaluate    $matcher.matches_range([["Data", "A"]])
    Should Be True    ${res1}

    # Matches with 3 non-empty cells
    ${res2}=    Evaluate    $matcher.matches_range([["Data", "A", "B", "C"]])
    Should Be True    ${res2}

    # Does not match with 0 non-empty cells
    ${res3}=    Evaluate    $matcher.matches_range([["Data"]])
    Should Be Equal    ${res3}    ${False}

Verify Cell Cardinality Zero Or More
    [Documentation]    Verify cell rule zero_or_more (*) matches 0 or more cells.
    ${pattern}=    Evaluate
    ...    tabularix.RangePattern1D(tabularix.value("Data"), tabularix.empty().zero_or_more())
    ...    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    # Matches with 0 empty cells
    ${res1}=    Evaluate    $matcher.matches_range([["Data"]])
    Should Be True    ${res1}

    # Matches with 2 empty cells
    ${res2}=    Evaluate    $matcher.matches_range([["Data", None, None]])
    Should Be True    ${res2}

Verify Cell Cardinality Optional
    [Documentation]    Verify cell rule optional (?) matches 0 or 1 cell.
    ${pattern}=    Evaluate
    ...    tabularix.RangePattern1D(tabularix.value("Data"), tabularix.value("Extra").optional())
    ...    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    # Matches with 0 "Extra" cells
    ${res1}=    Evaluate    $matcher.matches_range([["Data"]])
    Should Be True    ${res1}

    # Matches with 1 "Extra" cell
    ${res2}=    Evaluate    $matcher.matches_range([["Data", "Extra"]])
    Should Be True    ${res2}

    # Does not match with 2 "Extra" cells
    ${res3}=    Evaluate    $matcher.matches_range([["Data", "Extra", "Extra"]])
    Should Be Equal    ${res3}    ${False}

Verify Cell Cardinality Repeat Custom Range Exact
    [Documentation]    Verify cell rule repeat matches exact custom count.
    ${pattern}=    Evaluate    tabularix.RangePattern1D(tabularix.non_empty().repeat(3))    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    ${res1}=    Evaluate    $matcher.matches_range([["A", "B", "C"]])
    Should Be True    ${res1}

    ${res2}=    Evaluate    $matcher.matches_range([["A", "B"]])
    Should Be Equal    ${res2}    ${False}

Verify Cell Cardinality Repeat Custom Range Interval
    [Documentation]    Verify cell rule repeat matches custom interval range.
    ${pattern2}=    Evaluate    tabularix.RangePattern1D(tabularix.empty().repeat(2, 4))    modules=tabularix
    ${matcher2}=    Evaluate    $pattern2.to_matcher(direction="LR")

    ${res3}=    Evaluate    $matcher2.matches_range([[None, None, None]])
    Should Be True    ${res3}

    ${res4}=    Evaluate    $matcher2.matches_range([[None]])
    Should Be Equal    ${res4}    ${False}

Verify Row Cardinality Repeating
    [Documentation]    Verify row repetitions (one_or_more, zero_or_more, optional) work correctly.
    ${pattern}=    Evaluate    tabularix.RangePattern1D(tabularix.non_empty().repeat(2))    modules=tabularix
    ${matcher}=    Evaluate
    ...    tabularix.RangePattern2D($pattern.one_or_more()).to_matcher(outer_direction="TB", inner_direction="LR")
    ...    modules=tabularix

    # Matches 1 row
    ${res1}=    Evaluate    $matcher.matches_range([["A", "B"]])
    Should Be True    ${res1}

    # Matches 3 rows
    ${res2}=    Evaluate    $matcher.matches_range([["A", "B"], ["C", "D"], ["E", "F"]])
    Should Be True    ${res2}

Verify Row Cardinality Optional
    [Documentation]    Verify row optional repetition works correctly.
    ${r1}=    Evaluate    tabularix.RangePattern1D(tabularix.value("Header"))    modules=tabularix
    ${r2}=    Evaluate    tabularix.RangePattern1D(tabularix.value("Data")).optional()    modules=tabularix
    ${r3}=    Evaluate    tabularix.RangePattern1D(tabularix.value("Footer"))    modules=tabularix
    ${matcher}=    Evaluate
    ...    tabularix.RangePattern2D($r1, $r2, $r3).to_matcher(outer_direction="TB", inner_direction="LR")
    ...    modules=tabularix

    # Matches 0 reps
    ${res1}=    Evaluate    $matcher.matches_range([["Header"], ["Footer"]])
    Should Be True    ${res1}

    # Matches 1 rep
    ${res2}=    Evaluate    $matcher.matches_range([["Header"], ["Data"], ["Footer"]])
    Should Be True    ${res2}

Verify Row Cardinality Zero Or More
    [Documentation]    Verify row zero_or_more repetition works correctly.
    ${r1}=    Evaluate    tabularix.RangePattern1D(tabularix.value("Header"))    modules=tabularix
    ${r2}=    Evaluate    tabularix.RangePattern1D(tabularix.value("Data")).zero_or_more()    modules=tabularix
    ${r3}=    Evaluate    tabularix.RangePattern1D(tabularix.value("Footer"))    modules=tabularix
    ${matcher}=    Evaluate
    ...    tabularix.RangePattern2D($r1, $r2, $r3).to_matcher(outer_direction="TB", inner_direction="LR")
    ...    modules=tabularix

    # Matches 0 reps
    ${res1}=    Evaluate    $matcher.matches_range([["Header"], ["Footer"]])
    Should Be True    ${res1}

    # Matches 2 reps
    ${res2}=    Evaluate    $matcher.matches_range([["Header"], ["Data"], ["Data"], ["Footer"]])
    Should Be True    ${res2}

Verify Row Cardinality Repeat Custom Range
    [Documentation]    Verify row custom repetition count works correctly.
    ${pattern}=    Evaluate    tabularix.RangePattern1D(tabularix.non_empty().repeat(2))    modules=tabularix
    ${matcher}=    Evaluate
    ...    tabularix.RangePattern2D($pattern.repeat(2, 3)).to_matcher(outer_direction="TB", inner_direction="LR")
    ...    modules=tabularix

    # Matches 2 rows
    ${res1}=    Evaluate    $matcher.matches_range([["A", "B"], ["C", "D"]])
    Should Be True    ${res1}

    # Does not match 1 row
    ${res2}=    Evaluate    $matcher.matches_range([["A", "B"]])
    Should Be Equal    ${res2}    ${False}
