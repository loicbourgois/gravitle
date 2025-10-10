import { kind } from "./kind.js";

const link_color_priority = {
	[kind.CORE]: 0,
	[kind.ARMOR]: 1,
	[kind.BOOSTER]: 2,
};

// m: me: player's ship
// g: ghost: player's ghosts
// o: other_ghost: shared ghost
const get_color = (cell_kind, activation, layer, player_kind) => {
	try {
		const aa = {
			[kind.ARMOR]: {
				0: {
					0: {
						m: "#aaf",
						g: "#558",
						o: "#585",
					},
				},
			},
			[kind.BOOSTER]: {
				0: {
					0: {
						m: "#fa0",
						g: "#850",
						o: "#850",
					},
				},
				1: {
					0: {
						m: "#fa0",
						g: "#850",
						o: "#850",
					},
					1: {
						m: "#f80",
						g: "#840",
						o: "#840",
					},
					2: {
						m: "#f00",
						g: "#800",
						o: "#800",
					},
				},
			},
			[kind.CORE]: {
				0: {
					0: {
						m: "#ffa",
						g: "#885",
						o: "#885",
					},
				},
			},
			[kind.ASTEROID]: {
				0: {
					0: {
						m: "#b60",
					},
				},
			},
			[kind.UNLIGHTED]: {
				0: {
					0: {
						m: "#ff93",
					},
				},
			},
			[kind.LIGHTED]: {
				0: {
					0: {
						m: "#ffad",
						g: "#0000",
						o: "#4f44",
					},
				},
			},
		};
		return aa[cell_kind][activation][layer][player_kind];
	} catch (error) {
		console.log(cell_kind, activation, layer, player_kind);
		throw error;
	}
};

export { link_color_priority, get_color };
