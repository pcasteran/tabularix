*** Settings ***
Documentation       Acceptance tests for RangeMatcher scanning and matching directions (1D and 2D).

Library             Collections


*** Test Cases ***
Verify 1D Match Direction LR
    [Documentation]    Test 1D matching pattern in LR direction.
    ${p}=    Evaluate
    ...    tabularix.RangePattern1D(tabularix.value("A"), tabularix.value("B"), tabularix.value("C"))
    ...    modules=tabularix
    ${m}=    Evaluate    $p.to_matcher(direction="LR")
    ${res_ok}=    Evaluate    $m.matches_range([["A", "B", "C"]])
    Should Be True    ${res_ok}
    ${res_fail}=    Evaluate    $m.matches_range([["C", "B", "A"]])
    Should Be Equal    ${res_fail}    ${False}

Verify 1D Match Direction RL
    [Documentation]    Test 1D matching pattern in RL direction.
    ${p}=    Evaluate
    ...    tabularix.RangePattern1D(tabularix.value("A"), tabularix.value("B"), tabularix.value("C"))
    ...    modules=tabularix
    ${m}=    Evaluate    $p.to_matcher(direction="RL")
    ${res_ok}=    Evaluate    $m.matches_range([["C", "B", "A"]])
    Should Be True    ${res_ok}
    ${res_fail}=    Evaluate    $m.matches_range([["A", "B", "C"]])
    Should Be Equal    ${res_fail}    ${False}

Verify 1D Match Direction TB
    [Documentation]    Test 1D matching pattern in TB direction.
    ${p}=    Evaluate
    ...    tabularix.RangePattern1D(tabularix.value("A"), tabularix.value("B"), tabularix.value("C"))
    ...    modules=tabularix
    ${m}=    Evaluate    $p.to_matcher(direction="TB")
    ${res_ok}=    Evaluate    $m.matches_range([["A"], ["B"], ["C"]])
    Should Be True    ${res_ok}
    ${res_fail}=    Evaluate    $m.matches_range([["C"], ["B"], ["A"]])
    Should Be Equal    ${res_fail}    ${False}

Verify 1D Match Direction BT
    [Documentation]    Test 1D matching pattern in BT direction.
    ${p}=    Evaluate
    ...    tabularix.RangePattern1D(tabularix.value("A"), tabularix.value("B"), tabularix.value("C"))
    ...    modules=tabularix
    ${m}=    Evaluate    $p.to_matcher(direction="BT")
    ${res_ok}=    Evaluate    $m.matches_range([["C"], ["B"], ["A"]])
    Should Be True    ${res_ok}
    ${res_fail}=    Evaluate    $m.matches_range([["A"], ["B"], ["C"]])
    Should Be Equal    ${res_fail}    ${False}

Verify 2D Match Directions TB LR
    [Documentation]    Test 2D matching pattern with TB outer and LR inner directions (positive & negative).
    ${p}=    Get 2D Sample Pattern
    ${m}=    Evaluate    $p.to_matcher(outer_direction="TB", inner_direction="LR")
    ${res_ok}=    Evaluate    $m.matches_range([["1", "2"], ["3", "4"]])
    Should Be True    ${res_ok}
    ${res_fail1}=    Evaluate    $m.matches_range([["2", "1"], ["4", "3"]])
    Should Be Equal    ${res_fail1}    ${False}
    ${res_fail2}=    Evaluate    $m.matches_range([["1", "2"], ["3", "X"]])
    Should Be Equal    ${res_fail2}    ${False}

Verify 2D Match Directions TB RL
    [Documentation]    Test 2D matching pattern with TB outer and RL inner directions (positive & negative).
    ${p}=    Get 2D Sample Pattern
    ${m}=    Evaluate    $p.to_matcher(outer_direction="TB", inner_direction="RL")
    ${res_ok}=    Evaluate    $m.matches_range([["2", "1"], ["4", "3"]])
    Should Be True    ${res_ok}
    ${res_fail1}=    Evaluate    $m.matches_range([["1", "2"], ["3", "4"]])
    Should Be Equal    ${res_fail1}    ${False}
    ${res_fail2}=    Evaluate    $m.matches_range([["2", "1"], ["4", "X"]])
    Should Be Equal    ${res_fail2}    ${False}

Verify 2D Match Directions BT LR
    [Documentation]    Test 2D matching pattern with BT outer and LR inner directions (positive & negative).
    ${p}=    Get 2D Sample Pattern
    ${m}=    Evaluate    $p.to_matcher(outer_direction="BT", inner_direction="LR")
    ${res_ok}=    Evaluate    $m.matches_range([["3", "4"], ["1", "2"]])
    Should Be True    ${res_ok}
    ${res_fail1}=    Evaluate    $m.matches_range([["1", "2"], ["3", "4"]])
    Should Be Equal    ${res_fail1}    ${False}
    ${res_fail2}=    Evaluate    $m.matches_range([["3", "4"], ["1", "X"]])
    Should Be Equal    ${res_fail2}    ${False}

