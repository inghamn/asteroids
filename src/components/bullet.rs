const SHOT_SPEED:    f32 = 7.0;   // Arena units per ms
const SHOT_DURATION: u16 = 288;   // ms

pub struct Bullet {
    pub  x: i16,
    pub  y: i16,
    pub vx:  u8,
    pub vy:  u8,
    pub status: u8
}
