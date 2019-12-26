S: engineering: infinite loop
SW: hallway: escape pod (avoid)
SWW: observatory
SWWW: storage: space heater 
SWWWW: security checkpoint
SWWWWW: pressure-sensitive floor
SWWS: holodeck (end)
SWS: gift wrapping center -- antenna 
SS: arcade: photons (avoid)
SSW: science lab (NE)
SSWN: warp drive maintenance (NS) -- molten lava, sounds dangerous
SSWNN: sick bay (end) -- tambourine
SSS: crew quarters: fixed point (harmless?)
SSSW: kitchen: asterisk (harmless?)
SSSS: Passages: festive hat (harmless?)
SSSSW: Navigation
SSSSWW: Corridor: jam
SSSSWWS: Stables: easter egg
SSSSE: Hot Chocolate Fountain: giant electromagnet (avoid)

Maybe the thing about the floors is a hint that the corridor is the place we need the 
right weight? No, it's a specific room.

This is not heavy enough:
- asterisk
- antenna
- easter egg
- space heater
- jam
- festive hat
- fixed point

And this is too heavy:
- asterisk
- antenna
- easter egg
- space heater
- jam
- tambourine
- festive hat
- fixed point

Dropping one at a time:
- tambourine: too light
- asterisk: too heavy
- antenna: too heavy
- easter egg: too heavy
- space heater: too heavy
- jam: too heavy
- festive hat: too heavy
- fixed point: too heavy

OK so it seems we have to hold the tambourine and drop at least 2 of the other things. 

Actually let's make sure we have all the things. Seems like it.

dropping:
ast, ant - too heavy
ast, ant, egg - too heavy
ast, ant, egg, heater - too heavy
ast, ant, egg, heater, jam - too light
ast, ant, egg, jam - too light
ast, ant, jam - too heavy

these combinations are too heavy: no point trying any more:
- ast, tam
- ant, tam
- egg, heater, tam, hat, fix
    - ant, egg, heater, tam, hat, fix
    - ast, ant, egg, heater, tam, hat, fix
- egg, heater, tam, hat, fix
- jam, tam
    - heater, jam, tam
    - heater, jam, tam, hat
    - egg, heater, jam, tam, hat

these combinations are too light: no point trying any less:
- heater, tam, hat, fix
- egg, tam, hat, fix
- ast, ant, egg, heater, jam, hat, fix
- egg, heater, tam, hat
- jam
- tam

This seems to mean the jam and tambourine are among the heavier objects.
Either of them on their own is too light, but both together are too heavy.
So we need exactly one of them plus some other objects. 

Taking everything but the tambourine is not heavy enough.

Therefore, we must take the tambourine and must not take the jam.

Can we apply similar logic again to narrow things down?

Carrying the tambourine, is there any one object that will put us over weight, 
that we could eliminate? We know the heater, egg, hat, and fix are not such.

Yes, the asterisk and tambourine together are too heavy, and we know we need the 
tambourine. So, no asterisk.

Also the antenna and tambourine together are too heavy. So no antenna.

Correct answer is egg, heater, tam, fix.