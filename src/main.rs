use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::image::LoadSurface;
use sdl2::image::LoadTexture;
use std::time::Duration;
#[allow(unused_imports)]
use std::time::Instant;

mod road;
//mod traffic_light;
mod vehicle;
mod world;

pub use world::World;
pub use smart_road::{draw_panel, draw_hud};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Fucking Road", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Load the sprite sheet into a Surface.
    let mut surface = Surface::from_file("./src/assets/sprite.png")
        .map_err(|e| e.to_string())?;
    // Set the color key so that white becomes transparent.
    // surface
    //     .set_color_key(true, Color::RGB(255, 255, 255))
    //     .map_err(|e| e.to_string())?;

    // Create a texture creator from the canvas and load the sprite sheet.
    let texture_creator = canvas.texture_creator();
    let sprite_texture = texture_creator
        .load_texture("./src/assets/sprite.png")
        .map_err(|e| e.to_string())?;
    // let sprite_texture = texture_creator
    //     .create_texture_from_surface(&surface)
    //     .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let sdl_context = sdl2::init().unwrap();
    // Initialize the TTF context here, and keep it alive for the whole program.
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut world = World::new(&sdl_context);
    let mut auto_spawning = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    // Pass the ttf_context to draw_panel.
                    draw_panel(&mut canvas, world.get_vehicles_passed(),
                    world.get_max_velocity(),
                    world.get_min_velocity(),
                    world.max_vehicles_time(),
                    world.min_vehicles_time(),
                    world.get_total_close_call_count(),
                    &ttf_context);
                    // Wait for user input to close
                    loop {
                        for event in event_pump.poll_iter() {
                            match event {
                                Event::Quit { .. } |
                                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                                    break 'running;
                                }
                                _ => {}
                            }
                        }
                        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
                    }
                    //break 'running
                },
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        Keycode::R => {
                            auto_spawning = !auto_spawning;
                        },
                        Keycode::Down if !auto_spawning => world.spawn_dir(1),
                        Keycode::Up if !auto_spawning => world.spawn_dir(0),
                        Keycode::Right if !auto_spawning => world.spawn_dir(3),
                        Keycode::Left if !auto_spawning => world.spawn_dir(2),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if auto_spawning {
            world.auto_spawn();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        //world.auto_spawn();
        world.update();
        world.draw(&mut canvas, &sprite_texture)?;
        draw_hud(&mut canvas, &ttf_context, auto_spawning);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}