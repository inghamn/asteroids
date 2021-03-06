use crate::components::physics::Physics;
use crate::components::renderable::Renderable;

pub const BULLET_SPEED:    f32 = 0.75;   // Arena units per ms
pub const BULLET_DURATION: i16 = 800;    // ms

static BULLET_SHAPE: [(i8, i8); 1] = [(0,0)];

pub struct Bullet<'a> {
    pub physics:    Physics,
    pub renderable: Renderable<'a>,
    pub status: u8,
    pub timer:  i16
}
impl<'a> Bullet<'a>
{
    pub fn new(x: f32, y: f32, direction: f32) -> Bullet<'a>
    {
        Bullet {
            physics: Physics {
                 x: x,
                 y: y,
                vx: direction.cos() * BULLET_SPEED,
                vy: direction.sin() * BULLET_SPEED
            },
            renderable: Renderable {
                shape:  &BULLET_SHAPE,
                scale:  1.0,
                direction: 0.0,
                radius: 1
            },
            status: super::STATUS_ACTIVE,
            timer: BULLET_DURATION
        }
    }
}
