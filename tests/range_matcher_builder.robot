*** Settings ***
Documentation       Acceptance tests for RangeMatcher builder API and chaining.

Library             Collections


*** Test Cases ***
Verify CellGroupPattern Builder Chaining
    [Documentation]    Verify CellGroupPattern methods return the self instance to support chaining.
    ${pattern}=    Evaluate    tabularix.CellGroupPattern()    modules=tabularix
    ${res}=    Evaluate    $pattern.value("Date").empty().non_empty().any()
    Should Be Equal    ${res}    ${pattern}

Verify RangeMatcher Builder Chaining
    [Documentation]    Verify RangeMatcher methods return the self instance to support chaining.
    ${pattern}=    Evaluate    tabularix.CellGroupPattern()    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RangeMatcher()    modules=tabularix
    ${res}=    Evaluate    $matcher.row($pattern).one_or_more()
    Should Be Equal    ${res}    ${matcher}

Verify Cell Cardinality Exclusivity Error
    [Documentation]    Verify calling multiple cardinality methods on a cell pattern raises ValueError.
    ${pattern}=    Evaluate    tabularix.CellGroupPattern().empty()    modules=tabularix
    Evaluate    $pattern.optional()
    Run Keyword And Expect Error    *Cannot set multiple cardinalities*    Evaluate    $pattern.one_or_more()

Verify Row Cardinality Exclusivity Error
    [Documentation]    Verify calling multiple cardinality methods on a row pattern raises ValueError.
    ${pattern}=    Evaluate    tabularix.CellGroupPattern().value("Header")    modules=tabularix
    ${matcher}=    Evaluate    tabularix.RangeMatcher().row($pattern)    modules=tabularix
    Evaluate    $matcher.optional()
    Run Keyword And Expect Error    *Cannot set multiple cardinalities*    Evaluate    $matcher.one_or_more()
