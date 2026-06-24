*** Settings ***
Documentation       Acceptance tests for Table exports (to_arrow, to_pandas, to_polars).

Library             Collections
Resource            common.resource


*** Test Cases ***
Verify Export to PyArrow Table
    [Documentation]    Test exporting the custom Table to a PyArrow Table.
    ${sheet}=    Load Simple Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A2:C3")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:C1")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header, clean_names=True)
    ${arrow_table}=    Evaluate    $table.to_arrow()
    ${shape}=    Evaluate    ($arrow_table.num_rows, $arrow_table.num_columns)
    Should Be Equal As Strings    ${shape}    (2, 3)
    ${schema_names}=    Evaluate    $arrow_table.schema.names
    Should Be Equal As Strings    ${schema_names}    ['header_1', 'header_2', 'header_3']

Verify Export to Pandas DataFrame
    [Documentation]    Test exporting the custom Table to a Pandas DataFrame via Arrow.
    ${sheet}=    Load Simple Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A2:C3")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:C1")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header, clean_names=True)
    ${arrow_table}=    Evaluate    $table.to_arrow()
    ${df}=    Evaluate    $arrow_table.to_pandas()
    ${shape}=    Evaluate    tuple($df.shape)
    Should Be Equal As Strings    ${shape}    (2, 3)
    ${cols}=    Evaluate    list($df.columns)
    Should Be Equal As Strings    ${cols}    ['header_1', 'header_2', 'header_3']

Verify Export to Polars DataFrame
    [Documentation]    Test exporting the custom Table to a Polars DataFrame via Arrow.
    ${sheet}=    Load Simple Sheet
    ${data}=    Evaluate    tabularix.Range.from_a1("A2:C3")    modules=tabularix
    ${header}=    Evaluate    tabularix.Range.from_a1("A1:C1")    modules=tabularix
    ${table}=    Evaluate    $sheet.extract_table($data, $header, clean_names=True)
    ${arrow_table}=    Evaluate    $table.to_arrow()
    ${df}=    Evaluate    polars.from_arrow($arrow_table)    modules=polars
    ${shape}=    Evaluate    tuple($df.shape)
    Should Be Equal As Strings    ${shape}    (2, 3)
    ${cols}=    Evaluate    $df.columns
    Should Be Equal As Strings    ${cols}    ['header_1', 'header_2', 'header_3']

Verify Export Date Column to PyArrow
    [Documentation]    Test that exporting a table with date cells to PyArrow maps them to a date32 array.
    ${sheet}=    Load Complex Sheet
    Verify Date Column In Table    ${sheet}


*** Keywords ***
Verify Date Column In Table
    [Documentation]    Extract employee table and assert start_date column matches date32 schema in PyArrow.
    [Arguments]    ${sheet}
    ${expr}=    Catenate    SEPARATOR=\n
    ...    (lambda s: [
    ...    table := s.extract_table(
    ...    tabularix.Range.from_a1("A11:D12"),
    ...    tabularix.Range.from_a1("A10:D10"),
    ...    clean_names=True
    ...    ),
    ...    at := table.to_arrow(),
    ...    at.schema.names == ['name', 'role', 'active', 'start_date'] and
    ...    str(at.schema.field('start_date').type) == 'date32[day]' and
    ...    type(at.column('start_date')[0].as_py()) is datetime.date and
    ...    str(at.column('start_date')[0].as_py()) == '2023-01-15'
    ...    ][-1])($sheet)
    ${success}=    Evaluate    ${expr}    modules=tabularix,datetime
    Should Be True    ${success}