Verify 2D Match Directions BT RL
    [Documentation]    Test 2D matching pattern with BT outer and RL inner directions (positive & negative).
    ${p}=    Get 2D Sample Pattern
    ${m}=    Evaluate    $p.to_matcher(outer_direction="BT", inner_direction="RL")
    ${res_ok}=    Evaluate    $m.matches_range([["4", "3"], ["2", "1"]])
    Should Be True    ${res_ok}
    ${res_fail1}=    Evaluate    $m.matches_range([["3", "4"], ["1", "2"]])
    Should Be Equal    ${res_fail1}    ${False}
    ${res_fail2}=    Evaluate    $m.matches_range([["4", "3"], ["2", "X"]])
    Should Be Equal    ${res_fail2}    ${False}

Verify 2D Match Directions LR TB
    [Documentation]    Test 2D matching pattern with LR outer and TB inner directions (positive & negative).
    ${p}=    Get 2D Sample Pattern
    ${m}=    Evaluate    $p.to_matcher(outer_direction="LR", inner_direction="TB")
    ${res_ok}=    Evaluate    $m.matches_range([["1", "3"], ["2", "4"]])
    Should Be True    ${res_ok}
    ${res_fail1}=    Evaluate    $m.matches_range([["1", "2"], ["3", "4"]])
    Should Be Equal    ${res_fail1}    ${False}
    ${res_fail2}=    Evaluate    $m.matches_range([["1", "3"], ["2", "X"]])
    Should Be Equal    ${res_fail2}    ${False}

Verify 2D Match Directions LR BT
    [Documentation]    Test 2D matching pattern with LR outer and BT inner directions (positive & negative).
    ${p}=    Get 2D Sample Pattern
    ${m}=    Evaluate    $p.to_matcher(outer_direction="LR", inner_direction="BT")
    ${res_ok}=    Evaluate    $m.matches_range([["2", "4"], ["1", "3"]])
    Should Be True    ${res_ok}
    ${res_fail1}=    Evaluate    $m.matches_range([["2", "1"], ["4", "3"]])
    Should Be Equal    ${res_fail1}    ${False}
    ${res_fail2}=    Evaluate    $m.matches_range([["2", "4"], ["1", "X"]])
    Should Be Equal    ${res_fail2}    ${False}

Verify 2D Match Directions RL TB
    [Documentation]    Test 2D matching pattern with RL outer and TB inner directions (positive & negative).
    ${p}=    Get 2D Sample Pattern
    ${m}=    Evaluate    $p.to_matcher(outer_direction="RL", inner_direction="TB")
    ${res_ok}=    Evaluate    $m.matches_range([["3", "1"], ["4", "2"]])
    Should Be True    ${res_ok}
    ${res_fail1}=    Evaluate    $m.matches_range([["3", "4"], ["1", "2"]])
    Should Be Equal    ${res_fail1}    ${False}
    ${res_fail2}=    Evaluate    $m.matches_range([["3", "1"], ["4", "X"]])
    Should Be Equal    ${res_fail2}    ${False}

Verify 2D Match Directions RL BT
    [Documentation]    Test 2D matching pattern with RL outer and BT inner directions (positive & negative).
    ${p}=    Get 2D Sample Pattern
    ${m}=    Evaluate    $p.to_matcher(outer_direction="RL", inner_direction="BT")
    ${res_ok}=    Evaluate    $m.matches_range([["4", "2"], ["3", "1"]])
    Should Be True    ${res_ok}
    ${res_fail1}=    Evaluate    $m.matches_range([["4", "3"], ["2", "1"]])
    Should Be Equal    ${res_fail1}    ${False}
    ${res_fail2}=    Evaluate    $m.matches_range([["4", "2"], ["3", "X"]])
    Should Be Equal    ${res_fail2}    ${False}


*** Keywords ***
Get 2D Sample Pattern
    [Documentation]    Helper to build a 2D sample pattern [[1, 2], [3, 4]]
    ${r1}=    Evaluate    tabularix.RangePattern1D(tabularix.value("1"), tabularix.value("2"))    modules=tabularix
    ${r2}=    Evaluate    tabularix.RangePattern1D(tabularix.value("3"), tabularix.value("4"))    modules=tabularix
    ${p}=    Evaluate    tabularix.RangePattern2D($r1, $r2)    modules=tabularix
    RETURN    ${p}
