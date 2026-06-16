*** Settings ***
Documentation       Acceptance tests for exporting sheets to SVG.

Library             OperatingSystem


*** Test Cases ***
Export Sheet To SVG Default Zero-Based
    [Documentation]    Verify exporting a sheet to an SVG file using default zero-based indices.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()

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
    ${sheet}=    Evaluate    $wb.active_sheet()

    # Export to SVG with zero_based_indices=False
    Evaluate    $sheet.to_svg("results/sample_one_based.svg", zero_based_indices=False)

    # Verify file exists and has content
    File Should Exist    results/sample_one_based.svg
    ${content}=    Get File    results/sample_one_based.svg
    # 1-based column header has both representation and numeric index
    Should Contain    ${content}    class="hdr-text">A (1)</text>
    # 1-based row header starts at 1, so the 0-based row header must NOT be present
    Should Not Contain    ${content}    class="hdr-text">0</text>
