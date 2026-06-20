# Project-Scoped Rules

- Never touch the git staging area (e.g., avoid running `git add`, `git reset`, or committing files), as the staging area is used by the user to checkpoint validated changes.
- Always use `git mv` when moving files in order to preserve file history.
