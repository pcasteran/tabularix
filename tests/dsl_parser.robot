*** Settings ***
Documentation       Acceptance tests for Tabularix RangePattern Builder DSL.

Library             Collections
Resource            common.resource


*** Test Cases ***
Verify Parsing Short Exact and Regex Cell Rules
    [Documentation]    Test parsing CellRules in short form for exact and regex matches.
    ${rule_v}=    Evaluate    tabularix.parse_pattern_1d('[v: "Hello"]')    modules=tabularix
    Should Be Equal As Strings    ${rule_v.elements[0].rule_type}    exact
    Should Be Equal As Strings    ${rule_v.elements[0].value}    Hello

    ${rule_r}=    Evaluate    tabularix.parse_pattern_1d('[r: "^Q[1-4]$"]')    modules=tabularix
    Should Be Equal As Strings    ${rule_r.elements[0].rule_type}    regex
    Should Be Equal As Strings    ${rule_r.elements[0].value}    ^Q[1-4]$

Verify Parsing Short State Cell Rules
    [Documentation]    Test parsing CellRules in short form for empty, non-empty, and any.
    ${rule_e}=    Evaluate    tabularix.parse_pattern_1d('[e]')    modules=tabularix
    Should Be Equal As Strings    ${rule_e.elements[0].rule_type}    empty

    ${rule_ne}=    Evaluate    tabularix.parse_pattern_1d('[ne]')    modules=tabularix
    Should Be Equal As Strings    ${rule_ne.elements[0].rule_type}    non_empty

    ${rule_a}=    Evaluate    tabularix.parse_pattern_1d('[a]')    modules=tabularix
    Should Be Equal As Strings    ${rule_a.elements[0].rule_type}    any

Verify Parsing Long Exact and Regex Cell Rules
    [Documentation]    Test parsing CellRules in long form for value and regex.
    ${rule_value}=    Evaluate    tabularix.parse_pattern_1d('[value: "World"]')    modules=tabularix
    Should Be Equal As Strings    ${rule_value.elements[0].rule_type}    exact
    Should Be Equal As Strings    ${rule_value.elements[0].value}    World

    ${rule_regex}=    Evaluate    tabularix.parse_pattern_1d('[regex: "\\\\d+"]')    modules=tabularix
    Should Be Equal As Strings    ${rule_regex.elements[0].rule_type}    regex
    Should Be Equal As Strings    ${rule_regex.elements[0].value}    \\d+

Verify Parsing Long State Cell Rules
    [Documentation]    Test parsing CellRules in long form for empty, non-empty, and any.
    ${rule_empty}=    Evaluate    tabularix.parse_pattern_1d('[empty]')    modules=tabularix
    Should Be Equal As Strings    ${rule_empty.elements[0].rule_type}    empty

    ${rule_non_empty}=    Evaluate    tabularix.parse_pattern_1d('[non_empty]')    modules=tabularix
    Should Be Equal As Strings    ${rule_non_empty.elements[0].rule_type}    non_empty

    ${rule_any}=    Evaluate    tabularix.parse_pattern_1d('[any]')    modules=tabularix
    Should Be Equal As Strings    ${rule_any.elements[0].rule_type}    any

Verify Parsing Optional Quantifiers
    [Documentation]    Test parsing optional quantifiers ? and ??.
    ${rule_opt}=    Evaluate    tabularix.parse_pattern_1d('[e]?')    modules=tabularix
    Should Be Equal As Integers    ${rule_opt.elements[0].min}    0
    Should Be Equal As Integers    ${rule_opt.elements[0].max}    1
    Should Be Equal    ${rule_opt.elements[0].greedy}    ${True}

    ${rule_opt_lazy}=    Evaluate    tabularix.parse_pattern_1d('[e]??')    modules=tabularix
    Should Be Equal As Integers    ${rule_opt_lazy.elements[0].min}    0
    Should Be Equal As Integers    ${rule_opt_lazy.elements[0].max}    1
    Should Be Equal    ${rule_opt_lazy.elements[0].greedy}    ${False}

