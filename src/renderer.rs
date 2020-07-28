extern crate sdl2;
use crate::components::*;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{WindowCanvas};

/*
                 n   d  wo ho
     320 x  240  1 / 3
     640 x  480  1 / 2
    1280 x 1024  1 / 1  21  6
    1920 x 1080  3 / 2
*/

pub fn render(canvas: &mut WindowCanvas, ship: &mut Ship) -> Result<(), String>
{
    let scale = 1.0f32 / 1.0f32;
    let wo = 21.0f32;
    let ho =  6.0f32;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for i in (0..ship.shape.len()) {
        let x = (ship.x as f32 + ship.shape[i].0 as f32 - wo) * scale;
        let y = (ship.y as f32 + ship.shape[i].1 as f32 - ho) * scale;

        ship.render[i] = Point::new(
            x.round() as i32,
            y.round() as i32
        );
    }
    canvas.draw_lines(&ship.render[..]);

    canvas.present();
    Ok(())
}
