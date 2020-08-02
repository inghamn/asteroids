use std::f32::consts::{PI};

const TAU:           f32 = PI * 2.0;
const ROTATION_RATE: f32 = 0.008; // Radians per ms
const THRUST_ACCEL:  f32 = 0.05;  // Arena units per ms

pub struct Ship {
    pub  x: f32,
    pub  y: f32,
    pub vx: f32,
    pub vy: f32,
    pub direction: f32, // Radians
    pub status: u8,
    pub shape:  Vec<(i8, i8)>
}
impl Ship {
    pub fn new(x: f32, y: f32, d: f32) -> Ship {
        Ship {
            x: x,
            y: y,
            vx:  0.0,
            vy:  0.0,
            direction: d,
            status: super::STATUS_ACTIVE,
            // Definition for 320x240
            shape: vec![
                (-1,  1),
                (-1, -1),
                (-2, -2),
                ( 4,  0),
                (-2,  2),
                (-1,  1)
            ]
        }
    }
    /**
     * Change state of ship based on user inputs
     * @param dt  Milliseconds since last update
     */
    pub fn update(&mut self, dt: f32, commands: &Commands)
    {
        let t = ROTATION_RATE * dt;

        if commands.left  { self.direction = (self.direction - t) % TAU; }
        if commands.right { self.direction = (self.direction + t) % TAU; }

        if commands.thrust {
            self.vx += self.direction.cos() * THRUST_ACCEL;
            self.vy += self.direction.sin() * THRUST_ACCEL;
        }
    }
}

pub struct Commands {
    pub left:       bool,
    pub right:      bool,
    pub thrust:     bool,
    pub fire:       bool,
    pub hyperspace: bool
}
impl Commands {
    pub fn new() -> Commands {
        Commands {
            left:       false,
            right:      false,
            thrust:     false,
            fire:       false,
            hyperspace: false
        }
    }
}
