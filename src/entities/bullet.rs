pub const BULLET_SPEED:    f32 = 7.0;   // Arena units per ms
pub const BULLET_DURATION: u16 = 288;   // ms

pub struct Bullet {
    pub  x: f32,
    pub  y: f32,
    pub vx: f32,
    pub vy: f32,
    pub status: u8,
    pub timer:  u16
}
