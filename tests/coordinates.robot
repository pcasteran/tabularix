*** Settings ***
Documentation     Acceptance tests for A1 coordinate conversion API.

*** Test Cases ***
Verify Index To A1 Conversion
    ${a1_1}=    Evaluate    tabularix.index_to_a1(0, 0)    modules=tabularix
    Should Be Equal As Strings    ${a1_1}    A1

    ${a1_2}=    Evaluate    tabularix.index_to_a1(9, 25)    modules=tabularix
    Should Be Equal As Strings    ${a1_2}    Z10

    ${a1_3}=    Evaluate    tabularix.index_to_a1(0, 26)    modules=tabularix
    Should Be Equal As Strings    ${a1_3}    AA1

Verify A1 To Index Conversion
    ${idx_1}=    Evaluate    tabularix.a1_to_index("A1")    modules=tabularix
    ${expected_idx_1}=    Evaluate    (0, 0)
    Should Be Equal    ${idx_1}    ${expected_idx_1}

    ${idx_2}=    Evaluate    tabularix.a1_to_index("Z10")    modules=tabularix
    ${expected_idx_2}=    Evaluate    (9, 25)
    Should Be Equal    ${idx_2}    ${expected_idx_2}

    ${idx_3}=    Evaluate    tabularix.a1_to_index("AA1")    modules=tabularix
    ${expected_idx_3}=    Evaluate    (0, 26)
    Should Be Equal    ${idx_3}    ${expected_idx_3}

Verify Invalid A1 Conversions Raise ValueError
    Run Keyword And Expect Error    *ValueError: Invalid A1 notation*    Evaluate    tabularix.a1_to_index("A")    modules=tabularix
    Run Keyword And Expect Error    *ValueError: Invalid A1 notation*    Evaluate    tabularix.a1_to_index("1")    modules=tabularix
