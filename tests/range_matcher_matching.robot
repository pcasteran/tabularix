*** Settings ***
Documentation       Acceptance tests for RangeMatcher cell matching rules.

Library             Collections


*** Test Cases ***
Verify Match Exact Values
    [Documentation]    Verify exact string match value rule.
    ${pattern}=    Evaluate    tabularix.RangePattern1D([tabularix.value("Total")])    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    ${res1}=    Evaluate    $matcher.matches_range([["Total"]])
    Should Be True    ${res1}

    ${res2}=    Evaluate    $matcher.matches_range([["Subtotal"]])
    Should Be Equal    ${res2}    ${False}

Verify Match Non Empty
    [Documentation]    Verify non_empty matches any non-empty cell.
    ${pattern}=    Evaluate    tabularix.RangePattern1D([tabularix.non_empty()])    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    ${res1}=    Evaluate    $matcher.matches_range([["Anything"]])
    Should Be True    ${res1}

    ${res2}=    Evaluate    $matcher.matches_range([[None]])
    Should Be Equal    ${res2}    ${False}

Verify Match Empty
    [Documentation]    Verify empty matches blank cells.
    ${pattern}=    Evaluate    tabularix.RangePattern1D([tabularix.empty()])    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    ${res1}=    Evaluate    $matcher.matches_range([[None]])
    Should Be True    ${res1}

    ${res2}=    Evaluate    $matcher.matches_range([["Something"]])
    Should Be Equal    ${res2}    ${False}

Verify Match Regex String
    [Documentation]    Verify regex matching using a plain string regex pattern.
    ${pattern}=    Evaluate    tabularix.RangePattern1D([tabularix.regex("^Q[1-4]$")])    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    ${res1}=    Evaluate    $matcher.matches_range([["Q3"]])
    Should Be True    ${res1}

    ${res2}=    Evaluate    $matcher.matches_range([["Q5"]])
    Should Be Equal    ${res2}    ${False}

Verify Match Regex Compiled
    [Documentation]    Verify regex matching using a compiled Python regex object (re.compile).
    ${re}=    Evaluate    re.compile("^\\\\d{4}$")    modules=re
    ${pattern}=    Evaluate    tabularix.RangePattern1D([tabularix.regex($re)])    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    ${res1}=    Evaluate    $matcher.matches_range([["2026"]])
    Should Be True    ${res1}

    ${res2}=    Evaluate    $matcher.matches_range([["abc"]])
    Should Be Equal    ${res2}    ${False}

Verify Match Nested Group
    [Documentation]    Verify matching a nested group pattern like (Expected Actual)+.
    ${sub}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("Expected"), tabularix.value("Actual")])
    ...    modules=tabularix
    ${pattern}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("Product"), $sub.one_or_more()])
    ...    modules=tabularix
    ${matcher}=    Evaluate    $pattern.to_matcher(direction="LR")

    # Matches: Product, and two pairs of Expected/Actual
    ${res1}=    Evaluate    $matcher.matches_range([["Product", "Expected", "Actual", "Expected", "Actual"]])
    Should Be True    ${res1}

    # Fails: Product, then Expected, then Expected
    ${res2}=    Evaluate    $matcher.matches_range([["Product", "Expected", "Expected"]])
    Should Be Equal    ${res2}    ${False}

    # Fails: Product only
    ${res3}=    Evaluate    $matcher.matches_range([["Product"]])
    Should Be Equal    ${res3}    ${False}