Verify Parsing Plus and Star Quantifiers
    [Documentation]    Test parsing + and * quantifiers.
    ${rule_plus}=    Evaluate    tabularix.parse_pattern_1d('[ne]+')    modules=tabularix
    Should Be Equal As Integers    ${rule_plus.elements[0].min}    1
    Should Be Equal    ${rule_plus.elements[0].max}    ${None}
    Should Be Equal    ${rule_plus.elements[0].greedy}    ${True}

    ${rule_star_lazy}=    Evaluate    tabularix.parse_pattern_1d('[a]*?')    modules=tabularix
    Should Be Equal As Integers    ${rule_star_lazy.elements[0].min}    0
    Should Be Equal    ${rule_star_lazy.elements[0].max}    ${None}
    Should Be Equal    ${rule_star_lazy.elements[0].greedy}    ${False}

Verify Parsing Repetition Range Quantifiers
    [Documentation]    Test parsing repetition quantifiers like {n}, {min,max}, {min,}.
    ${rule_rep}=    Evaluate    tabularix.parse_pattern_1d('[v: "X"]{3}')    modules=tabularix
    Should Be Equal As Integers    ${rule_rep.elements[0].min}    3
    Should Be Equal As Integers    ${rule_rep.elements[0].max}    3

    ${rule_range}=    Evaluate    tabularix.parse_pattern_1d('[v: "X"]{2,5}?')    modules=tabularix
    Should Be Equal As Integers    ${rule_range.elements[0].min}    2
    Should Be Equal As Integers    ${rule_range.elements[0].max}    5
    Should Be Equal    ${rule_range.elements[0].greedy}    ${False}

    ${rule_unbounded}=    Evaluate    tabularix.parse_pattern_1d('[v: "X"]{2,}')    modules=tabularix
    Should Be Equal As Integers    ${rule_unbounded.elements[0].min}    2
    Should Be Equal    ${rule_unbounded.elements[0].max}    ${None}

Verify Parsing 1D Pattern Grouping and Nesting
    [Documentation]    Test parsing comma-separated elements and nested sub-groups.
    ${pat}=    Evaluate
    ...    tabularix.parse_pattern_1d('[v: "Category"], ([r: "^Q[1-4]$"], [e]?){4}')
    ...    modules=tabularix
    Should Be Equal As Integers    ${pat.elements.__len__()}    2
    Should Be Equal As Strings    ${pat.elements[0].rule_type}    exact
    VAR    ${nested}=    ${pat.elements[1]}
    Should Be Equal As Integers    ${nested.min}    4
    Should Be Equal As Integers    ${nested.max}    4
    Should Be Equal As Integers    ${nested.elements.__len__()}    2

Verify Parsing 2D Patterns
    [Documentation]    Test parsing 2D semicolon-separated grid patterns.
    ${grid}=    Evaluate
    ...    tabularix.parse_pattern_2d('([v: "Region"], [r: "^Q[1-4]$"]{4}) ; ([r: "^(?!Total).*$"], [ne]{4})+')
    ...    modules=tabularix
    Should Be Equal As Integers    ${grid.patterns.__len__()}    2
    Should Be Equal As Strings    ${grid.patterns[0].elements[0].value}    Region
    Should Be Equal As Integers    ${grid.patterns[1].min}    1
    Should Be Equal    ${grid.patterns[1].max}    ${None}

Verify Parsing Multiline 2D Patterns
    [Documentation]    Test parsing multiline 2D patterns containing newlines.
    VAR    ${multiline}=    ([v: "Region"], [r: "^Q[1-4]$"]{4})\n;\n([r: "^(?!Total).*$"], [ne]{4})+
    ${grid}=    Evaluate    tabularix.parse_pattern_2d('''${multiline}''')    modules=tabularix
    Should Be Equal As Integers    ${grid.patterns.__len__()}    2
    Should Be Equal As Strings    ${grid.patterns[0].elements[0].value}    Region

Verify Parsing Strings With Semicolons And Single Quotes
    [Documentation]    Test string literals containing semicolons or single quotes.
    ${pat_semicolon}=    Evaluate    tabularix.parse_pattern_1d('[v: "a;b"]')    modules=tabularix
    Should Be Equal As Strings    ${pat_semicolon.elements[0].value}    a;b

    ${pat_single_quote}=    Evaluate    tabularix.parse_pattern_1d("[v: 'item']")    modules=tabularix
    Should Be Equal As Strings    ${pat_single_quote.elements[0].value}    item

