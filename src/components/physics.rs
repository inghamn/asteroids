pub struct Physics {
    pub  x: f32,
    pub  y: f32,
    pub vx: f32,
    pub vy: f32,
}

pub fn update(dt: f32, physics: &mut Physics)
{
    physics.x = position_wrapped_horizontal(physics.x, physics.vx, dt);
    physics.y = position_wrapped_vertical  (physics.y, physics.vy, dt);
}

fn position_wrapped_horizontal(x: f32, vx: f32, dt: f32) -> f32
{
    (x + (vx * dt) + crate::ARENA_WIDTH) % crate::ARENA_WIDTH
}

fn position_wrapped_vertical(y: f32, vy: f32, dt: f32) -> f32
{
    (y + (vy * dt) + crate::ARENA_HEIGHT) % crate::ARENA_HEIGHT
}
