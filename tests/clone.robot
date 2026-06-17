*** Settings ***
Documentation       Acceptance tests for cloning sheets via copy and deep copy protocols.

Library             Collections


*** Test Cases ***
Verify Sheet Copy Method Properties
    [Documentation]    Verify properties of a sheet cloned via the copy method.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${clone}=    Evaluate    $sheet.copy()
    ${orig_name}=    Evaluate    $sheet.name
    ${clone_name}=    Evaluate    $clone.name
    Should Be Equal As Strings    ${orig_name}    ${clone_name}
    ${orig_shape}=    Evaluate    $sheet.shape
    ${clone_shape}=    Evaluate    $clone.shape
    Should Be Equal    ${orig_shape}    ${clone_shape}

Verify Sheet Copy Method Independence
    [Documentation]    Verify mutating a sheet cloned via copy method does not affect the original.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${clone}=    Evaluate    $sheet.copy()
    Evaluate    $clone.set_cell_value(0, 0, "Mutated")
    ${orig_val_after}=    Evaluate    $sheet.get_cell_value(0, 0)
    ${clone_val_after}=    Evaluate    $clone.get_cell_value(0, 0)
    Should Be Equal As Strings    ${orig_val_after}    Header #1
    Should Be Equal As Strings    ${clone_val_after}    Mutated

Verify Python Copy Protocol Properties
    [Documentation]    Verify properties of a sheet cloned using Python's copy.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${clone}=    Evaluate    copy.copy($sheet)    modules=copy
    ${clone_name}=    Evaluate    $clone.name
    Should Be Equal As Strings    ${clone_name}    simple
    ${clone_shape}=    Evaluate    $clone.shape
    ${expected_shape}=    Evaluate    (5, 3)
    Should Be Equal    ${clone_shape}    ${expected_shape}

Verify Python Copy Protocol Independence
    [Documentation]    Verify mutating a sheet cloned via copy does not affect the original.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${clone}=    Evaluate    copy.copy($sheet)    modules=copy
    Evaluate    $clone.set_cell_value(0, 0, "MutatedCopy")
    ${orig_val}=    Evaluate    $sheet.get_cell_value(0, 0)
    ${clone_val_after}=    Evaluate    $clone.get_cell_value(0, 0)
    Should Be Equal As Strings    ${orig_val}    Header #1
    Should Be Equal As Strings    ${clone_val_after}    MutatedCopy

Verify Python Deep Copy Protocol Properties
    [Documentation]    Verify properties of a sheet cloned using Python's deepcopy.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${clone}=    Evaluate    copy.deepcopy($sheet)    modules=copy
    ${clone_name}=    Evaluate    $clone.name
    Should Be Equal As Strings    ${clone_name}    simple
    ${clone_shape}=    Evaluate    $clone.shape
    ${expected_shape}=    Evaluate    (5, 3)
    Should Be Equal    ${clone_shape}    ${expected_shape}

Verify Python Deep Copy Protocol Independence
    [Documentation]    Verify mutating a sheet cloned via deepcopy does not affect the original.
    ${wb}=    Evaluate    tabularix.load_workbook("tests/data/sample.xlsx")    modules=tabularix
    ${sheet}=    Evaluate    $wb.active_sheet()
    ${clone}=    Evaluate    copy.deepcopy($sheet)    modules=copy
    Evaluate    $clone.set_cell_value(0, 0, "MutatedDeepCopy")
    ${orig_val}=    Evaluate    $sheet.get_cell_value(0, 0)
    ${clone_val_after}=    Evaluate    $clone.get_cell_value(0, 0)
    Should Be Equal As Strings    ${orig_val}    Header #1
    Should Be Equal As Strings    ${clone_val_after}    MutatedDeepCopy
