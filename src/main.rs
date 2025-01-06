use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

mod intersect;
use intersect::Intersection;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const LANE_WIDTH: u32 = 50;
const LANES_PER_ROAD: u32 = 6;
const ROAD_WIDTH: u32 = LANE_WIDTH * LANES_PER_ROAD;
const INTERSECTION_SIZE: u32 = ROAD_WIDTH * 2;
const ARROW_SIZE: i16 = 30;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Smart Fucking Road", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let intersection = Intersection::new(
        (WINDOW_WIDTH as i32 / 2) - (INTERSECTION_SIZE as i32 / 2),
        (WINDOW_HEIGHT as i32 / 2) - (INTERSECTION_SIZE as i32 / 2),
        INTERSECTION_SIZE,
        LANE_WIDTH
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        intersection.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
