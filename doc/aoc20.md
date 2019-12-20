# AoC 2019 #20

Perhaps rather than working entirely in the maze map coordinates, we could
transform it into a list of shortest-distances between portals, then look
for the shortest path through them.

It looks like all the portals occur on either the inner or outer edges
of the donut, not at arbitrary places inside. 

*When on an empty square next to a portal, a single step takes you to the 
other tile with the same label.*

So we want to parse the input into, as a first pass, a map of wall/space
binaries, and a list of 2-letter labels of portals onto squares.

Actually, perhaps it's easier to read it into a 2d matrix of chars,
then parse from that - will make it a little easier to deal with finding
letters above/below each other.
