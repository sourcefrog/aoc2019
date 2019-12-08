# Day 3

I can think of two ways to track where the wires go:

1.  Keep a bitmap of occupied spaces and fill in individual squares as we
    follow their paths.

    This might use a lot of space and require tracking an array of unknown size
    but it seems simple to implement: as we visit new squares simply see if
    they've been visited by the other side.

1.  Remember the coordinates of horizontal and vertical segments.

    On the second wire, look for intersections.

    This doesn't require doing per-square work or having an array of maximum
    size, but it does make finding intersections a little more complicated.
    Probably not very complicated, though.

I ended up doing the second, which is pretty simple and works well for part A,
at least.