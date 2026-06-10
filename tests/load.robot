*** Settings ***
Documentation     Acceptance tests for loading Excel workbooks and sheets.
Library           Collections

*** Test Cases ***
Load Workbook And Inspect Sheets
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix

    # Check sheet names
    ${names}=    Evaluate    $wb.sheet_names()
    ${expected_names}=    Create List    Sheet1
    Lists Should Be Equal    ${names}    ${expected_names}

    # Get active sheet and check metadata
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${name}=    Evaluate    $sheet.name
    Should Be Equal As Strings    ${name}    Sheet1

    ${shape}=    Evaluate    $sheet.shape
    ${expected_shape}=    Evaluate    (5, 2)
    Should Be Equal    ${shape}    ${expected_shape}

    # Check cell values
    ${val1}=    Evaluate    $sheet.cell(0, 0)
    Should Be Equal As Strings    ${val1}    Header1

    ${val2}=    Evaluate    $sheet.cell(1, 1)
    ${expected_val2}=    Evaluate    123.45
    Should Be Equal    ${val2}    ${expected_val2}

    # Check out of bounds error
    Run Keyword And Expect Error    *IndexError: Out of bounds*    Evaluate    $sheet.cell(5, 0)
