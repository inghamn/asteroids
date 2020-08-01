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
    1280 x  960  4
    1440 x 1080  6
*/

pub fn render(canvas: &mut WindowCanvas, ship: &Ship) -> Result<(), String>
{

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    draw_ship(canvas, ship);

    canvas.present();
    Ok(())
}

fn draw_ship(canvas: &mut WindowCanvas, ship: &Ship)
{
    let render = render_ship(ship, (ship.x, ship.y));
    let _ = canvas.draw_lines(&render[..]);

    let ghost = wrapped_ghost(ship);
    if (ghost.is_some()) {
        let render = render_ship(ship, ghost.unwrap());
        let _ = canvas.draw_lines(&render[..]);
    }
}

fn wrapped_ghost(ship: &Ship) -> Option<(f32, f32)>
{
    let mut ghost_x: Option<f32> = None;
    let mut ghost_y: Option<f32> = None;

    let border = 2.0 * SHAPE_SCALE;
    if (ship.x <= border) { ghost_x = Some(ship.x + crate::ARENA_WIDTH ); }
    if (ship.y <= border) { ghost_y = Some(ship.y + crate::ARENA_HEIGHT); }
    if (ship.x >= crate::ARENA_WIDTH  - border) { ghost_x = Some(ship.x - crate::ARENA_WIDTH ); }
    if (ship.y >= crate::ARENA_HEIGHT - border) { ghost_y = Some(ship.y - crate::ARENA_HEIGHT); }

    if (ghost_x.is_some() && ghost_y.is_none()) {
        ghost_y = Some(ship.y);
    }
    if (ghost_y.is_some() && ghost_x.is_none()) {
        ghost_x = Some(ship.x);
    }
    if (ghost_x.is_some()) {
        return Some((ghost_x.unwrap(), ghost_y.unwrap()));
    }
    None
}

fn render_ship(ship: &Ship, location: (f32, f32)) -> Vec<Point>
{
    let sin_t  = ship.direction.sin() * SHAPE_SCALE;
    let cos_t  = ship.direction.cos() * SHAPE_SCALE;
    let mut render = vec![Point::new(0, 0); ship.shape.len()];

    for i in (0..ship.shape.len()) {
        let x = (ship.shape[i].0 as f32 * cos_t) - (ship.shape[i].1 as f32 * sin_t);
        let y = (ship.shape[i].1 as f32 * cos_t) + (ship.shape[i].0 as f32 * sin_t);

        render[i] = Point::new(
            x.round() as i32 + (location.0 * COORD_SCALE).round() as i32,
            y.round() as i32 + (location.1 * COORD_SCALE).round() as i32
        );
    }
    render
}
