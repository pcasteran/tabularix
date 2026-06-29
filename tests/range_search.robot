*** Settings ***
Documentation       Acceptance tests for Sheet.search_range and Sheet.search_range_relative.

Library             Collections
Resource            common.resource


*** Test Cases ***
Verify Absolute Range Search Basic
    [Documentation]    Test Sheet.search_range with default coordinates.
    ${sheet}=    Load Simple Sheet
    ${hdr_p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.non_empty(), tabularix.any(), tabularix.any()])
    ...    modules=tabularix
    ${matcher}=    Evaluate    $hdr_p.to_matcher(direction="LR")

    ${range}=    Evaluate    $sheet.search_range($matcher)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    0    0    0    2

    ${repr}=    Evaluate    repr($range)
    Should Be Equal    ${repr}    Range(A1:C1, cols=0..2, rows=0..0)

Verify Absolute Range Search Sliced
    [Documentation]    Test Sheet.search_range within a sliced window.
    ${sheet}=    Load Simple Sheet
    ${hdr_p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.non_empty(), tabularix.any(), tabularix.any()])
    ...    modules=tabularix
    ${matcher}=    Evaluate    $hdr_p.to_matcher(direction="LR")

    ${range_sliced}=    Evaluate    $sheet.search_range($matcher, start_row=1)
    Should Not Be Equal    ${range_sliced}    ${None}
    Verify Range Coordinates    ${range_sliced}    1    1    0    2

Verify Absolute Range Search Bounds and Errors
    [Documentation]    Test negative and out of bounds errors in search_range.
    ${sheet}=    Load Simple Sheet
    ${p}=    Evaluate    tabularix.RangePattern1D([tabularix.any()])    modules=tabularix
    ${matcher}=    Evaluate    $p.to_matcher(direction="LR")

    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_range($matcher, start_row=-1)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_range($matcher, end_row=100)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_range($matcher, start_col=-5)
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.search_range($matcher, end_col=10)

Verify Absolute Range Search Inverted Bounds
    [Documentation]    Test inverted range errors in search_range.
    ${sheet}=    Load Simple Sheet
    ${p}=    Evaluate    tabularix.RangePattern1D([tabularix.any()])    modules=tabularix
    ${matcher}=    Evaluate    $p.to_matcher(direction="LR")

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range($matcher, start_row=3, end_row=1)
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range($matcher, start_col=2, end_col=1)

Verify Relative Range Search Below
    [Documentation]    Test search_range_relative below a matched range.
    ${sheet}=    Load Simple Sheet
    ${header_range}=    Find Range By Value In Simple Sheet    ${sheet}    Header #1

    ${abc_p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("ABC"), tabularix.any(), tabularix.any()])
    ...    modules=tabularix
    ${abc_m}=    Evaluate    $abc_p.to_matcher(direction="LR")
    ${abc_range}=    Evaluate    $sheet.search_range_relative($abc_m, below=$header_range)
    Should Not Be Equal    ${abc_range}    ${None}
    Verify Range Coordinates    ${abc_range}    1    1    0    2

Verify Relative Range Search Below and Above
    [Documentation]    Test search_range_relative bounded vertically by below and above.
    ${sheet}=    Load Simple Sheet
    ${header_range}=    Find Range By Value In Simple Sheet    ${sheet}    Header #1
    ${def_range}=    Find Range By Value In Simple Sheet    ${sheet}    DEF

    ${abc_p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("ABC"), tabularix.any(), tabularix.any()])
    ...    modules=tabularix
    ${abc_m}=    Evaluate    $abc_p.to_matcher(direction="LR")
    ${abc_range_both}=    Evaluate    $sheet.search_range_relative($abc_m, below=$header_range, above=$def_range)
    Should Not Be Equal    ${abc_range_both}    ${None}
    Verify Range Coordinates    ${abc_range_both}    1    1    0    2

Verify Relative Range Search Vertical Conflicts
    [Documentation]    Test vertical boundary conflicts when 'below' is above 'above'.
    ${sheet}=    Load Simple Sheet
    ${header_range}=    Find Range By Value In Simple Sheet    ${sheet}    Header #1
    ${def_range}=    Find Range By Value In Simple Sheet    ${sheet}    DEF

    ${abc_p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("ABC"), tabularix.any(), tabularix.any()])
    ...    modules=tabularix
    ${abc_m}=    Evaluate    $abc_p.to_matcher(direction="LR")

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range_relative($abc_m, below=$def_range, above=$header_range)

Verify Relative Range Search Vertical Conflicts - Row Spans
    [Documentation]    Test vertical boundary conflicts when column spans do not align.
    ${sheet}=    Load Simple Sheet
    ${abc_p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("ABC"), tabularix.any(), tabularix.any()])
    ...    modules=tabularix
    ${abc_m}=    Evaluate    $abc_p.to_matcher(direction="LR")

    ${rg1}=    Evaluate    tabularix.Range.from_a1("A2:C2")    modules=tabularix
    ${rg2}=    Evaluate    tabularix.Range.from_a1("A4:B4")    modules=tabularix
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range_relative($abc_m, below=$rg1, above=$rg2)

