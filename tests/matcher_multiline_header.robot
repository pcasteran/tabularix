*** Settings ***
Documentation       Acceptance tests for multiline headers matching using RowGroupMatcher.

Library             Collections


*** Test Cases ***
Verify Multiline Header Matching - Success Case
    [Documentation]    Verify matching a sequence of 3 header rows with distinct cell patterns.
    ${matcher}=    Get Multiline Header Matcher
    ${expr}=    Catenate    SEPARATOR=\n
    ...    [
    ...    ["Sales Report 2026", None, None, None, None],
    ...    ["Product", "Q1", "Q2", None, None],
    ...    [None, "Actual", "Forecast", "Actual", "Forecast"]
    ...    ]
    ${rows}=    Evaluate    ${expr}
    ${res}=    Evaluate    $matcher.matches_row_group($rows)
    Should Be True    ${res}

Verify Multiline Header Matching - Incorrect Title
    [Documentation]    Verify failure when header title is incorrect.
    ${matcher}=    Get Multiline Header Matcher
    ${expr}=    Catenate    SEPARATOR=\n
    ...    [
    ...    ["Incorrect Title", None, None, None, None],
    ...    ["Product", "Q1", "Q2", None, None],
    ...    [None, "Actual", "Forecast", "Actual", "Forecast"]
    ...    ]
    ${rows}=    Evaluate    ${expr}
    ${res}=    Evaluate    $matcher.matches_row_group($rows)
    Should Be Equal    ${res}    ${False}

Verify Multiline Header Matching - Incorrect Subheaders Count
    [Documentation]    Verify failure when subheaders count is incorrect.
    ${matcher}=    Get Multiline Header Matcher
    ${expr}=    Catenate    SEPARATOR=\n
    ...    [
    ...    ["Sales Report 2026", None, None, None, None],
    ...    ["Product", "Q1", "Q2", None, None],
    ...    [None, "Actual", "Forecast"]
    ...    ]
    ${rows}=    Evaluate    ${expr}
    ${res}=    Evaluate    $matcher.matches_row_group($rows)
    Should Be Equal    ${res}    ${False}


*** Keywords ***
Get Multiline Header Matcher
    [Documentation]    Returns a RowGroupMatcher configured for multiline headers.
    ${r1}=    Evaluate    tabularix.value("Sales Report 2026").empty().zero_or_more()    modules=tabularix
    ${r2}=    Evaluate
    ...    tabularix.value("Product").regex("^Q[1-4]$").repeat(2).empty().zero_or_more()
    ...    modules=tabularix
    ${r3}=    Evaluate    tabularix.empty().regex("^(Actual|Forecast)$").repeat(4)    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RowGroupMatcher().row($r1).row($r2).row($r3)    modules=tabularix
    RETURN    ${matcher}
