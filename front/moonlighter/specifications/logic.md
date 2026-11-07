# Gravitle - Moonlighter - Specifications - Logic


## Filter
Filter is needed to protect Pump.
Without filter, moon dust would go into the pump, clogging it.
We want the player to use a filter in front of the pump.


## Transfer
Any transfer between 2 directly connected particles happens at a maximum of 1 unit/tick.
To transfer more, we need more parallel lines.
It is OK to transfer out more than 1 unit per particle per tick,
if that particle has multiple connections going out.


## Solar panel
A solar panel cannot produce and transfer a unit of electricity at the same time.
The gradual flow also exists outside the boundary of the system.
sun -> solar panel -> wire


## Misc
// Electricity
//
// Solar Panel
// A panal generates 1 electricity per tick
//
// We only distribute by increment of 1 by step
// We want the electricity to flow gradually.
// No exception.
//
// Electricty should flow equally to everything
// (10,1) -> (9,2)
// we must first equalize the network, then push the overflow to the biggest
// (10/100,2/2,0/2) -> (10/100,2/2,1/2) -> (10/100,2/2,2/2) -> (11/100,2/2,2/2)