Verify Relative Range Search Horizontal Bounds
    [Documentation]    Test search_range_relative with horizontal boundaries (left, right).
    ${sheet}=    Load Simple Sheet
    ${right_rg}=    Evaluate    tabularix.Range.from_a1("A2:A3")    modules=tabularix
    ${left_rg}=    Evaluate    tabularix.Range.from_a1("C2:C3")    modules=tabularix

    ${val_p}=    Evaluate    tabularix.RangePattern1D([tabularix.non_empty()])    modules=tabularix
    ${val_m}=    Evaluate    $val_p.to_matcher(direction="LR")

    ${range}=    Evaluate    $sheet.search_range_relative($val_m, right=$right_rg, left=$left_rg)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    1    1    1    1

Verify Relative Range Search Horizontal Conflicts
    [Documentation]    Test horizontal boundary conflicts.
    ${sheet}=    Load Simple Sheet
    ${right_rg}=    Evaluate    tabularix.Range.from_a1("A2:A3")    modules=tabularix
    ${left_rg}=    Evaluate    tabularix.Range.from_a1("C2:C3")    modules=tabularix

    ${val_p}=    Evaluate    tabularix.RangePattern1D([tabularix.non_empty()])    modules=tabularix
    ${val_m}=    Evaluate    $val_p.to_matcher(direction="LR")

    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range_relative($val_m, right=$left_rg, left=$right_rg)

    ${rg_row1}=    Evaluate    tabularix.Range.from_a1("A2")    modules=tabularix
    ${rg_row2}=    Evaluate    tabularix.Range.from_a1("C3")    modules=tabularix
    Run Keyword And Expect Error
    ...    *ValueError*
    ...    Evaluate
    ...    $sheet.search_range_relative($val_m, right=$rg_row1, left=$rg_row2)

Verify Relative Range Search Above
    [Documentation]    Test search_range_relative above a matched range.
    ${sheet}=    Load Simple Sheet
    ${def_range}=    Find Range By Value In Simple Sheet    ${sheet}    DEF
    ${abc_p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("ABC"), tabularix.any(), tabularix.any()])
    ...    modules=tabularix
    ${abc_m}=    Evaluate    $abc_p.to_matcher(direction="LR")
    ${abc_range}=    Evaluate    $sheet.search_range_relative($abc_m, above=$def_range)
    Should Not Be Equal    ${abc_range}    ${None}
    Verify Range Coordinates    ${abc_range}    1    1    0    2

Verify Relative Range Search Left
    [Documentation]    Test search_range_relative left of a matched range.
    ${sheet}=    Load Simple Sheet
    ${left_rg}=    Evaluate    tabularix.Range.from_a1("C2:C3")    modules=tabularix
    ${def_p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("DEF"), tabularix.any()])
    ...    modules=tabularix
    ${def_m}=    Evaluate    $def_p.to_matcher(direction="LR")
    ${def_range}=    Evaluate    $sheet.search_range_relative($def_m, left=$left_rg)
    Should Not Be Equal    ${def_range}    ${None}
    Verify Range Coordinates    ${def_range}    2    2    0    1

Verify Relative Range Search Right
    [Documentation]    Test search_range_relative right of a matched range.
    ${sheet}=    Load Simple Sheet
    ${right_rg}=    Evaluate    tabularix.Range.from_a1("A1:A4")    modules=tabularix
    ${alice_p}=    Evaluate    tabularix.RangePattern1D([tabularix.value("Alice")])    modules=tabularix
    ${alice_m}=    Evaluate    $alice_p.to_matcher(direction="LR")
    ${alice_range}=    Evaluate    $sheet.search_range_relative($alice_m, right=$right_rg)
    Should Not Be Equal    ${alice_range}    ${None}
    Verify Range Coordinates    ${alice_range}    1    1    2    2

Verify Partial Column Range Search
    [Documentation]    Test that RangeMatcher can match a subset of columns in a wider sheet.
    ${sheet}=    Load Simple Sheet
    # Match Header #2 and Header #3 (columns 1 and 2)
    ${p}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("Header #2"), tabularix.value("Header #3")])
    ...    modules=tabularix
    ${matcher}=    Evaluate    $p.to_matcher(direction="LR")
    ${range}=    Evaluate    $sheet.search_range($matcher)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    0    0    1    2

Verify Greedy Range Search Matching
    [Documentation]    Verify search_range matches the largest matching column span (greedy).
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${sub}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("Expected"), tabularix.value("Actual")])
    ...    modules=tabularix
    ${sub_hdr}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.regex(r"\\d{4}"), tabularix.empty()])
    ...    modules=tabularix
    ${hdr1}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("Product"), $sub_hdr.zero_or_more(greedy=True)])
    ...    modules=tabularix
    ${hdr2}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.empty(), $sub.zero_or_more(greedy=True)])
    ...    modules=tabularix
    ${matcher}=    Evaluate
    ...    tabularix.RangePattern2D([$hdr1, $hdr2]).to_matcher(outer_direction="TB", inner_direction="LR")
    ${range}=    Evaluate    $sheet.search_range($matcher)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    12    13    1    5

