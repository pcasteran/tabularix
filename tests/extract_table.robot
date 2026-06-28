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
    ${sheet}=    Load Mutated Multi-Row Header Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A3:C5")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:C2")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header, flatten_header=True, header_separator="::")
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['Product::Name', 'Q1::Actual', 'Q1::Forecast']

Verify Table Extraction With Nested Multi-Row Header
    [Documentation]    Test extracting table with a nested (non-flattened) multi-row header.
    ${sheet}=    Load Mutated Multi-Row Header Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A3:C5")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:C2")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header, flatten_header=False)
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['Product', 'Q1']


*** Keywords ***
Load Mutated Multi-Row Header Sheet
    [Documentation]    Loads the simple sheet and mutates its first two rows to form a multi-row header.
    ${sheet}=    Load Simple Sheet
    Evaluate    $sheet.set_cell_value(0, 0, "Product")
    Evaluate    $sheet.set_cell_value(0, 1, "Q1")
    Evaluate    $sheet.set_cell_value(0, 2, "Q1")
    Evaluate    $sheet.set_cell_value(1, 0, "Name")
    Evaluate    $sheet.set_cell_value(1, 1, "Actual")
    Evaluate    $sheet.set_cell_value(1, 2, "Forecast")
    RETURN    ${sheet}
