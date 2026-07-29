*** Settings ***
Documentation       Acceptance tests for exporting sheets to SVG.

Library             OperatingSystem


*** Test Cases ***
Export Sheet To SVG Default Zero-Based
    [Documentation]    Verify exporting a sheet to an SVG file using default zero-based indices.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    # Export to SVG (default)
    Call Method    ${sheet}    to_svg    results/sample_default.svg

    # Verify file exists and has content
    File Should Exist    results/sample_default.svg
    ${content}=    Get File    results/sample_default.svg
    # Default 0-based column header has both representation and numeric index
    Should Contain    ${content}    class="hdr-text">A (0)</text>
    # Default 0-based row header starts at 0
    Should Contain    ${content}    class="hdr-text">0</text>

Export Sheet To SVG One-Based
    [Documentation]    Verify exporting a sheet to an SVG file with one-based indices.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    # Export to SVG with zero_based_indices=False
    Evaluate    $sheet.to_svg("results/sample_one_based.svg", zero_based_indices=False)

    # Verify file exists and has content
    File Should Exist    results/sample_one_based.svg
    ${content}=    Get File    results/sample_one_based.svg
    # 1-based column header has both representation and numeric index
    Should Contain    ${content}    class="hdr-text">A (1)</text>
    # 1-based row header starts at 1, so the 0-based row header must NOT be present
    Should Not Contain    ${content}    class="hdr-text">0</text>

Export Sheet To SVG With Multi-Byte UTF-8 String
    [Documentation]    Verify multi-byte UTF-8 string SVG export.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("simple")

    # Set a cell value to a long multi-byte string
    Evaluate    $sheet.set_cell_value(0, 0, "中文测试Emojis🌟🔥🚀" * 5)

    # Export to SVG (should not panic)
    Evaluate    $sheet.to_svg("results/sample_unicode.svg")

    # Verify file exists and has content
    File Should Exist    results/sample_unicode.svg
    ${content}=    Get File    results/sample_unicode.svg
    Should Contain    ${content}    ...

Export Sheet With Formulas To SVG
    [Documentation]    Verify formula placeholders in exported SVG.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("complex")
    Evaluate    $sheet.to_svg("results/complex_formulas.svg")
    File Should Exist    results/complex_formulas.svg
    ${content}=    Get File    results/complex_formulas.svg
    Should Contain    ${content}    &lt;formula&gt;
    Should Contain    ${content}    rect-formula
    Should Contain    ${content}    val-formula

Export Sheet With Groups And Ranges To SVG
    [Documentation]    Verify semantic grouping and data-original-range properties.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("complex")
    Evaluate    $sheet.to_svg("results/complex_groups.svg")
    ${content}=    Get File    results/complex_groups.svg
    Should Contain    ${content}    data-original-range="A8"
    Should Contain    ${content}    data-original-range="B8"
    Should Contain    ${content}    <g class="data-cells">
    Should Contain    ${content}    <g class="headers">

Export Sheet With Anonymisation To SVG Checks Preserved
    [Documentation]    Verify range anonymisation preserves untargeted cells.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("complex")
    # Test single string anonymise_ranges
    Evaluate    $sheet.to_svg("results/complex_anonymised.svg", anonymise_ranges="B4:E7")
    File Should Exist    results/complex_anonymised.svg
    ${content}=    Get File    results/complex_anonymised.svg
    Should Contain    ${content}    Region
    Should Contain    ${content}    Q1
    Should Contain    ${content}    North
    Should Contain    ${content}    South

Export Sheet With Anonymisation To SVG Checks Obfuscated
    [Documentation]    Verify range anonymisation obfuscates targeted cells using list of A1 strings.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("complex")
    Evaluate    $sheet.to_svg("results/complex_anonymised_obfuscated.svg", anonymise_ranges=["B4:E7"])
    ${content}=    Get File    results/complex_anonymised_obfuscated.svg
    Should Not Contain    ${content}    12000.5
    Should Not Contain    ${content}    15000.75
    Should Not Contain    ${content}    11000
    Should Contain    ${content}    data-original-range="B4"
    Should Contain    ${content}    data-original-range="C4"
    Should Contain    ${content}    data-original-range="C5"
