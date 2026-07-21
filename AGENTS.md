# Project-Scoped Rules

- Never touch the git staging area (e.g., avoid running `git add`, `git reset`, or committing files), as the staging area is used by the user to checkpoint validated changes.
- Always use `git mv` when moving tracked files in order to preserve file history.
- Always print implementation plans and detailed summaries directly in responses instead of only linking to external/local artifacts, as the user cannot follow file links.
- Every story implementation must include updating the project documentation if relevant, as well as the tests.
- When developing a new feature or fixing a bug always verify the changes by:
    1. running `just prek` to perform static analysis of the codebase
    2. running `just unit-test` to run the unit tests
    3. running `just acceptance-test` to run the acceptance tests
