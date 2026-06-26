# Project-Scoped Rules

- Never touch the git staging area (e.g., avoid running `git add`, `git reset`, or committing files), as the staging area is used by the user to checkpoint validated changes.
- Always use `git mv` when moving tracked files in order to preserve file history.
- Always print implementation plans and detailed summaries directly in responses instead of only linking to external/local artifacts, as the user cannot follow file links.
- When running the acceptance tests always use the command `just acceptance-test`.
