# AoC 2019 #21

The furthest ahead we can possibly see is four squares; if we get into trouble
after that I don't see what can be done.

How far does it actually jump? It jumps four squares. So if the fourth square
(D) is a hole, we certainly should not jump. And if the next square (A) is a
hole, we certainly should jump. 

Using x as don't-know

   .xxx => jump
   xxx. => don't jump
   
Of course there is a case where both A and D are holes and then we seem to be
doomed; we must avoid this case by jumping earlier to land on the island in
the middle. (There must be an island or it's just impossible.) So we need to
recognize

    .... => doomed
    ...# => jump
    ..#. => doomed
    ..## => jump
    .#.. => doomed
    .#.# => jump
    .##. => doomed
    .### => jump
    #... => don't jump
    #..# => jump, because the next D might be empty
    #.#. => don't jump
    #.## => jump??
    ##.. => don't jump
    ##.# => jump??
    ###. => don't jump
    #### => don't jump

If A is empty, you need to jump, or you'll fall.
If D is empty, don't jump, or you'll fall into it.

    jump = D && (!A || !B || !C)
    jump = D && !(A && B && C)

I wonder if we actually should solve this by reasoning about it, or by making
the computer discover a working program. But there are up to sixteen
instructions, and many possibilities for each, so it seems infeasible to do it
by brute force...

So: jump if D is floor and there are any gaps. In other words jump eagerly
if you know you'll need to jump and if you won't die by jumping now.

## Part B

Example:
      @ABCDEFGHI
    #####.#.##...####

We need to not jump too early and land on the single square, as then we're
doomed. So it seems like the condition should be extended to, also don't
jump unless there's either a square after D, (E is set), or you can jump
again from D, (H is set.) At least it would work in this example.

    jump = D && !(A && B && C) && (E || H)
