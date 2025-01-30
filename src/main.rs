use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
#[allow(unused_imports)]
use std::time::Instant;

mod road;
//mod traffic_light;
mod vehicle;
mod world;

use world::World;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Fucking Road", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut world = World::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        match keycode {
                            Keycode::Down => world.spawn_dir(1),
                            Keycode::Up => world.spawn_dir(0),
                            Keycode::Right => world.spawn_dir(3),
                            Keycode::Left => world.spawn_dir(2),
                            Keycode::R => world.auto_spawn(),
                            _ => {}
                        }
                    }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        //world.auto_spawn();
        world.update();
        world.draw(&mut canvas)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
