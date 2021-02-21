use rand::prelude::random;
use crate::components::physics::Physics;
use crate::components::renderable::Renderable;

static SHAPE_ONE: [(i8, i8);  11] = [(-4,  2),
                                     (-2,  4),
                                     ( 0,  2),
                                     ( 2,  4),
                                     ( 4,  2),
                                     ( 3,  0),
                                     ( 4, -2),
                                     ( 1, -4),
                                     (-2, -4),
                                     (-4, -2),
                                     (-4,  2)];
pub struct Asteroid<'a> {
    pub physics:    Physics,
    pub renderable: Renderable<'a>,
    pub status:    u8
}

impl<'a> Asteroid<'a>
{
    pub fn new(x: f32, y: f32) -> Asteroid<'a>
    {
        Asteroid {
            physics: Physics {
                x: x,
                y: y,
                vx:  0.1,
                vy:  0.1
            },
            renderable: Renderable {
                shape:  &SHAPE_ONE,
                radius: 4,
                direction: random::<f32>() * crate::TAU
            },
            status: super::STATUS_ACTIVE
        }
    }
}
