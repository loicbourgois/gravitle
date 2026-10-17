import { kind } from "./kind.js";
import { colors_generated } from "./colors_generated.js";

const link_color_priority = {
	[kind.CORE]: 0,
	[kind.ARMOR]: 1,
	[kind.BOOSTER]: 2,
};

// m: me: player's ship
// g: ghost: player's ghosts
// o: other_ghost: shared ghost
const get_color = (particle_kind, activation, layer, player_kind) => {
	try {
		return colors_generated[particle_kind][activation][layer][player_kind];
	} catch (error) {
		console.log(particle_kind, activation, layer, player_kind);
		throw error;
	}
};

export { link_color_priority, get_color };