Verify Syntax Error Diagnostics
    [Documentation]    Test syntax errors like missing brackets, missing row parentheses, and extra trailing tokens.
    Run Keyword And Expect Error    *Parse error at line 1, column 4: Expected token of type STRING, got RBRACKET*
    ...    Evaluate    tabularix.parse_pattern_1d('[v:]')    modules=tabularix

    # 2D pattern missing parenthesis around row
    Run Keyword And Expect Error    *Each row in a 2D pattern must be wrapped in parentheses*
    ...    Evaluate    tabularix.parse_pattern_2d('[v: "A"] ; ([v: "B"])')    modules=tabularix

    # Lazy exact quantifier rejected
    Run Keyword And Expect Error    *Exact count repetition '{3}?' cannot be lazy*
    ...    Evaluate    tabularix.parse_pattern_1d('[v: "X"]{3}?')    modules=tabularix

Verify Rule Identifier Error Diagnostics
    [Documentation]    Test diagnostics for unknown bare or attribute rule identifiers.
    # Unknown bare rule identifier
    Run Keyword And Expect Error    *Unknown bare state rule type 'invalid'*
    ...    Evaluate    tabularix.parse_pattern_1d('[invalid]')    modules=tabularix

    # Unknown attribute-value rule identifier
    Run Keyword And Expect Error    *Unknown attribute-value rule type 'bad'*
    ...    Evaluate    tabularix.parse_pattern_1d('[bad: "val"]')    modules=tabularix

    # Unexpected token after 2D pattern
    Run Keyword And Expect Error    *Unexpected token 'extra'*
    ...    Evaluate    tabularix.parse_pattern_2d('([v: "A"]) ; ([v: "B"]) extra')    modules=tabularix

Verify Stringification Round-Trip
    [Documentation]    Verify str(parsed) outputs the correct shorthand DSL representation.
    VAR    ${dsl_1d}=    [v: "Category"], ([r: "^Q[1-4]$"], [e]?){4}
    ${pat_1d}=    Evaluate    tabularix.parse_pattern_1d('''${dsl_1d}''')    modules=tabularix
    ${str_1d}=    Evaluate    str($pat_1d)
    Should Be Equal As Strings    ${str_1d}    ${dsl_1d}

    VAR    ${dsl_2d}=    ([v: "Region"], [r: "^Q[1-4]$"]{4}) ; ([r: "^(?!Total).*$"], [ne]{4})+
    ${pat_2d}=    Evaluate    tabularix.parse_pattern_2d('''${dsl_2d}''')    modules=tabularix
    ${str_2d}=    Evaluate    str($pat_2d)
    Should Be Equal As Strings    ${str_2d}    ${dsl_2d}

Verify Debug Repr Expression
    [Documentation]    Verify repr(parsed) outputs valid python constructor expressions.
    ${pat_1d}=    Evaluate    tabularix.parse_pattern_1d('[v: "Category"], [e]?')    modules=tabularix
    ${repr_1d}=    Evaluate    repr($pat_1d)
    Should Be Equal As Strings    ${repr_1d}    RangePattern1D(value('Category'), empty().optional(greedy=True))

    ${pat_2d}=    Evaluate    tabularix.parse_pattern_2d('([v: "A"]) ; ([v: "B"])+')    modules=tabularix
    ${repr_2d}=    Evaluate    repr($pat_2d)
    Should Be Equal As Strings
    ...    ${repr_2d}
    ...    RangePattern2D(RangePattern1D(value('A')), RangePattern1D(value('B')).one_or_more(greedy=True))

Verify High Level Table Extraction With Parsed DSL Patterns
    [Documentation]    Verify extract_table_with_header_and_data using DSL parsed 1D and 2D patterns.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("complex")
    ${header}=    Evaluate    tabularix.parse_pattern_1d('[v: "Region"], [r: "^Q[1-4]$"]{4}')    modules=tabularix
    ${data}=    Evaluate
    ...    tabularix.parse_pattern_2d('([r: "^(?!Total).*$"], [ne]{4})+')
    ...    modules=tabularix
    ${table}=    Evaluate    tabularix.extract_table_with_header_and_data($sheet, $header, $data)    modules=tabularix
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['region', 'q1', 'q2', 'q3', 'q4']
    ${shape}=    Evaluate    $table.shape
    Should Be Equal As Strings    ${shape}    (4, 5)
