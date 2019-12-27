# AoC 2019 # 17

So we apparently need to first work out a (preferably minimal) path
that visits every path of the scaffold at least once. Then, reduce that
to a set of steps and turns, and then compress that into the two-level
program.

Possibly, the choice of paths can make the program simpler, but there
are not a lot of crossings where we have a choice where to go - I 
guess it's actually the number of intersections from part A.

How many turns do we actually have to take? Maybe a really simple 
approach is enough? 35 turns. And each function can be at most 20 characters, 
so it's tight. A turn and a 2-digit move is `L,xx,` 5 characters, so we only
get 4 of them per function. And the main routine can invoke 10 subroutines, so 
the routines apparently need to make an average of 3 turns each, and probably
no more than 5 or 6 turns each.

We could use A for a turn left, B for a turn right, and C for some kind of
move, although that's also not going to be enough because we can only call
10 subroutines. So we know the routines must make multiple turns.

At the intersections we can choose a path that facilitates reuse.

It's perhaps unlikely to be useful but we can cross passages twice or
even spin around and go back the way we came.

We could "cheat" by actually not walking it all in one go, but rather
starting multiple runs through the program that discover different
parts of the map and the robots located at them, and then add them
up. That's assuming all the robots to be cleaned remain still, which
it seems they would. But this shouldn't really be necessary, as the 
stated problem is to visit everything.

With 10 subroutine calls each of which can make 5 turns we should have
enough space to actually do this.

The map is:

```
........................................#############..
........................................#...........#..
........#######.........................#...........#..
........#.....#.........................#...........#..
........#.....#.#############...........#...........#..
........#.....#.#...........#...........#...........#..
........#.....#.#...........#...........#...........#..
........#.....#.#...........#...........#...........#..
........#############.......#...........#...........#..
..............#.#...#.......#...........#...........#..
..............#.#.###########.....#######...........#..
..............#.#.#.#.............#.................#..
..............###########.........#.......###########..
................#.#.#...#.........#.......#............
................#.#.#...#.........#.......#............
................#.#.#...#.........#.......#............
................##########^.......#.......#............
..................#.#...#.........#.......#............
..........###########...#.........#.......#............
..........#.......#.....#.........#.......#............
..........#.......###########.....#.......#............
..........#.............#...#.....#.......#............
......###########.......###########.......#############
......#...#.....#...........#.........................#
......#...#.....#...........#.........................#
......#...#.....#...........#.........................#
......#...#.....#...........#.........................#
......#...#.....#...........#.........................#
###########.....#...........#.........................#
#.....#.........#...........#..........................
#.....#.........#...........#..........................
#.....#.........#...........#..........................
#.....#.........#############..........................
#.....#................................................
#######................................................
```

What are the path lengths between corners? We have some 
options. 

- L then 2 or 6 or 8 or 10
- 

How about just the long paths? In fact the last section
is most constrained as it has no intersections. Starting from
the corner after the last intersection and going through the
right side of the graph.

- L, 12, R, 6, L, 10, R, 12 R, 12, R, 10, L, 10, L, 12, R, 6

I start to see some common sequences:

- `12,R` or `R,12` or maybe `6,R,6`

Other thoughts:

- Of course a subroutine could stop in the middle of 
  a path - need not start or end with a turn.

- It's possible we can make turns that cancel each 
  other, or do a 270 degree turn, but the program space
  is so constrained that this seems unlikely.

Perhaps rather than trying to work it out by hand I should 
write a program that discovers possible paths? I can imagine
starting by either enumerating all programs and trying to see
if they stay on the path, and cover the whole thing? Or,
enumerate possible paths (of which there are several because of the 
intersections) and then try to compress out common sequences. Both seem
complicated...

Given a path we'd want a routine that finds the longest substring that
occurs more than once. Which is possible. (Although, not exactly the longest, as we
need to balance accounting for the whole path in three subroutines with 
all the size constraints.)

A naive greedy approach would be to start filling A, then B, then see what we 
can reuse and adjust:

   01234567890123456789
A: L,10,R,12,R,12,R,6,R
B: 10,L,10,L,10,R,12,R
C: 12,R,10,L,10,L,12,R

(This brings us to the right of the bottom line, having made 13 of the ~35 necessary
turns.)

This path altogether is 

    L,10,R,12,R,12,R,6,R,10,L,10,L,10,R,12,R,12,R,10,L,10,L,12,R,6,R,6,R,10,L,10,R,10,L,10,L,12,R,6,R,6,R,10,L,10,R,10,L,10,
    AAAAAAAAAAAAAA BBBBBBBBBBBBB AAAAAAAAAAAAAA CCCCCCCCCCCCCCCCCC BBBBBBBBBBBBB CCCCCCCCCCCCCCCCCC BBBBBBBBBBBBB CCCCCCCCCC        


So I wonder if I could make the two paths at the start fit better together by taking different choices?

we can see

   01234567890123456789
A: L,10,R,12,R,12,R
B: 6,R,10,L,10
C: 10,L,10,

A longer path

    L,10,R,12,R,12,R,6,R,10,L,10,L10,R,12,R,12,R,10,L,10,L,12,R

Given a maze can we enumerate all possible paths through all intersections? 
Assuming we want to traverse every square only once, yes, that seems easy.

Given a path through the maze could we algorithmically break it into A,B,C?
Surely yes by brute force enumeration, and the number of possibilities is probably
not too high.

One way is to say: the start of the path must be A. Consider all prefixes of it as
A, that encode up to length 20. Only As that occur at least one more time in the
path are interesting. Perhaps allow for breaking in the middle of a path. 
Then, B must encode the next part after the first A, and probably must also recur
at least once. Then C must account for what remains.