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
