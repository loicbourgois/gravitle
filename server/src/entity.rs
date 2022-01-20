use crate::gravitle::DIAMETER_MIN;
use crate::gravitle::DIAMETER_MAX;
use crate::point::Point;
use crate::Float;
use crate::plan::Plan;
use crate::data::Data;
use std::sync::RwLock;
use std::sync::Arc;
use crate::link::Link;
use crate::PID;
use crate::gravitle::part_id_next;
use crate::gravitle::HEIGHT;
use crate::gravitle::WIDTH;
use crate::maths::p_coords;
use crate::gravitle::cell_id;
use crate::maths::distance_squared_wrap_around;

pub fn add_part(data: &mut Data, position: &Point) -> PID {
    let i: usize = ((position.x * WIDTH as Float) as usize)%WIDTH;
    let j: usize = ((position.y * HEIGHT as Float) as usize)%HEIGHT;
    let cid = cell_id(i, j);
    let pid = part_id_next(cid, &data.depths);
    data.depths[cid] += 1;
    data.parts[pid].p.x = position.x;
    data.parts[pid].p.y = position.y;
    let max_speed = 0.001 * 1.0;
    data.parts[pid].pp.x = position.x;
    data.parts[pid].pp.y = position.y;
    data.parts[pid].d = DIAMETER_MIN;
    data.parts[pid].m = 1.0;
    data.new_pids[pid] = pid;
    return pid;
}

pub fn add_link(datas: &mut Vec<Arc<RwLock<Data>>>, pid1: PID, pid2: PID, thread_id1: usize, thread_id2: usize) {
    datas[thread_id1].write().unwrap().links[thread_id2].push(Link {
        pid1: pid1,
        pid2: pid2,
    });
    datas[thread_id2].write().unwrap().links[thread_id1].push(Link {
        pid1: pid2,
        pid2: pid1,
    });
}

pub fn add_entity(datas: &mut Vec<Arc<RwLock<Data>>>, plan: &Plan, position: &Point, rotation: Float, thread_id: usize) {
    let mut positions = Vec::new();
    let mut pids = Vec::new();
    {
        let mut data = &mut datas[thread_id].write().unwrap();
        let position1 = position - &Point {
            x: DIAMETER_MIN * 0.5,
            y: 0.0,
        };
        let position2 = position + &Point {
            x: DIAMETER_MIN * 0.5,
            y: 0.0,
        };
        positions.push(position1);
        positions.push(position2);
        pids.push(add_part(data, &position1));
        pids.push(add_part(data, &position2));
    }
    add_link(datas, pids[0], pids[1], thread_id, thread_id);
    for part in plan.part_plans.iter() {
        let position = p_coords(&positions[part.a], &positions[part.b]);
        let pid1 = {
            let mut data = &mut datas[thread_id].write().unwrap();
            add_part(data, &position)
        };
        let p1 = datas[thread_id].read().unwrap().parts[pid1];
        for pid2 in pids.iter() {
            let p2 = datas[thread_id].read().unwrap().parts[*pid2];
            let d_sqrd = distance_squared_wrap_around(&p1.p, &p2.p);
            let diams = (p1.d + p2.d) * 0.5;
            if d_sqrd < diams * diams * 2.0 {
                add_link(datas, pid1, *pid2, thread_id, thread_id);
            }
        }
        pids.push(pid1);
        positions.push(position);
    }
    datas[thread_id].write().unwrap().parts_to_remove.insert(pids[0]);
}
