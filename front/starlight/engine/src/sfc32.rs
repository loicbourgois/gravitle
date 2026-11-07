pub struct Sfc32 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Sfc32 {
    pub fn new(seed: &[u32]) -> Sfc32 {
        Sfc32 {
            a: seed[0],
            b: seed[1],
            c: seed[2],
            d: seed[3],
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        const ROTATION: u32 = 21;
        const RIGHT_SHIFT: u32 = 9;
        const LEFT_SHIFT: u32 = 3;
        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.d);
        self.d += 1;
        self.a = self.b ^ (self.b >> RIGHT_SHIFT);
        self.b = self.c.wrapping_add(self.c << LEFT_SHIFT);
        self.c = self.c.rotate_left(ROTATION).wrapping_add(tmp);
        tmp
    }

    pub fn next_f32(&mut self) -> f32 {
        let i: i64 = 4_294_967_296;
        self.next_u32() as f32 / (i as f32)
    }
}
