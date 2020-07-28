extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Instant;

mod components;
mod inputs;
mod renderer;

const ARENA_WIDTH:   u32 = 1280;
const ARENA_HEIGHT:  u32 = 1024;
const WINDOW_WIDTH:  u32 = 1280;
const WINDOW_HEIGHT: u32 = 1024;


fn main() -> Result<(), String> {
    let sdl    = sdl2::init()?;
    let video  = sdl.video()?;
    let window = video.window("Something", WINDOW_WIDTH, WINDOW_HEIGHT)
                      .opengl()
                      .build().map_err(|e| e.to_string())?;
    let mut events  = sdl.event_pump()?;
    let mut canvas  = window.into_canvas()
                            .present_vsync()
                            .build()
                            .map_err(|e| e.to_string())?;
    let mut ship     = components::Ship::new();
    let mut commands = inputs::Commands::new();

    let mut previousTime = Instant::now();
    let mut currentTime  = Instant::now();
    let mut dt           = currentTime.duration_since(previousTime);

    'running: loop {
        currentTime  = Instant::now();
        dt           = currentTime.duration_since(previousTime);
        previousTime = currentTime;

        // Clear the momentary commands
        commands.fire       = false;
        commands.hyperspace = false;

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Momentary commands
                Event::KeyDown { keycode: Some(Keycode::LCtrl), .. } => { commands.fire       = true;  },
                Event::KeyDown { keycode: Some(Keycode::Tab  ), .. } => { commands.hyperspace = true;  },

                // Continuous commands
                Event::KeyDown { keycode: Some(Keycode::Left ), .. } => { commands.left    = true;  },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => { commands.right   = true;  },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => { commands.thrust  = true;  },
                Event::KeyUp   { keycode: Some(Keycode::Left ), .. } => { commands.left    = false; },
                Event::KeyUp   { keycode: Some(Keycode::Right), .. } => { commands.right   = false; },
                Event::KeyUp   { keycode: Some(Keycode::Space), .. } => { commands.thrust  = false; },
                _ => {}
            }
        }
        inputs::update(&commands, &mut ship);
        renderer::render(&mut canvas, &mut ship).unwrap();
    }
    Ok(())
}

