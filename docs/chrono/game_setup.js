function game_setup() {
	const kinds = {
		armor: this.gravitle.Kind.Armor,
		booster: this.gravitle.Kind.Booster,
		core: this.gravitle.Kind.Core,
	};
	for (let idx = 0; idx < this.worlds.length; idx++) {
		const world = this.worlds[idx];
		const ghost = this.ghosts[idx - 1];
		let user_kind = 1;
		if (ghost && ghost.kind == "me") {
			user_kind = 2;
		}
		if (ghost && ghost.kind == "other") {
			user_kind = 3;
		}
		// add ship
		for (const e of this.ship.parts) {
			world.add_cell(e.p.x, e.p.y, e.d, kinds[e.kind], user_kind);
		}
		for (const l of this.ship.links) {
			world.add_link(l.a, l.b);
		}
		world.setup(this.seed, this.asteroid_count, this.stars_count, user_kind);
	}
}
export { game_setup };
