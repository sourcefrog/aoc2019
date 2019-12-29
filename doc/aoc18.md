# AoC 2019 #18


## Part A

The problem could be stated as: enumerate all possible orders to visit the keys,
and select the smallest.

In principle there are a completely infeasible $26!$ orders we could traverse 
them, but probably/hopefully for these mazes there are many fewer orders that
are actually reachable.

If we know it takes distance D to traverse *abcdef* then we can look at each 
key reachable from *f* given those keys, and those constitute the next set
of paths.

In fact we want to do this only once for the minimum distance across every
permutation of *abcdef* ending in *f*. And having found the new longer 
paths, we don't need to remember this any more.

How can we make sure to compute it only once? One answer: compute all paths
of length *N*, then collapse them into minimal sets ending at each point.
Then that's the next generation: compute all of that and then collapse it.
Continue until you reach the desired number of keys. 

One additional small optimization: search for a path can end once we
know we've found all the remaining keys.

## Part B

Similarly to in part A, we just ("just") need to enumerate all possible orders
to pick up the keys, of course discarding any that become infeasible. The
complication is that there are multiple robots moving and when we look for a new
move, we need to look from each of the robots.