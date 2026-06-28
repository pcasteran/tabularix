*** Settings ***
Documentation       Acceptance tests for RangeMatcher scanning and matching directions (1D and 2D).

Library             Collections


*** Test Cases ***
Verify 1D Match Directions
    [Documentation]    Test 1D matching patterns (LR, RL, TB, BT).
    # Pattern: A, B, C
    ${p}=    Evaluate    tabularix.RangePattern1D([tabularix.value("A"), tabularix.value("B"), tabularix.value("C")])    modules=tabularix

    # LR: Matches horizontal A, B, C
    ${m_lr}=    Evaluate    $p.to_matcher(direction="LR")
    ${res_lr_ok}=    Evaluate    $m_lr.matches_range([["A", "B", "C"]])
    Should Be True    ${res_lr_ok}
    ${res_lr_fail}=    Evaluate    $m_lr.matches_range([["C", "B", "A"]])
    Should Be Equal    ${res_lr_fail}    ${False}

    # RL: Matches horizontal C, B, A
    ${m_rl}=    Evaluate    $p.to_matcher(direction="RL")
    ${res_rl_ok}=    Evaluate    $m_rl.matches_range([["C", "B", "A"]])
    Should Be True    ${res_rl_ok}
    ${res_rl_fail}=    Evaluate    $m_rl.matches_range([["A", "B", "C"]])
    Should Be Equal    ${res_rl_fail}    ${False}

    # TB: Matches vertical A, B, C
    ${m_tb}=    Evaluate    $p.to_matcher(direction="TB")
    ${res_tb_ok}=    Evaluate    $m_tb.matches_range([["A"], ["B"], ["C"]])
    Should Be True    ${res_tb_ok}
    ${res_tb_fail}=    Evaluate    $m_tb.matches_range([["C"], ["B"], ["A"]])
    Should Be Equal    ${res_tb_fail}    ${False}

    # BT: Matches vertical C, B, A
    ${m_bt}=    Evaluate    $p.to_matcher(direction="BT")
    ${res_bt_ok}=    Evaluate    $m_bt.matches_range([["C"], ["B"], ["A"]])
    Should Be True    ${res_bt_ok}
    ${res_bt_fail}=    Evaluate    $m_bt.matches_range([["A"], ["B"], ["C"]])
    Should Be Equal    ${res_bt_fail}    ${False}

Verify 2D Match Directions Horizontal Outer
    [Documentation]    Test 2D matching patterns with vertical outer (TB, BT) and horizontal inner (LR, RL).
    # Pattern: [[1, 2], [3, 4]]
    ${r1}=    Evaluate    tabularix.RangePattern1D([tabularix.value("1"), tabularix.value("2")])    modules=tabularix
    ${r2}=    Evaluate    tabularix.RangePattern1D([tabularix.value("3"), tabularix.value("4")])    modules=tabularix
    ${p}=    Evaluate    tabularix.RangePattern2D([$r1, $r2])    modules=tabularix

    # TB / LR: Matches [[1, 2], [3, 4]]
    ${m_tb_lr}=    Evaluate    $p.to_matcher(outer_direction="TB", inner_direction="LR")
    ${res_tb_lr}=    Evaluate    $m_tb_lr.matches_range([["1", "2"], ["3", "4"]])
    Should Be True    ${res_tb_lr}

    # TB / RL: Matches [[2, 1], [4, 3]]
    ${m_tb_rl}=    Evaluate    $p.to_matcher(outer_direction="TB", inner_direction="RL")
    ${res_tb_rl}=    Evaluate    $m_tb_rl.matches_range([["2", "1"], ["4", "3"]])
    Should Be True    ${res_tb_rl}

    # BT / LR: Matches [[3, 4], [1, 2]]
    ${m_bt_lr}=    Evaluate    $p.to_matcher(outer_direction="BT", inner_direction="LR")
    ${res_bt_lr}=    Evaluate    $m_bt_lr.matches_range([["3", "4"], ["1", "2"]])
    Should Be True    ${res_bt_lr}

    # BT / RL: Matches [[4, 3], [2, 1]]
    ${m_bt_rl}=    Evaluate    $p.to_matcher(outer_direction="BT", inner_direction="RL")
    ${res_bt_rl}=    Evaluate    $m_bt_rl.matches_range([["4", "3"], ["2", "1"]])
    Should Be True    ${res_bt_rl}

Verify 2D Match Directions Vertical Outer
    [Documentation]    Test 2D matching patterns with horizontal outer (LR, RL) and vertical inner (TB, BT).
    # Pattern: Column 1: [1, 2], Column 2: [3, 4]
    ${c1}=    Evaluate    tabularix.RangePattern1D([tabularix.value("1"), tabularix.value("2")])    modules=tabularix
    ${c2}=    Evaluate    tabularix.RangePattern1D([tabularix.value("3"), tabularix.value("4")])    modules=tabularix
    ${p}=    Evaluate    tabularix.RangePattern2D([$c1, $c2])    modules=tabularix

    # LR / TB: Matches [[1, 3], [2, 4]]
    ${m_lr_tb}=    Evaluate    $p.to_matcher(outer_direction="LR", inner_direction="TB")
    ${res_lr_tb}=    Evaluate    $m_lr_tb.matches_range([["1", "3"], ["2", "4"]])
    Should Be True    ${res_lr_tb}

    # LR / BT: Matches [[2, 4], [1, 3]]
    ${m_lr_bt}=    Evaluate    $p.to_matcher(outer_direction="LR", inner_direction="BT")
    ${res_lr_bt}=    Evaluate    $m_lr_bt.matches_range([["2", "4"], ["1", "3"]])
    Should Be True    ${res_lr_bt}

    # RL / TB: Matches [[3, 1], [4, 2]]
    ${m_rl_tb}=    Evaluate    $p.to_matcher(outer_direction="RL", inner_direction="TB")
    ${res_rl_tb}=    Evaluate    $m_rl_tb.matches_range([["3", "1"], ["4", "2"]])
    Should Be True    ${res_rl_tb}

    # RL / BT: Matches [[4, 2], [3, 1]]
    ${m_rl_bt}=    Evaluate    $p.to_matcher(outer_direction="RL", inner_direction="BT")
    ${res_rl_bt}=    Evaluate    $m_rl_bt.matches_range([["4", "2"], ["3", "1"]])
    Should Be True    ${res_rl_bt}
