use crate::gravithrust::Gravithrust;
use crate::kind::Kind;
use crate::math::angle;
use crate::math::cross;
use crate::math::normalize_2;
use crate::math::wrap_around;
use crate::math_small::Vector;
use crate::particle::Particle;
use crate::particle::Particles;
use crate::ship::Ship;
use crate::ship::ShipMore;
impl Gravithrust {
    pub fn get_movement_action(
        &self,
        ship: &Ship,
        slow_down_max_angle_better: f32,
        s: &ShipMore,
    ) -> MovementAction {
        match (s.anchor_pid, s.target_pid) {
            (Some(anchor_pid), Some(target_pid)) => anchor_target_movement_action(
                self.max_rotation_speed,
                slow_down_max_angle_better,
                ship,
                &self.particles,
                anchor_pid,
                target_pid,
                ship.previous_orientation,
            ),
            (None, Some(target_pid)) => target_only_movement_action(
                self.forward_max_speed,
                self.max_speed_at_target,
                self.slow_down_max_speed_to_target_ratio,
                self.forward_max_angle,
                self.max_rotation_speed,
                self.slow_down_max_angle,
                ship,
                &self.particles[target_pid],
            ),
            _ => MovementAction::Nothing,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub enum MovementAction {
    SlowDown,
    TurnLeft,
    TurnRight,
    Forward,
    TranslateLeft,
    TranslateRight,
    Nothing,
}
pub fn apply_movement_action(
    particles: &mut Particles,
    movement_action: MovementAction,
    ship_more: &ShipMore,
) {
    for pid in &ship_more.pids {
        if particles[*pid].k == Kind::Booster {
            particles[*pid].a = 0;
        }
    }
    let pid0 = ship_more.pids[0];
    match movement_action {
        MovementAction::SlowDown => {
            for x in &ship_more.ship_control.slow {
                particles[pid0 + x].a = 1;
            }
        }
        MovementAction::TurnLeft => {
            for x in &ship_more.ship_control.left {
                particles[pid0 + x].a = 1;
            }
        }
        MovementAction::TurnRight => {
            for x in &ship_more.ship_control.right {
                particles[pid0 + x].a = 1;
            }
        }
        MovementAction::Forward => {
            for x in &ship_more.ship_control.forward {
                particles[pid0 + x].a = 1;
            }
        }
        MovementAction::TranslateLeft => {
            for x in &ship_more.ship_control.translate_left {
                particles[pid0 + x].a = 1;
            }
        }
        MovementAction::TranslateRight => {
            for x in &ship_more.ship_control.translate_right {
                particles[pid0 + x].a = 1;
            }
        }
        MovementAction::Nothing => {}
    };
}
pub fn anchor_target_movement_action(
    max_rotation_speed: f32,
    slow_down_max_angle_better: f32,
    ship: &Ship,
    particles: &Particles,
    anchor_pid: usize,
    target_pid: usize,
    ship_previous_orientation: Vector,
) -> MovementAction {
    let rotation_speed = cross(ship.orientation, ship_previous_orientation);
    let anchor = particles[anchor_pid].p;
    let target = particles[target_pid].p;
    let target_delta = wrap_around(ship.p, target);
    let target_delta_old = wrap_around(ship.pp, target);
    let target_to_anchor = wrap_around(anchor, target);
    let target_to_anchor_distance = target_to_anchor.d_sqrd.sqrt();
    let distance_to_target = target_delta.d_sqrd.sqrt();
    let speed_toward_target = target_delta_old.d_sqrd.sqrt() - distance_to_target;
    let target_direction = target_delta.d;
    let anchor_delta = wrap_around(ship.p, anchor);
    let distance_to_anchor = anchor_delta.d_sqrd.sqrt();
    let anchor_delta_old = wrap_around(ship.pp, anchor);
    let speed_toward_anchor = anchor_delta_old.d_sqrd.sqrt() - distance_to_anchor;
    let anchor_direction = anchor_delta.d;
    let orientation_angle = cross(normalize_2(ship.orientation), normalize_2(target_direction));
    let angle_anchor_to_target =
        cross(normalize_2(anchor_direction), normalize_2(target_direction));
    if distance_to_target < target_to_anchor_distance
        && orientation_angle.abs() < slow_down_max_angle_better
        && speed_toward_target > -0.000_000_5
    {
        MovementAction::SlowDown
    } else if orientation_angle > 0.0 && rotation_speed > -max_rotation_speed {
        MovementAction::TurnLeft
    } else if orientation_angle < 0.0 && rotation_speed < max_rotation_speed {
        MovementAction::TurnRight
    } else if angle_anchor_to_target > 0.0
        && distance_to_target < target_to_anchor_distance * 1.2
        && speed_toward_anchor < 0.000_001
    {
        MovementAction::TranslateRight
    } else if angle_anchor_to_target < 0.0
        && distance_to_target < target_to_anchor_distance * 1.2
        && speed_toward_anchor < 0.000_001
    {
        MovementAction::TranslateLeft
    } else {
        MovementAction::Nothing
    }
}
pub fn target_only_movement_action(
    forward_max_speed: f32,
    max_speed_at_target: f32,
    slow_down_max_speed_to_target_ratio: f32,
    forward_max_angle: f32,
    max_rotation_speed: f32,
    slow_down_max_angle: f32,
    ship: &Ship,
    target: &Particle,
) -> MovementAction {
    let rotation_speed = cross(ship.orientation, ship.previous_orientation);
    let speed = wrap_around(ship.pp, ship.p).d_sqrd.sqrt();
    let wa1 = wrap_around(ship.pp, target.pp);
    let wa2 = wrap_around(ship.p, target.p);
    let distance_to_target = wa2.d_sqrd.sqrt();
    let target_vs_ship_delta_v = wa1.d_sqrd.sqrt() - distance_to_target;
    let orientation_angle_corrected_2 = angle(normalize_2(ship.cross), normalize_2(ship.td));
    let orientation_angle_corrected = cross(normalize_2(ship.cross), normalize_2(ship.td));
    if orientation_angle_corrected_2.abs() < slow_down_max_angle / 2.0
        && target_vs_ship_delta_v
            > (max_speed_at_target * 0.75)
                .max(distance_to_target * slow_down_max_speed_to_target_ratio)
    {
        MovementAction::SlowDown
    } else if orientation_angle_corrected > 0.0 && rotation_speed > -max_rotation_speed {
        MovementAction::TurnLeft
    } else if orientation_angle_corrected < 0.0 && rotation_speed < max_rotation_speed {
        MovementAction::TurnRight
    } else if speed < forward_max_speed
        && orientation_angle_corrected_2.abs() < forward_max_angle / 2.0
    {
        MovementAction::Forward
    } else {
        MovementAction::Nothing
    }
}
