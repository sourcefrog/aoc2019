# AoC 2019-19

## Part B

A naive way to approach this would be to draw the whole map and look for a
square, but that could get very large and take a long time.

Here's another approach: pick a Y value. Calculate the X range of the beam on
that row. See if there is a square of the desired size (100) there. Bisect to
find Y, and the upper-left corner within that Y.
