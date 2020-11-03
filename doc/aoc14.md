# AoC 2019 #14

## Part B

How much fuel could we produce with a trillion (1e12) units of ore?

One way to get there would be to work out the ratios, maybe as floating point
numbers, of how much ore is ultimately used to generate one unit of fuel, then
divide one trillion by that number. I'm concerned this'll either floating point
imprecision(?) or disregard that even with that large number, some amount of ore
will be left over at the end.

The solution to part A was, 178154 ORE produces 1 FUEL, probably with some
waste. So we should be able to produce at least 5_613_121 FUEL from 1e12 ORE,
but probably rather better.

Another way would be to proceed iteratively: make that much fuel. See what
ingredients are left over.

...

Another way would be to decompose it like this:

- guess a number of FUEL that can be produced
- work out whether we have enough ingredients to produce that many FUEL
- iterate, perhaps by bisection, until we find the maximum possible FUEL

Can we do the second part: work out the amount of ORE required to produce a
given amount of FUEL, or alternatively whether it is _possible_ to produce the
desired FUEL from a fixed quantity of ORE.

As we work down we could have a set of _needed_ quantities, and a set of
_present_ quantities.

We know how many ingredients we need for the final step, so we can
