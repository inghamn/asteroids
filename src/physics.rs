use crate::components::ship::Ship;

pub fn update(dt: f32, ship: &mut Ship)
{
    ship.physics.x = position_wrapped_horizontal(ship.physics.x, ship.physics.vx);
    ship.physics.y = position_wrapped_vertical  (ship.physics.y, ship.physics.vy);

    for bullet in ship.bullets.iter_mut() {
        bullet.x = position_wrapped_horizontal(bullet.x, bullet.vx);
        bullet.y = position_wrapped_vertical  (bullet.y, bullet.vy);
    }
}

fn position_wrapped_horizontal(x: f32, vx: f32) -> f32
{
    (x + vx + crate::ARENA_WIDTH) % crate::ARENA_WIDTH
}

fn position_wrapped_vertical(y: f32, vy: f32) -> f32
{
    (y + vy + crate::ARENA_HEIGHT) % crate::ARENA_HEIGHT
}
