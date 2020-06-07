extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::image::{LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use specs::prelude::*;

mod components;
mod renderer;
use crate::components::*;

fn main() -> Result<(), String> {
    let sdl    = sdl2::init()?;
    let video  = sdl.video()?;
    let window = video.window("Something", 640, 480)
                      .opengl()
                      .build().map_err(|e| e.to_string())?;

    let mut events  = sdl.event_pump()?;
    let mut canvas  = window.into_canvas().build().map_err(|e| e.to_string())?;
    let textures    = canvas.texture_creator();
    let spritesheet = textures.load_texture(Path::new("resources/sprites.png"))?;

    let mut game   = GameState { world: World::new() };

    game.world.register::<Position>();
    game.world.register::<Sprite>();
    game.world.register::<Velocity>();
    game.world.create_entity()
              .with(Position { x: 320, y: 240})
              .with(Velocity { x: 0,   y: 0  })
              .with(Sprite {frame: Rect::new(0, 0, 56, 34)})
              .build();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        renderer::render(&mut canvas, game.world.system_data(), &spritesheet).unwrap();
    }

    Ok(())
}

struct GameState {
    world: World
}
