use crate::gravithrust::Gravithrust;
use crate::kind::Kind;
use crate::math::normalize_2;
use crate::particle::QuantityKind;
impl Gravithrust {
    pub fn update_particles(&mut self) {
        for (pid, p1) in self.particles.iter_mut().enumerate() {
            let mut d1 = &mut self.particles_internal[pid];
            p1.direction = normalize_2(d1.direction);
            if !p1.k.is_static() {
                p1.v.x += d1.dv.x;
                p1.v.y += d1.dv.y;
                p1.p.x += d1.dp.x;
                p1.p.y += d1.dp.y;
            }
            if p1.k == Kind::Booster && p1.a == 1 && p1.quantity(QuantityKind::Energy) >= 10 {
                p1.v.x -= d1.direction.x * self.booster_acceleration;
                p1.v.y -= d1.direction.y * self.booster_acceleration;
                p1.remove_quantity(QuantityKind::Energy, 10);
            }
            match &d1.new_state {
                Some(state) => {
                    if p1.live != state.live {
                        if state.live == 0 {
                            self.dead_particles.insert(p1.idx);
                            self.live_particles.remove(&p1.idx);
                        } else {
                            self.dead_particles.remove(&p1.idx);
                            self.live_particles.insert(p1.idx);
                        }
                    }
                    p1.live = state.live;
                }
                _ => {}
            }
            d1.dp.x = 0.0;
            d1.dp.y = 0.0;
            d1.dv.x = 0.0;
            d1.dv.y = 0.0;
            d1.direction.x = 0.0;
            d1.direction.y = 0.0;
            d1.new_state = None;
            p1.v.x = p1.v.x.max(-self.diameter * 0.5);
            p1.v.x = p1.v.x.min(self.diameter * 0.5);
            p1.v.x *= 0.9999;
            p1.v.y = p1.v.y.max(-self.diameter * 0.5);
            p1.v.y = p1.v.y.min(self.diameter * 0.5);
            p1.v.y *= 0.9999;
            p1.p.x = (10.0 + p1.p.x + p1.v.x) % 1.0;
            p1.p.y = (10.0 + p1.p.y + p1.v.y) % 1.0;
            p1.pp.x = p1.p.x - p1.v.x;
            p1.pp.y = p1.p.y - p1.v.y;
            // if p1.q1 > p1.k.capacity() {
            //     error(&format!(
            //         "over capacity | {:?} : {} > {}",
            //         p1.k,
            //         p1.q1,
            //         p1.k.capacity()
            //     ));
            // }
        }
    }
}
