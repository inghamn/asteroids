pub const STATUS_DEAD:   u8 = 0;
pub const STATUS_ACTIVE: u8 = 1;

pub struct Physics {
    pub  x: f32,
    pub  y: f32,
    pub vx: f32,
    pub vy: f32,
    pub direction: f32 // Radians
}

pub mod ship;
pub mod bullet;
pub mod asteroid;
pub mod saucer;
