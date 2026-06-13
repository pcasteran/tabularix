*** Settings ***
Documentation       Acceptance tests for exporting sheets to SVG.

Library             OperatingSystem


*** Test Cases ***
Export Sheet To SVG
    [Documentation]    Verify exporting a sheet to an SVG file and confirming the file's existence and content.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()

    # Export to SVG
    Call Method    ${sheet}    to_svg    results/sample.svg

    # Verify file exists and has content
    File Should Exist    results/sample.svg
    File Should Not Be Empty    results/sample.svg
