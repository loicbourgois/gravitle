# Gravitle - Moonlighter - Design


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
