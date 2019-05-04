pub struct Segment {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    p1_id: usize,
    p2_id: usize
}

impl Segment {
    pub fn new(p1_id: usize, p2_id: usize) -> Segment {
        Segment {
            p1_id: p1_id,
            p2_id: p2_id,
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0
        }
    }

    pub fn set_coordinates(
            &mut self,
            x1: & f64, y1: & f64,
            x2: & f64, y2: & f64
    ) {
        self.x1 = *x1;
        self.y1 = *y1;
        self.x2 = *x2;
        self.y2 = *y2;
    }

    pub fn get_p1_id(&self) -> usize {
        self.p1_id
    }

    pub fn get_p2_id(&self) -> usize {
        self.p2_id
    }
}
