## Change builder pattern to array

Test if using an array instead of a builder pattern is better for the CellGroupPattern and RangeMatcher.

## Rename matcher and pattern

Naming not consistent: RangeMatcher vs CellGroupPattern

Propositions:
  - CellGroupMatcher + OrthogonalGroupMatcher / OrthogonalMatcher
  - CellsMatcher + RangeMatcher
  - CellGroupMatcher + CellGroupListMatcher

## Cell matching direction

Currently the matching direction is vertical first (rows, top-down) and then horizontal (cell groups, left-to-right)
Is it possible to specify the matching direction:
  - CellGroupPattern: array of cell matchers (value, regex, ...) + direction (LR, RL, TB, BT)
  - RangePattern: array of CellGroupPattern + direction
    - check that all CellGroupPattern have same matching direction
    - only allow orthogonal matching direction:
      - LR / RL if CellGroupPattern are TB / BT
      - TB / BT if CellGroupPattern are LR / RL

Allow nesting CellGroupMatcher as long as they have exactly the same direction.

When calling `extract_table()`:
  - check that data and header RangeMatcher have same matching direction


Helper methods:
  - horizontal_cell_group_matcher(left_to_right = True)
  - vertical_cell_group_matcher(top_to_bottom = True)
