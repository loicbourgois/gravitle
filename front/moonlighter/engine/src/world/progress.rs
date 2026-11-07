// In progress, state can change
// if it preserves internal equilibrium.
// Transfer to the outside happens elsewhere. 

use crate::world::Kind;
use crate::world::World;
use crate::state::State;

pub fn press(world: &mut World) {
    for i1 in 0..world.particles.len() {
        let p1 = world.pm(i1);
        let dust_empty = p1.moon_q == 0;
        let dust_full = p1.moon_q == p1.moon_c;
        let progress_0 = p1.progress_q == 0;
        let progress_full = p1.progress_q == p1.progress_c;
        let solid_0 = p1.solidmoon_q == 0;
        let solid_full = p1.solidmoon_q == p1.solidmoon_c;
        match (p1.kind, p1.state) {
            (Kind::Press, State::ReadyEmpty) if dust_empty && progress_0 && solid_0 => {
                p1.state = State::LoadingMoonDust;
            }
            (Kind::Press, State::LoadingMoonDust) if dust_full && progress_0 && solid_0 => {
                p1.state = State::Pressing;
            }
            (Kind::Press, State::Pressing) if dust_full && !progress_full && solid_0 => {
                p1.progress_q += 1;
            }
            (Kind::Press, State::Pressing) if dust_full && progress_full => {
                p1.moon_q = 0;
                p1.solidmoon_q = 1;
                p1.state = State::Releasing;
            }
            (Kind::Press, State::Releasing) if solid_full && !progress_0 => {
                p1.progress_q -= 1;
            }
            (Kind::Press, State::Releasing) if dust_empty && progress_0 && solid_full => {
                p1.state = State::ReadyFull;
            }
            (Kind::Press, State::ReadyFull) if dust_empty && progress_0 && solid_0 => {
                p1.state = State::ReadyEmpty;
            }
            // (Kind::Press, _) => {
            //     p1.state_status = StateStatus::Error;
            // }
            _ => {}
        }
    }
}


pub fn conveyor(world: &mut World) {
    for i1 in 0..world.particles.len() {
        let p1 = world.pm(i1);
        let progress_0 = p1.progress_q == 0;
        let progress_full = p1.progress_q == p1.progress_c;
        let solid_0 = p1.solidmoon_q == 0;
        let solid_full = p1.solidmoon_q == p1.solidmoon_c;
        match (p1.kind, p1.state) {
            (Kind::Conveyor, State::ReadyStart) if solid_full && progress_0 => {
                p1.state = State::MovingAway;
            }
            (Kind::Conveyor, State::MovingAway) if solid_full && !progress_full => {
                p1.progress_q += 1;
            }
            (Kind::Conveyor, State::MovingAway) if solid_full && progress_full => {
                p1.state = State::ReadyEnd;
            }
            (Kind::Conveyor, State::ReadyEnd) if solid_0 && progress_full => {
                p1.state = State::MovingBack;
            }
            (Kind::Conveyor, State::MovingBack) if solid_0 && !progress_0 => {
                p1.progress_q -= 1;
            }
            (Kind::Conveyor, State::MovingBack) if solid_0 && progress_0 => {
                p1.state = State::ReadyStart;
            }
            _ => {}
        }
    }
}
