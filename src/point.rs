//
// Represent 2d coordinates
//
#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

//
// Utilitarian functions
//
impl Point {

    //
    // Returns a distance squared
    //
    pub fn get_distance_squared(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let delta_x = x1 - x2;
        let delta_y = y1 - y2;
        delta_x * delta_x + delta_y * delta_y
    }

    //
    // Returns the distance between two pairs of xy coordinates
    //
    pub fn get_distance(
            x1: f64, y1: f64,
            x2: f64, y2: f64
    ) -> f64 {
        Point::get_distance_squared(x1, y1, x2, y2).sqrt()
    }

    //
    // Returns the distance between two Points.
    //
    pub fn get_distance_2(p1: & Point, p2: & Point) -> f64 {
        Point::get_distance(p1.x, p1.y, p2.x, p2.y)
    }
}
