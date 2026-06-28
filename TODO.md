update doc

## High-Level API

Helper methods:

- horizontal_cell_group_matcher(left_to_right = True)
- vertical_cell_group_matcher(top_to_bottom = True)

...

TwoDimensionalRangeMatcher: outer_direction, inner_direction, [OneDimensionalRangePattern]

sheet.search_range(OneDimensionalRangeMatcher|TwoDimensionalRangeMatcher)

- internal matching optimized according to the direction as we did