Verify Lazy Range Search Matching
    [Documentation]    Verify search_range matches the smallest matching column span when lazy.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${sub}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("Expected"), tabularix.value("Actual")])
    ...    modules=tabularix
    ${sub_hdr}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.regex(r"\\d{4}"), tabularix.empty()])
    ...    modules=tabularix
    ${hdr1}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.value("Product"), $sub_hdr.zero_or_more(greedy=False)])
    ...    modules=tabularix
    ${hdr2}=    Evaluate
    ...    tabularix.RangePattern1D([tabularix.empty(), $sub.zero_or_more(greedy=False)])
    ...    modules=tabularix
    ${matcher}=    Evaluate
    ...    tabularix.RangePattern2D([$hdr1, $hdr2]).to_matcher(outer_direction="TB", inner_direction="LR")
    ${range}=    Evaluate    $sheet.search_range($matcher)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    12    13    1    1

Verify Greedy Row Repetition Search Matching
    [Documentation]    Verify vertical greedy row matching matches as many rows as possible.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${row_pat}=    Evaluate    tabularix.RangePattern1D([tabularix.regex(r"^Product .*$")])    modules=tabularix
    ${pat}=    Evaluate    tabularix.RangePattern2D([$row_pat.one_or_more(greedy=True)])    modules=tabularix
    ${matcher}=    Evaluate    $pat.to_matcher(outer_direction="TB", inner_direction="LR")
    ${range}=    Evaluate    $sheet.search_range($matcher, start_row=14, end_row=16, start_col=1, end_col=1)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    14    16    1    1

Verify Lazy Row Repetition Search Matching
    [Documentation]    Verify vertical lazy row matching matches as few rows as possible.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${row_pat}=    Evaluate    tabularix.RangePattern1D([tabularix.regex(r"^Product .*$")])    modules=tabularix
    ${pat}=    Evaluate    tabularix.RangePattern2D([$row_pat.one_or_more(greedy=False)])    modules=tabularix
    ${matcher}=    Evaluate    $pat.to_matcher(outer_direction="TB", inner_direction="LR")
    ${range}=    Evaluate    $sheet.search_range($matcher, start_row=14, end_row=16, start_col=1, end_col=1)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    14    14    1    1

Verify Greedy Row Repetition Backtracking Search Matching
    [Documentation]    Verify row repetition backtracks correctly when followed by a boundary row of the same type.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${row_pat}=    Evaluate    tabularix.RangePattern1D([tabularix.regex(r"^Product .*$")])    modules=tabularix
    ${pat}=    Evaluate    tabularix.RangePattern2D([$row_pat.one_or_more(greedy=True), $row_pat])    modules=tabularix
    ${matcher}=    Evaluate    $pat.to_matcher(outer_direction="TB", inner_direction="LR")
    ${range}=    Evaluate    $sheet.search_range($matcher, start_row=14, end_row=16, start_col=1, end_col=1)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    14    16    1    1

Verify Lazy Row Repetition Backtracking Search Matching (same type)
    [Documentation]    Verify lazy row repetition backtracks correctly when followed by same type boundary.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${row_pat}=    Evaluate    tabularix.RangePattern1D([tabularix.regex(r"^Product .*$")])    modules=tabularix
    ${pat}=    Evaluate
    ...    tabularix.RangePattern2D([$row_pat.one_or_more(greedy=False), $row_pat])
    ...    modules=tabularix
    ${matcher}=    Evaluate    $pat.to_matcher(outer_direction="TB", inner_direction="LR")
    ${range}=    Evaluate    $sheet.search_range($matcher, start_row=14, end_row=16, start_col=1, end_col=1)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    14    15    1    1

Verify Lazy Row Repetition Backtracking Search Matching (different type)
    [Documentation]    Verify lazy row repetition backtracks correctly when followed by different type boundary.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${row_pat}=    Evaluate    tabularix.RangePattern1D([tabularix.regex(r"^Product .*$")])    modules=tabularix
    ${total_pat}=    Evaluate    tabularix.RangePattern1D([tabularix.value("Total")])    modules=tabularix
    ${pat}=    Evaluate
    ...    tabularix.RangePattern2D([$row_pat.one_or_more(greedy=False), $total_pat])
    ...    modules=tabularix
    ${matcher}=    Evaluate    $pat.to_matcher(outer_direction="TB", inner_direction="LR")
    ${range}=    Evaluate    $sheet.search_range($matcher, start_row=14, end_row=17, start_col=1, end_col=1)
    Should Not Be Equal    ${range}    ${None}
    Verify Range Coordinates    ${range}    14    17    1    1
