extern crate sdl2;
use crate::components::*;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::*;
use sdl2::render::{WindowCanvas, Texture};

use specs::prelude::*;

pub const DIRECTION_UP:    f64 = 270.0;
pub const DIRECTION_DOWN:  f64 =  90.0;
pub const DIRECTION_LEFT:  f64 = 180.0;
pub const DIRECTION_RIGHT: f64 =   0.0;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(canvas: &mut WindowCanvas,
              buffer: &mut Texture,
              data: SystemData,
              spritesheet: &Texture) -> Result<(), String>
{
    canvas.with_texture_canvas(buffer, |b| {
        b.set_draw_color(Color::RGB(0, 0, 0));
        b.clear();

        for (pos, sprite) in (&data.0, &data.1).join() {
            let frame = Rect::from_center(Point::new(pos.x, pos.y), pos.w, pos.h);
            b.copy_ex(&spritesheet, Some(sprite.frame), Some(frame), pos.dir, None, false, false).unwrap();
        }
    }).map_err(|e| e.to_string())?;

    let b = buffer.query();
    canvas.copy_ex(&buffer, Rect::new(0, 0, b.width, b.height), canvas.viewport(), 0.0, None, false, false )?;

    canvas.present();
    Ok(())
}
