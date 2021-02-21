use std::f32::consts::{PI};
use crate::entities::bullet::*;
use crate::components::physics::Physics;
use crate::components::renderable::Renderable;

const TAU:           f32 = PI * 2.0;
const ROTATION_RATE: f32 = 0.008;   // Radians per ms
const THRUST_ACCEL:  f32 = 0.0005;  // Arena units / ms / ms button held
const MAX_BULLETS: usize = 4;

// Definition for 320x240
static FREEFALL: [(i8, i8); 6] = [(-1,  1),
                                  (-1, -1),
                                  (-2, -2),
                                  ( 4,  0),
                                  (-2,  2),
                                  (-1,  1)];
static THRUST:   [(i8, i8); 8] = [(-1,  1),
                                  (-1, -1),
                                  (-2, -2),
                                  ( 4,  0),
                                  (-2,  2),
                                  (-1,  1),
                                  (-3,  0),
                                  (-1, -1)];

pub struct Ship<'a> {
    pub physics:    Physics,
    pub renderable: Renderable<'a>,
    pub status:    u8,
    pub bullets:   Vec<Bullet<'a>>
}
impl<'a> Ship<'a> {
    pub fn new(x: f32, y: f32, d: f32) -> Ship<'a> {
        Ship {
            physics: Physics {
                x: x,
                y: y,
                vx:  0.0,
                vy:  0.0
            },
            renderable: Renderable {
                shape:  &FREEFALL,
                radius: 4,
                direction: d
            },
            status: super::STATUS_ACTIVE,
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

        if commands.left  { self.renderable.direction = (self.renderable.direction - t) % TAU; }
        if commands.right { self.renderable.direction = (self.renderable.direction + t) % TAU; }

        if commands.fire && self.bullets.len() < MAX_BULLETS {
            self.bullets.push(Bullet::new(self.physics.x, self.physics.y, self.renderable.direction));
        }

        if commands.thrust {
            self.physics.vx += self.renderable.direction.cos() * THRUST_ACCEL * dt;
            self.physics.vy += self.renderable.direction.sin() * THRUST_ACCEL * dt;
            self.renderable.shape = &THRUST;
        }

        if commands.thrust_stop { self.renderable.shape = &FREEFALL; }

        for b in &mut self.bullets { b.timer -= dt.floor() as i16; }
        self.bullets.retain(|b| b.timer > 0);
    }
}

pub struct Commands {
    pub left:        bool,
    pub right:       bool,
    pub thrust:      bool,
    pub fire:        bool,
    pub hyperspace:  bool,
    pub thrust_stop: bool
}
impl Commands {
    pub fn new() -> Commands {
        Commands {
            left:        false,
            right:       false,
            thrust:      false,
            fire:        false,
            hyperspace:  false,
            thrust_stop: false
        }
    }
}
