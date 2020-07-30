use sdl2::rect::Point;

pub const STATUS_DEAD:   u8 = 0;
pub const STATUS_ACTIVE: u8 = 1;

pub struct Asteroid {
    pub  x: i16,
    pub  y: i16,
    pub vx:  u8,
    pub vy:  u8,
    pub status: u8
}

pub struct Ship {
    pub  x: f32,
    pub  y: f32,
    pub vx: f32,
    pub vy: f32,
    pub direction: f32, // Radians
    pub status: u8,
    pub shape:  Vec<(i8, i8)>,
    pub render: Vec<Point>
}
impl Ship {
    pub fn new(x: f32, y: f32, d: f32) -> Ship {
        Ship {
            x: x,
            y: y,
            vx:  0.0,
            vy:  0.0,
            direction: d,
            status: STATUS_ACTIVE,
            // Definition for 1280x1024
            shape: vec![
                (-1,  1),
                (-1, -1),
                (-2, -2),
                ( 4,  0),
                (-2,  2),
                (-1,  1)
            ],

            // Last rendered shape values
            render: vec![
                Point::new(18,  9),
                Point::new(18, 27),
                Point::new( 9, 36),
                Point::new(63, 18),
                Point::new( 9,  0),
                Point::new(18,  9)
            ]
        }
    }
}

pub struct Shot {
    pub  x: i16,
    pub  y: i16,
    pub vx:  u8,
    pub vy:  u8,
    pub status: u8
}

pub struct Saucer {
    pub  x: i16,
    pub  y: i16,
    pub vx:  u8,
    pub vy:  u8,
    pub status: u8
}
