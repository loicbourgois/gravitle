
import { cyrb128, sfc32 } from "./random.js";
import { ship } from "./ship.js";
import { get_cell } from "./get_cell.js";
import { wrap_around } from "./math.js";
import { draw_cells, draw_ship_only } from "./draw_cells.js";


const kinds = {
	armor: 0,
	booster: 1,
	core: 2,
};

const kb = {
	s: [],
	d: [],
};

function Game({
    gravitle, 
    memory, 
    seed_input,
    stars_count,
    ghost,
    view,
}) {
	this.gravitle = gravitle;
    this.memory = memory;
    this.view = view;
    this.stars_count = stars_count;
    this.ghost = ghost;
    this.ghosts = []
    this.seed = cyrb128(seed_input);
    this.last_frame = null
    ship.parts.forEach((e, idx) => {
        if (e.binding) {
            kb[e.binding].push(idx);
        }
    });
    this.key_bindings = new Map();
    for (let k of Object.keys(kb)) {
		if (!this.key_bindings.has(k)) {
			this.key_bindings.set(k, new Set());
		}
		for (let idx of kb[k]) {
			this.key_bindings.get(k).add(idx);
		}
	}
    document.addEventListener("keydown", (e) => {
        if (this.key_bindings.get(e.key)) {
            for (let idx of this.key_bindings.get(e.key)) {
                this.worlds[0].set_cell_activated(idx, 1);
            }
        }
        if (e.key == "e") {
            this.try_again()
        }
        document.querySelectorAll(".disappearable").forEach((x, i) => {
            x.classList.remove("disappear");
        });
    });
    document.addEventListener("keyup", (e) => {
        if (this.key_bindings.get(e.key)) {
            for (let idx of this.key_bindings.get(e.key)) {
                this.worlds[0].set_cell_activated(idx, 0);
            }
        }
    });
    this.restart()
    this.tick()
}


Game.prototype.tick = function() {
	const now = performance.now();
	const elapsed = now - this.last_frame;
	let steps = 2;
	if (elapsed > 15 && this.last_frame != null) {
		steps = 4;
	}
	this.last_frame = now;
	for (let index = 0; index < steps; index++) {
        for (let i = 1; i < this.worlds.length; i++) {
            const world = this.worlds[i]
            const ghost = this.ghosts[i-1]
            const es = ghost[world.step];
            if (es != null) {
                // console.log(es)
				for (const e of es) {
					world.switch_cell_activated(e.c);
					// ghost_cells.add(e.c);
				}
				// delete ghost_json[worlds[1].step];
			}
        }
        for (const world of this.worlds) {
            world.run_step();
        }
	}
	this.view.set_backgound("#102");
    for (let i = 1; i < this.worlds.length; i++) {
        draw_ship_only(this.gravitle, this.worlds[i], this.memory, this.view);
    }
	draw_cells(this.gravitle, this.worlds[0], this.memory, this.view);
	requestAnimationFrame(() => {
        this.tick()
	});
}


Game.prototype.restart = function() {
    this.worlds = [this.gravitle.World.new()];
    for (const _ghost of this.ghosts) {
        this.worlds.push(this.gravitle.World.new())
    }
    for (const world of this.worlds) {
		ship.parts.forEach((e, idx) => {
			world.add_cell(e.p.x - 0.3, e.p.y - 0.3, e.d, kinds[e.kind]);
		});
		const rand = sfc32(this.seed[0], this.seed[1], this.seed[2], this.seed[3]);
		for (let index = 0; index < 20; index++) {
			const diameter = 0.05;
			let iterations = 0;
			while (true) {
				iterations += 1;
				if (iterations > 100) {
					throw "too many iterations";
				}
				const x = rand();
				const y = rand();
				const cells_ptr = world.cells();
				const cell_size = this.gravitle.Cell.size();
				const cells_view = new DataView(
					this.memory.buffer,
					cells_ptr,
					world.cells_count() * cell_size,
				);
				let ok = true;
				for (let i = 0; i < world.cells_count(); i++) {
					const cell = get_cell(cells_view, cell_size, i);
					const wa = wrap_around(cell.p, { x, y });
					let diams = cell.diameter + diameter * 1.5;
					let colliding = wa.d_sqrd < diams * diams;
					if (colliding) {
						ok = false;
						break;
					}
				}
				if (ok) {
					world.add_cell(x, y, diameter, 4);
					break;
				}
			}
		}
		for (let index = 0; index < this.stars_count; index++) {
			const diameter = 0.015;
			let iterations = 0;
			while (true) {
				iterations += 1;
				if (iterations > 200) {
					throw "too many iterations";
				}
				const x = rand();
				const y = rand();
				const cells_ptr = world.cells();
				const cell_size = this.gravitle.Cell.size();
				const cells_view = new DataView(
					this.memory.buffer,
					cells_ptr,
					world.cells_count() * cell_size,
				);
				let ok = true;
				for (let i = 0; i < world.cells_count(); i++) {
					const cell = get_cell(cells_view, cell_size, i);
					const wa = wrap_around(cell.p, { x: x, y: y });
					let diams = cell.diameter * 0.5 + diameter * 5;
					let colliding = wa.d_sqrd < diams * diams;
					if (colliding) {
						ok = false;
						break;
					}
				}
				if (ok) {
					world.add_cell(x, y, diameter, 5);
					break;
				}
			}
		}
		for (const l of ship.links) {
			world.add_link(l.a, l.b);
		}
	}
    // const a = Math.random() * 200
    // const b = Math.random() * 200
    // const c = a + Math.random() * 200
    // const d = b + Math.random() * 200
    // setTimeout(() => {
    //     const event = new KeyboardEvent('keydown', { key: 's' });
    //     document.dispatchEvent(event);
    // },  a);
    // setTimeout(() => {
    //     const event = new KeyboardEvent('keydown', { key: 'd' });
    //     document.dispatchEvent(event);
    // }, b );
    // setTimeout(() => {
    //     const event = new KeyboardEvent('keyup', { key: 's' });
    //     document.dispatchEvent(event);
    // }, c);
    // setTimeout(() => {
    //     const event = new KeyboardEvent('keyup', { key: 'd' });
    //     document.dispatchEvent(event);
    // }, d);
    // if (this.ghosts.length < 10) {
    //     setTimeout(() => {
    //         const event = new KeyboardEvent('keydown', { key: 'e' });
    //         document.dispatchEvent(event);
    //     }, 500);
    // }
}


Game.prototype.try_again = function() {
    console.log("try_again")
    this.ghosts.push(JSON.parse(this.worlds[0].get_activation_events()))
    this.restart()
};


export {
    Game,
}
