#[derive(Debug)]
pub struct Player {
    pub x: f32,
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            speed: 120.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.x = self.x + (self.speed * dt);
    }
}
