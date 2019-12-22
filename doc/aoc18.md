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
