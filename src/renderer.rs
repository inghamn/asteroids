extern crate sdl2;
use crate::components::*;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{WindowCanvas};

const SHAPE_SCALE: f32 = 2.0;
const COORD_SCALE: f32 = 0.5;

/*
     320 x  240  1
     640 x  480  2
    1280 x 1024  4
    1920 x 1080  6
*/

pub fn render(canvas: &mut WindowCanvas, ship: &mut Ship) -> Result<(), String>
{
    let sin_t  = ship.direction.sin() * SHAPE_SCALE;
    let cos_t  = ship.direction.cos() * SHAPE_SCALE;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for i in (0..ship.shape.len()) {
        let x = (ship.shape[i].0 as f32 * cos_t) - (ship.shape[i].1 as f32 * sin_t);
        let y = (ship.shape[i].1 as f32 * cos_t) + (ship.shape[i].0 as f32 * sin_t);

        ship.render[i] = Point::new(
            x.round() as i32 + (ship.x * COORD_SCALE).round() as i32,
            y.round() as i32 + (ship.y * COORD_SCALE).round() as i32
        );
    }
    let _ = canvas.draw_lines(&ship.render[..]);

    canvas.present();
    Ok(())
}
