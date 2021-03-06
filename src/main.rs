extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Instant;
use std::f32::consts::{PI};

mod components;
mod entities;
mod renderer;

use components::physics;
use entities::asteroid::Asteroid;
use entities::ship::{Ship,Commands};

pub const TAU:           f32 = PI * 2.0;
pub const ARENA_WIDTH:   f32 = 1024.0;
pub const ARENA_HEIGHT:  f32 =  768.0;
pub const WINDOW_WIDTH:  u32 = 1024;
pub const WINDOW_HEIGHT: u32 =  768;
pub const NUM_ASTEROIDS_MIN: u8 =  4;
pub const NUM_ASTEROIDS_MAX: u8 = 11;

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
    let mut ship     = Ship::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, -PI / 2.0);
    let mut commands = Commands::new();
    let mut asteroids = Asteroid::generate_wave(NUM_ASTEROIDS_MIN);

    let mut prev_time = Instant::now();
    let mut cur_time  = Instant::now();
    let mut dt        = cur_time.duration_since(prev_time).as_millis() as f32;

    'running: loop {
        cur_time  = Instant::now();
        dt        = cur_time.duration_since(prev_time).as_millis() as f32;
        prev_time = cur_time;

        // Clear the momentary commands
        commands.fire        = false;
        commands.hyperspace  = false;
        commands.thrust_stop = false;

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
                Event::KeyUp   { keycode: Some(Keycode::Space), .. } => { commands.thrust  = false; commands.thrust_stop = true; },
                _ => {}
            }
        }
        ship.update(dt, &commands);
        physics::inertia(dt, &mut ship.physics);
        for b in &mut ship.bullets { physics::inertia(dt, &mut b.physics); }
        for a in &mut asteroids    { physics::inertia(dt, &mut a.physics); }
        renderer::render(&mut canvas, &mut ship, &asteroids).unwrap();
    }
    Ok(())
}
