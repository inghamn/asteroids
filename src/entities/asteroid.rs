use rand::{thread_rng, Rng};
use crate::components::physics::Physics;
use crate::components::renderable::Renderable;

const SIZE_LARGE: f32 = 12.5;
const SIZE_MED:   f32 = 6.0;
const SIZE_SMALL: f32 = 3.0;

const SPEED_MIN:  f32 = 0.0375; // Arena units per ms
const SPEED_MAX:  f32 = 0.1;   // Arena units per ms

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
        let mut rng = thread_rng();
        let d: f32 = rng.gen::<f32>() * crate::TAU;
        let s: f32 = rng.gen_range(SPEED_MIN..SPEED_MAX);
        println!("{},{}", d,s);
        Asteroid {
            physics: Physics {
                x:  x,
                y:  y,
                vx: d.cos() * s,
                vy: d.sin() * s
            },
            renderable: Renderable {
                shape:  &SHAPE_ONE,
                scale:  SIZE_LARGE,
                radius: 4,
                direction: rng.gen::<f32>() * crate::TAU
            },
            status: super::STATUS_ACTIVE
        }
    }
}
