use rand::{thread_rng, Rng};
use crate::components::physics::Physics;
use crate::components::renderable::Renderable;

const SIZE_LARGE: f32 = 12.5;
const SIZE_MED:   f32 =  6.0;
const SIZE_SMALL: f32 =  3.0;

const SPEED_MIN:  f32 = 0.0375; // Arena units per ms
const SPEED_MAX:  f32 = 0.1;    // Arena units per ms

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

static SHAPE_TWO: [(i8, i8); 12] = [( 0,  3),
                                    (-2,  4),
                                    (-4,  2),
                                    (-4, -2),
                                    (-2, -4),
                                    (-1, -3),
                                    ( 2, -4),
                                    ( 4, -1),
                                    ( 2,  1),
                                    ( 4,  2),
                                    ( 2,  4),
                                    ( 0,  3)];
static SHAPE_THREE: [(i8, i8); 12] = [( 0, -1),
                                      ( 0, -4),
                                      ( 2, -4),
                                      ( 4, -1),
                                      ( 4,  1),
                                      ( 2,  4),
                                      (-1,  4),
                                      (-4,  1),
                                      (-2,  0),
                                      (-4, -1),
                                      (-2, -4),
                                      ( 0, -1)];
static SHAPE_FOUR: [(i8, i8); 13] = [( 1,  0),
                                     ( 4,  1),
                                     ( 4,  2),
                                     ( 1,  4),
                                     (-2,  4),
                                     (-1,  2),
                                     (-4,  2),
                                     (-4, -1),
                                     (-2, -4),
                                     ( 1, -3),
                                     ( 2, -4),
                                     ( 4, -2),
                                     ( 1,  0)];

pub struct Asteroid<'a> {
    pub physics:    Physics,
    pub renderable: Renderable<'a>,
    pub status:     u8
}


impl<'a> Asteroid<'a>
{
    pub fn new(x: f32, y: f32) -> Asteroid<'a>
    {
        let mut rng = thread_rng();
        let direction: f32 = rng.gen::<f32>() * crate::TAU;
        let speed:     f32 = rng.gen_range(SPEED_MIN..SPEED_MAX);
        let i:          u8 = rng.gen_range(1..4);

        Asteroid {
            physics: Physics {
                x:  x,
                y:  y,
                vx: direction.cos() * speed,
                vy: direction.sin() * speed
            },
            renderable: Renderable {
                scale:  SIZE_LARGE,
                radius: 4,
                direction: rng.gen::<f32>() * crate::TAU,
                shape:  match i {
                    1 => { &SHAPE_ONE   }
                    2 => { &SHAPE_TWO   }
                    3 => { &SHAPE_THREE }
                    _ => { &SHAPE_FOUR  }
                }
            },
            status: super::STATUS_ACTIVE
        }
    }

    pub fn generate_wave(asteroid_count: u8) -> Vec<Asteroid<'a>>
    {
        let mut rng = thread_rng();
        let mut a: Vec<Asteroid> = Vec::new();

        for i in 0..asteroid_count {
            if rng.gen_bool(1.0 / 2.0) {
                a.push(Asteroid::new(0.0, rng.gen_range(1.0..crate::ARENA_HEIGHT)));
            }
            else {
                a.push(Asteroid::new(rng.gen_range(1.0..crate::ARENA_WIDTH), 0.0));
            }
        }
        a
    }
}
