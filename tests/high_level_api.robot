*** Settings ***
Documentation       Acceptance tests for Tabularix high-level API functions.

Library             Collections
Resource            common.resource


*** Test Cases ***
Verify Vertical Table Extraction Header And Data
    [Documentation]    Test extract_table_with_header_and_data on a vertical table.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("complex")
    ${header}=    Evaluate    tabularix.group(tabularix.value("Region"), tabularix.regex(r"^Q[1-4]$").repeat(4, max=4))    modules=tabularix
    ${data}=    Evaluate    tabularix.grid(tabularix.group(tabularix.regex(r"^(?!Total).*$"), tabularix.non_empty().repeat(4, max=4)).one_or_more())    modules=tabularix
    ${table}=    Evaluate    tabularix.extract_table_with_header_and_data($sheet, $header, $data)    modules=tabularix
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['region', 'q1', 'q2', 'q3', 'q4']
    ${shape}=    Evaluate    $table.shape
    Should Be Equal As Strings    ${shape}    (4, 5)

Verify Vertical Table Extraction Between Header and Footer
    [Documentation]    Test extract_table_between_header_and_footer on a vertical table.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("complex")
    ${header}=    Evaluate    tabularix.group(tabularix.value("Region"), tabularix.regex(r"^Q[1-4]$").repeat(4, max=4))    modules=tabularix
    ${footer}=    Evaluate    tabularix.group(tabularix.regex(r"^Total \\d{4}$"), tabularix.any().repeat(4, max=4))    modules=tabularix
    ${table}=    Evaluate    tabularix.extract_table_between_header_and_footer($sheet, $header, $footer)    modules=tabularix
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['region', 'q1', 'q2', 'q3', 'q4']
    ${shape}=    Evaluate    $table.shape
    Should Be Equal As Strings    ${shape}    (4, 5)

Verify Horizontal Table Extraction Header And Data
    [Documentation]    Test extract_table_with_header_and_data on a horizontal table.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.get_sheet("multi-tables")
    ${header}=    Evaluate    tabularix.group(tabularix.regex(r"^(Date|Fiscal Year)$").repeat(2, 2))    modules=tabularix
    ${data}=    Evaluate    tabularix.group(tabularix.non_empty().repeat(2, 2))    modules=tabularix
    ${table}=    Evaluate    tabularix.extract_table_with_header_and_data($sheet, $header, $data, main_direction="LR", inner_direction="TB")    modules=tabularix
    ${cols}=    Evaluate    $table.columns
    Should Be Equal As Strings    ${cols}    ['date', 'fiscal_year']
    ${shape}=    Evaluate    $table.shape
    Should Be Equal As Strings    ${shape}    (1, 2)
