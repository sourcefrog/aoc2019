# AoC 2019 #14

## Part B

How much fuel could we produce with a trillion (1e12) units of ore?

One way to get there would be to work out the ratios, maybe as floating
point numbers, of how much ore is ultimately used to generate one unit of 
fuel, then divide one trillion by that number. I'm concerned this'll either 
floating point imprecision(?) or disregard that even with that large 
number, some amount of ore will be left over at the end.

The solution to part A was, 178154 ORE produces 1 FUEL, probably with some
waste. So we should be able to produce at least 5_613_121 FUEL from 1e12 
ORE, but probably rather better.

Another way would be to proceed iteratively: make that much fuel. See what
ingredients are left over. 