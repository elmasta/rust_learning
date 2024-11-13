#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {

    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point { x, y },
            radius,
        }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn diameter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        let distance = self.center.distance(&other.center);
        distance < (self.radius + other.radius)
    }
}

impl Point {

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

}
