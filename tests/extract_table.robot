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
