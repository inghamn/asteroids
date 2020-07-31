use crate::components::*;

pub fn update(dt: f32, ship: &mut Ship)
{
    ship.x = (ship.x + ship.vx + crate::ARENA_WIDTH ) % crate::ARENA_WIDTH;
    ship.y = (ship.y + ship.vy + crate::ARENA_HEIGHT) % crate::ARENA_HEIGHT;
}
