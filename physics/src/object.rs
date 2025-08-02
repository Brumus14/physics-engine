use nalgebra::Vector2;

pub enum Shape {
    Point,
    Circle(f64),
    Polygon(Vec<Vector2<f64>>),
}

pub struct Object {
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub acceleration: Vector2<f64>,
    pub mass: f64,
    pub shape: Shape,
}

impl Object {
    fn step(&mut self, delta_time: f64) {
        self.position;
    }
}
