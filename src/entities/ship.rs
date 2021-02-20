use std::f32::consts::{PI};
use crate::entities::bullet::*;
use crate::components::physics::Physics;

const TAU:           f32 = PI * 2.0;
const ROTATION_RATE: f32 = 0.008; // Radians per ms
const THRUST_ACCEL:  f32 = 0.05;  // Arena units per ms
const MAX_BULLETS: usize = 4;

pub struct Ship {
    pub physics: Physics,
    pub direction: f32, // Radians
    pub status:    u8,
    pub shape:     Vec<(i8, i8)>,
    pub radius:    u8,
    pub bullets:   Vec<Bullet>
}
impl Ship {
    pub fn new(x: f32, y: f32, d: f32) -> Ship {
        Ship {
            physics: Physics {
                x: x,
                y: y,
                vx:  0.0,
                vy:  0.0
            },
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
            ],
            radius: 4,
            bullets: Vec::with_capacity(MAX_BULLETS)
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

        if commands.fire && self.bullets.len() <= MAX_BULLETS { self.bullets.push(self.fire()); }

        if commands.thrust {
            self.physics.vx += self.direction.cos() * THRUST_ACCEL;
            self.physics.vy += self.direction.sin() * THRUST_ACCEL;
        }
    }

    fn fire(&self) -> Bullet
    {
        Bullet {
            x:   self.physics.x,
            y:   self.physics.y,
            vx: (self.direction.tan() / self.physics.y),
            vy: (self.direction.tan() * self.physics.x),
            status: super::STATUS_ACTIVE,
            timer: BULLET_DURATION
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
