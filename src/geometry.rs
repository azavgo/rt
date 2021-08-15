pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

pub struct Vector {
    p: Point,
}

impl Vector {
    pub fn new(p: Point) -> Self {
        Self { p: p }
    }
}
