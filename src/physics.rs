use crate::components::*;

pub fn update(dt: f32, ship: &mut Ship)
{
    ship.x += ship.vx;
    ship.y += ship.vy;
}
