use crate::data::Data;
use crate::gravitle::cell_id;
use crate::gravitle::part_id_next;
use crate::gravitle::DIAMETER_NEW;
use crate::gravitle::HEIGHT;
use crate::gravitle::WIDTH;
use crate::link::Link;
use crate::maths::distance_squared_wrap_around;
use crate::maths::p_coords;
use crate::plan::Plan;
use crate::point::Point;
use crate::Float;
use crate::Pid;
use core::part::Kind;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;

pub fn add_part(data: &mut Data, position: &Point, kind: &Kind, energy: Float) -> Pid {
    let i: usize = ((position.x * WIDTH as Float) as usize) % WIDTH;
    let j: usize = ((position.y * HEIGHT as Float) as usize) % HEIGHT;
    let cid = cell_id(i, j);
    let pid = part_id_next(cid, &data.depths);
    data.depths[cid] += 1;
    data.parts[pid].p.x = position.x;
    data.parts[pid].p.y = position.y;
    data.parts[pid].pp.x = position.x;
    data.parts[pid].pp.y = position.y;
    data.parts[pid].kind = *kind;
    data.parts[pid].d = DIAMETER_NEW;
    data.parts[pid].m = 1.0;
    data.parts[pid].energy = energy;
    data.new_pids[pid] = pid;
    pid
}


pub fn add_link_2(
    datas: &mut [RwLockWriteGuard<Data>],
    pid_a: Pid,
    pid_b: Pid,
    thread_id_a: usize,
    thread_id_b: usize,
) {
    datas[thread_id_a].links[thread_id_b].push(Link {
        pid1: pid_a,
        pid2: pid_b,
    });
    datas[thread_id_b].links[thread_id_a].push(Link {
        pid1: pid_b,
        pid2: pid_a,
    });
}



pub fn add_entity_2(
    datas: &mut [RwLockWriteGuard<Data>],
    plan: &Plan,
    position: &Point,
    // TODO
    // Allow new entity to spawn rotated.
    _rotation: Float,
    thread_id: usize,
    energy_total: Float,
) {
    let mut positions: Vec<Point> = Vec::new();
    let mut pids: Vec<Pid> = Vec::new();
    let energy_per_part = energy_total / (plan.part_plans.len() + 2) as Float;
    {
        let data = &mut datas[thread_id];
        let delta_position = Point {
            x: DIAMETER_NEW * 0.5,
            y: 0.0,
        };
        let position1 = position - &delta_position;
        let position2 = position + &delta_position;
        positions.push(position1);
        positions.push(position2);
        pids.push(add_part(data, &position1, &plan.kinds[0], energy_per_part));
        pids.push(add_part(data, &position2, &plan.kinds[1], energy_per_part));
    }
    add_link_2(datas, pids[0], pids[1], thread_id, thread_id);
    for part in plan.part_plans.iter() {
        let position = p_coords(&positions[part.a], &positions[part.b]);
        let pid1 = {
            let data = &mut datas[thread_id];
            add_part(data, &position, &part.k, energy_per_part)
        };
        let p1 = datas[thread_id].parts[pid1];
        for pid2 in pids.iter() {
            let p2 = datas[thread_id].parts[*pid2];
            let d_sqrd = distance_squared_wrap_around(&p1.p, &p2.p);
            let diams = (p1.d + p2.d) * 0.5;
            if d_sqrd < diams * diams * 2.0 {
                add_link_2(datas, pid1, *pid2, thread_id, thread_id);
            }
        }
        pids.push(pid1);
        positions.push(position);
    }
    // datas[thread_id]
    //     .write()
    //     .unwrap()
    //     .parts_to_remove
    //     .insert(pids[0]);
}
