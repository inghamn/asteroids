use sdl2::EventPump;
use std::f32::consts::{PI};
use crate::components::*;

const FULL_CIRCLE:   f32 = PI * 2.0;
const ROTATION_RATE: f32 = 0.5; // Radians per ms
const THRUST_ACCEL:  f32 = 0.5; // Arena units per ms
const SHOT_SPEED:    f32 = 0.5; // Arena units per ms

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

pub fn update(commands: &Commands, ship: &mut Ship)
{
    if (commands.left) {
        ship.direction = (ship.direction - ROTATION_RATE) % FULL_CIRCLE;
    }
    if (commands.right) {
        ship.direction = (ship.direction - ROTATION_RATE) % FULL_CIRCLE;
    }
}
