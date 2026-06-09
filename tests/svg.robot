*** Settings ***
Documentation     Acceptance tests for exporting sheets to SVG.
Library           OperatingSystem

*** Test Cases ***
Export Sheet To SVG
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    
    # Export to SVG
    Call Method    ${sheet}    to_svg    target/robot/sample.svg
    
    # Verify file exists and has content
    File Should Exist    target/robot/sample.svg
    File Should Not Be Empty    target/robot/sample.svg
