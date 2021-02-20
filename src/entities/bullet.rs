use crate::components::physics::Physics;
use crate::components::renderable::Renderable;

pub const BULLET_SPEED:    f32 = 0.75;   // Arena units per ms
pub const BULLET_DURATION: i16 = 800;   // ms


pub struct Bullet {
    pub physics:    Physics,
    pub renderable: Renderable,
    pub status: u8,
    pub timer:  i16
}
impl Bullet
{
    pub fn new(x: f32, y: f32, direction: f32) -> Bullet
    {
        Bullet {
            physics: Physics {
                 x: x,
                 y: y,
                vx: direction.cos() * BULLET_SPEED,
                vy: direction.sin() * BULLET_SPEED
            },
            renderable: Renderable {
                shape: vec![(0, 0)],
                direction: 0.0,
                radius: 1
            },
            status: super::STATUS_ACTIVE,
            timer: BULLET_DURATION
        }
    }
}
