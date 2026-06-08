*** Settings ***
Documentation     A simple acceptance test to verify the Rust extension is loaded.

*** Test Cases ***
Verify Hello World From Rust
    ${result}=    Evaluate    tabularix.hello_world()    modules=tabularix
    Should Be Equal As Strings    ${result}    Hello from Tabularix Rust Core!
