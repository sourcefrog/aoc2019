# AoC 2019 #18

It seems like the only reasonable strategy is to pick up keys one after the
other, and it seems there will be multiple options and the only way to know is
to explore the whole tree that arises from any order we collect the keys...

Given a map with a current position we can calculate the reachable keys.

As we take a key we can effectively remove the matching door. That opens up
two squares, previously occupied by the key and the door. That might make other
keys reachable or it might allow a shorter path to keys than was previously
known.

So it seems like we can discover all the possible orders to visit the keys,
and the length of the shortest path to them. Given one order we can generate
all the successor paths that include one more key. I'm not sure if we can 
avoid visiting any of them...

We could apply a second-order shortest-path walk that focuses attention on the
shortest known path.

We could potentially encode this as a shortest-path walk where the state space
coordinates are the (x, y) coordinate and also the list of (or set of?) keys
we've already collected.

We start at some position with no keys and we want to get - anywhere but with 
all of the keys. Hm.

...

Naively walking through the maze space, treating the keys as one of the
conditions, does find the right answer to part A, 4204. But, it's pretty
slow, at over sixty seconds for an unoptimized build...

One problem with this is that we'll repeatedly discover all the paths between
keys and doors, for all combinations of keys. Perhaps we should try a
two-phase structure where we first find the shortest paths between interesting
points (start, keys, doors), then find the best order to visit them all. 

As a meta point, getting a dumb solution to part A, so that I can see what's
in part B, might work well.
