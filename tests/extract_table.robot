*** Settings ***
Documentation       Acceptance tests for Sheet.extract_table and Table.

Library             Collections
Resource            common.resource


*** Test Cases ***
Verify Table Extraction Without Header
    [Documentation]    Test extracting table coordinates and getting default column names.
    ${sheet}=    Load Simple Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A2:C3")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data)
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['column_1', 'column_2', 'column_3']
    ${shape}=    Evaluate    $table.shape
    Should Be Equal As Strings    ${shape}    (2, 3)

Verify Table Extraction With Header
    [Documentation]    Test extracting table with a single-row header.
    ${sheet}=    Load Simple Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A2:C3")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:C1")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header)
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['Header #1', 'Header #2', 'Header #3']

Verify Table Extraction With Clean Names
    [Documentation]    Test extracting table with header name cleaning enabled.
    ${sheet}=    Load Simple Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A2:C3")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:C1")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header, clean_names=True)
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['header_1', 'header_2', 'header_3']

Verify Mismatching Column Count Error
    [Documentation]    Test that non-aligned header and data ranges raise ValueError.
    ${sheet}=    Load Simple Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A2:C3")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:B1")    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.extract_table($data, $header)

Verify Overlapping Ranges Error
    [Documentation]    Test that overlapping header and data ranges raise ValueError.
    ${sheet}=    Load Simple Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A2:C3")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:C2")    modules=tabularix
    Run Keyword And Expect Error    *ValueError*    Evaluate    $sheet.extract_table($data, $header)

Verify Slicing Out of Bounds Error
    [Documentation]    Test that range coordinates exceeding sheet dimensions raise IndexError.
    ${sheet}=    Load Simple Sheet
    ${bad_data}=    Evaluate    tabularix.Range.from_a1("A2:C11")    modules=tabularix
    Run Keyword And Expect Error    *IndexError*    Evaluate    $sheet.extract_table($bad_data)

Verify Table Extraction With Flattened Multi-Row Header
    [Documentation]    Test extracting table with a multi-row header flattened.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${data}=    Evaluate    tabularix.Range.from_a1("B15:F17")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("B13:F14")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header, flatten_header=True, header_separator="::")
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings
    ...    ${cols}
    ...    ['Product', '2025::Expected', '2025::Actual', '2026::Expected', '2026::Actual']

Verify Table Extraction With Nested Multi-Row Header
    [Documentation]    Test extracting table with a nested (non-flattened) multi-row header.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${data}=    Evaluate    tabularix.Range.from_a1("B15:F17")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("B13:F14")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header, flatten_header=False)
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['Product', '2025', '2026']

Verify Table Extraction Horizontal Alignment
    [Documentation]    Test extracting a horizontal table where headers are on the left and data is on the right.
    ${sheet}=    Load Mutated Horizontal Table Sheet
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:A2")    modules=tabularix
    ${data}=    Evaluate    tabularix.Range.from_a1("B1:B2")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header)
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['Date', 'Fiscal Year']
    ${shape}=    Evaluate    $table.shape
    Should Be Equal As Strings    ${shape}    (1, 2)
    Assert Extracted Horizontal Table Values    ${table}

Verify Table Extraction Horizontal Right Alignment
    [Documentation]    Test extracting a horizontal table where headers are on the right and data is on the left.
    ${sheet}=    Load Mutated Horizontal Right Table Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("B1:B2")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("C1:C2")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header)
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['Date', 'Fiscal Year']
    ${shape}=    Evaluate    $table.shape
    Should Be Equal As Strings    ${shape}    (1, 2)
    Assert Extracted Horizontal Table Values    ${table}

Verify Table Extraction Vertical Bottom Alignment
    [Documentation]    Test extracting a vertical table where headers are on the bottom and data is on the top.
    ${sheet}=    Load Mutated Vertical Bottom Table Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A1:B1")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A2:B2")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header)
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['Date', 'Fiscal Year']
    ${shape}=    Evaluate    $table.shape
    Should Be Equal As Strings    ${shape}    (1, 2)
    Assert Extracted Horizontal Table Values    ${table}


*** Keywords ***
Load Mutated Horizontal Table Sheet
    [Documentation]    Loads the simple sheet and mutates it for horizontal table extraction.
    ${sheet}=    Load Simple Sheet
    Evaluate    $sheet.set_cell_value(0, 0, "Date")
    Evaluate    $sheet.set_cell_value(0, 1, "2026-06-23")
    Evaluate    $sheet.set_cell_value(1, 0, "Fiscal Year")
    Evaluate    $sheet.set_cell_value(1, 1, "2025-2026")
    RETURN    ${sheet}

Load Mutated Horizontal Right Table Sheet
    [Documentation]    Loads the simple sheet and mutates it for horizontal right table extraction.
    ${sheet}=    Load Simple Sheet
    Evaluate    $sheet.set_cell_value(0, 1, "2026-06-23")
    Evaluate    $sheet.set_cell_value(0, 2, "Date")
    Evaluate    $sheet.set_cell_value(1, 1, "2025-2026")
    Evaluate    $sheet.set_cell_value(1, 2, "Fiscal Year")
    RETURN    ${sheet}

Load Mutated Vertical Bottom Table Sheet
    [Documentation]    Loads the simple sheet and mutates it for vertical bottom table extraction.
    ${sheet}=    Load Simple Sheet
    Evaluate    $sheet.set_cell_value(0, 0, "2026-06-23")
    Evaluate    $sheet.set_cell_value(0, 1, "2025-2026")
    Evaluate    $sheet.set_cell_value(1, 0, "Date")
    Evaluate    $sheet.set_cell_value(1, 1, "Fiscal Year")
    RETURN    ${sheet}

Assert Extracted Horizontal Table Values
    [Documentation]    Asserts the cell values in the extracted horizontal table.
    [Arguments]    ${table}
    ${pydict}=    Evaluate    $table.to_arrow().to_pydict()
    ${date_val}=    Get From Dictionary    ${pydict}    Date
    ${fiscal_val}=    Get From Dictionary    ${pydict}    Fiscal Year
    ${date_str}=    Evaluate    str($date_val[0])
    Should Be Equal As Strings    ${date_str}    2026-06-23
    Should Be Equal As Strings    ${fiscal_val}    ['2025-2026']
