use crate::data::Data;
use crate::gravitle::cell_id;
use crate::gravitle::part_id_next;
use crate::gravitle::DnaSave;
use crate::gravitle::DIAMETER_NEW;
use crate::gravitle::HEIGHT;
use crate::gravitle::START_ACTIVITY;
use crate::gravitle::WIDTH;
use crate::link::Link;
use crate::maths::distance_squared_wrap_around;
use crate::maths::p_coords;
use crate::plan::dna_to_str;
use crate::plan::dna_to_link_plan;
use crate::plan::link_plan_to_ab_plan;
use crate::point::Point;
use crate::Float;
use crate::Pid;
use core::part::Kind;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::RwLockWriteGuard;
use uuid::Uuid;

pub fn add_part_energy(data: &mut Data, position: &Point) -> Pid {
    add_part_(data, position, &Kind::Energy, 1.0, [255, 255, 0])
}

fn add_part_(data: &mut Data, position: &Point, kind: &Kind, energy: Float, color: [u8; 3]) -> Pid {
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
    data.parts[pid].activity = START_ACTIVITY;
    data.parts[pid].energy = energy;
    data.parts[pid].uuid = Uuid::new_v4().as_u128();
    data.parts[pid].r = color[0];
    data.parts[pid].g = color[1];
    data.parts[pid].b = color[2];
    data.new_pids[pid] = pid;
    pid
}

#[allow(clippy::too_many_arguments)]
fn add_part(
    data: &mut Data,
    position: &Point,
    kind: &Kind,
    energy: Float,
    color: [u8; 3],
    dnas: &mut HashMap<u128, DnaSave>,
    dna_save: &DnaSave,
    dna_save_file_path: &str,
) -> Pid {
    let pid = add_part_(data, position, kind, energy, color);
    if let Kind::Core = *kind {
        dnas.insert(
            data.parts[pid].uuid,
            DnaSave {
                dna: dna_save.dna,
                parent_uuid: dna_save.parent_uuid,
            },
        );
        let line = format!(
            "{},{},{}\n",
            Uuid::from_u128(data.parts[pid].uuid)
                .to_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer()),
            Uuid::from_u128(dna_save.parent_uuid)
                .to_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer()),
            dna_to_str(&dna_save.dna)
        );
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true) // This is needed to append to file
            .open(dna_save_file_path)
            .unwrap();
        file.write_all(line.as_bytes()).unwrap();
    }
    pid
}

pub fn add_link(
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

#[allow(clippy::too_many_arguments)]
pub fn add_entity(
    datas: &mut [RwLockWriteGuard<Data>],
    position: &Point,
    // TODO: Allow new entity to spawn rotated.
    _rotation: Float,
    thread_id: usize,
    energy_total: Float,
    dnas: &mut RwLockWriteGuard<HashMap<u128, DnaSave>>,
    dna_save: &DnaSave,
    dna_save_file_path: &str,
) {
    let link_plan = dna_to_link_plan(&dna_save.dna);
    let plan = link_plan_to_ab_plan(&link_plan);
    let mut positions: Vec<Point> = Vec::new();
    let mut pids: Vec<Pid> = Vec::new();
    let mut part_count = 2.0;
    for part in plan.part_plans.iter() {
        match part.k {
            Kind::Invalid => {
                break;
            }
            _ => {
                part_count += 1.0;
            }
        }
    }
    let energy_per_part = energy_total / part_count;
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
        pids.push(add_part(
            data,
            &position1,
            &plan.kinds[0],
            energy_per_part,
            [plan.colors[0], plan.colors[1], plan.colors[2]],
            dnas,
            dna_save,
            dna_save_file_path,
        ));
        pids.push(add_part(
            data,
            &position2,
            &plan.kinds[1],
            energy_per_part,
            [plan.colors[3], plan.colors[4], plan.colors[5]],
            dnas,
            dna_save,
            dna_save_file_path,
        ));
    }
    add_link(datas, pids[0], pids[1], thread_id, thread_id);
    for part in plan.part_plans.iter() {
        match part.k {
            Kind::Invalid => {
                break;
            }
            _ => {
                let position_a = part.a % positions.len();
                if position_a != part.a {
                    println!(
                        "[WARN]: position_a != part.a: {} != {} | l={}",
                        position_a,
                        part.a,
                        positions.len()
                    );
                }
                let position_b = part.b % positions.len();
                if position_b != part.b {
                    println!(
                        "[WARN]: position_b != part.b: {} != {} | l={}",
                        position_b,
                        part.b,
                        positions.len()
                    );
                }
                let kind = part.k;
                let position = p_coords(&positions[position_a], &positions[position_b]);
                let pid1 = {
                    let data = &mut datas[thread_id];
                    add_part(
                        data,
                        &position,
                        &kind,
                        energy_per_part,
                        [part.cr, part.cg, part.cb],
                        dnas,
                        dna_save,
                        dna_save_file_path,
                    )
                };
                let p1 = datas[thread_id].parts[pid1];
                for pid2 in pids.iter() {
                    let p2 = datas[thread_id].parts[*pid2];
                    let d_sqrd = distance_squared_wrap_around(&p1.p, &p2.p);
                    let diams = (p1.d + p2.d) * 0.5;
                    if d_sqrd < diams * diams * 2.0 {
                        add_link(datas, pid1, *pid2, thread_id, thread_id);
                    }
                }
                pids.push(pid1);
                positions.push(position);
            }
        }
    }
}
