extern crate sdl2;
use crate::components::renderable::*;
use crate::entities::ship::Ship;

use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};

pub fn render(canvas: &mut WindowCanvas, ship: &Ship) -> Result<(), String>
{

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));


    draw_renderable(canvas, &ship.renderable, ship.physics.x, ship.physics.y);
    for b in ship.bullets.iter() {
        draw_renderable(canvas, &b.renderable, b.physics.x, b.physics.y);
    }

    canvas.present();
    Ok(())
}
