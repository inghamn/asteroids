extern crate sdl2;
use crate::components::*;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::*;
use sdl2::render::{WindowCanvas, Texture};

use specs::prelude::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(canvas: &mut WindowCanvas, data: SystemData, spritesheet: &Texture) -> Result<(), String>
{
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for (position, sprite) in (&data.0, &data.1).join() {
        let frame = Rect::from_center(Point::new(position.x, position.y), 34, 21);
        canvas.copy_ex(&spritesheet, Some(sprite.frame), Some(frame), 0.0, None, false, false)?;
    }

    canvas.present();
    Ok(())
}
