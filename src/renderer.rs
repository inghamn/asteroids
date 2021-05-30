extern crate sdl2;
use crate::components::renderable::*;
use crate::entities::asteroid::Asteroid;
use crate::entities::ship::Ship;

use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};
use sdl2::gfx::primitives::DrawRenderer;

pub fn render(canvas: &mut WindowCanvas, ship: &Ship, asteroids: &Vec<Asteroid>) -> Result<(), String>
{

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));


    draw_renderable(canvas, &ship.renderable, ship.physics.x, ship.physics.y);
    for b in ship.bullets.iter() {
        draw_renderable(canvas, &b.renderable, b.physics.x, b.physics.y);
    }
    for a in asteroids.iter() {
        draw_renderable(canvas, &a.renderable, a.physics.x, a.physics.y);

        // Draw collision boundary
        canvas.circle(a.physics.x         as i16,
                      a.physics.y         as i16,
                      (a.renderable.radius as f32 * a.renderable.scale * COORD_SCALE) as i16,
                      Color::RGB(255, 255, 255)
        );
    }

    canvas.present();
    Ok(())
}
