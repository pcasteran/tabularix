*** Settings ***
Documentation       Acceptance tests for multiline headers matching using RangeMatcher.

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
    ${res}=    Evaluate    $matcher.matches_range($rows)
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
    ${res}=    Evaluate    $matcher.matches_range($rows)
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
    ${res}=    Evaluate    $matcher.matches_range($rows)
    Should Be Equal    ${res}    ${False}


*** Keywords ***
Get Multiline Header Matcher
    [Documentation]    Returns a RangeMatcher configured for multiline headers.
    ${r1}=    Get Row Pattern 1
    ${r2}=    Get Row Pattern 2
    ${r3}=    Get Row Pattern 3
    ${pat}=    Evaluate    tabularix.RangePattern2D([$r1, $r2, $r3])    modules=tabularix
    ${matcher}=    Evaluate    $pat.to_matcher(outer_direction="TB", inner_direction="LR")
    RETURN    ${matcher}

Get Row Pattern 1
    [Documentation]    Returns pattern for the first row of headers.
    ${title}=    Evaluate    tabularix.value("Sales Report 2026")    modules=tabularix
    ${r1_empty}=    Evaluate    tabularix.empty().zero_or_more()    modules=tabularix
    ${r1}=    Evaluate    tabularix.RangePattern1D([$title, $r1_empty])    modules=tabularix
    RETURN    ${r1}

Get Row Pattern 2
    [Documentation]    Returns pattern for the second row of headers.
    ${prod}=    Evaluate    tabularix.value("Product")    modules=tabularix
    ${quarters}=    Evaluate    tabularix.regex("^Q[1-4]$").repeat(2)    modules=tabularix
    ${r2_empty}=    Evaluate    tabularix.empty().zero_or_more()    modules=tabularix
    ${r2}=    Evaluate    tabularix.RangePattern1D([$prod, $quarters, $r2_empty])    modules=tabularix
    RETURN    ${r2}

Get Row Pattern 3
    [Documentation]    Returns pattern for the third row of headers.
    ${r3_empty}=    Evaluate    tabularix.empty()    modules=tabularix
    ${r3_forecast}=    Evaluate    tabularix.regex("^(Actual|Forecast)$").repeat(4)    modules=tabularix
    ${r3}=    Evaluate    tabularix.RangePattern1D([$r3_empty, $r3_forecast])    modules=tabularix
    RETURN    ${r3}
