*** Settings ***
Documentation       Acceptance tests for RangeMatcher builder API and chaining.

Library             Collections


*** Test Cases ***
Verify RangePattern1D Cardinality Chaining
    [Documentation]    Verify RangePattern1D methods return the self instance to support chaining.
    ${pattern}=    Evaluate    tabularix.RangePattern1D([])    modules=tabularix
    ${res}=    Evaluate    $pattern.one_or_more()
    Should Be Equal    ${res}    ${pattern}

Verify Cell Pattern Exclusivity Error
    [Documentation]    Verify calling multiple cardinality methods on a pattern raises ValueError.
    ${pattern}=    Evaluate    tabularix.RangePattern1D([tabularix.value("Header").one_or_more()])    modules=tabularix
    Run Keyword And Expect Error    *Cannot set multiple cardinalities*    Evaluate    $pattern.to_rust().one_or_more()
