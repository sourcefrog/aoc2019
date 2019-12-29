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

...

This solution to part A works, but is slow. Part B is about solving several 
mazes with doors coupled together.

From any point we can flood-fill out to find all the keys you can directly 
reach from that point, and their distance, given a set of currently-locked
doors. And for that matter also all the doors you can reach.

It seems it's only interesting to stop a move at a key, and so we're only
interested in the shortest path from either the start, or one key, to the
next key. But they may change depending on what other doors were already
opened.

The current code seems too slow for what it does... Perhaps because it's
not really correctly keeping track of the best options found so far.

Here's another way to put the simplification: we can measure the shortest paths
between A and B, where A and B are each either the starting point, a key, or a
door. The path only exists where there is a direct unobstructed path between
them, with no keys or doors in the way. These distances won't change as we open
doors. 

Then we want to find an order to walk through the keys or gates, starting at 
the start, with the constraint that we can only pass a gate after we previously
took its keys. 

## Take 3

OK, new, hopefully simpler and more successful approach:

Make a function that, from point *p*, given existing keys *ks*, returns
every new key we can pick up, and the distance to it. Just do this by walking
over the map.

Then walk down through the tree that represents every available order in which 
we can take keys. Whichever path is shortest is best.