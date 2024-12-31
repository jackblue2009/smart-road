mod road;
mod lane;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

use road::{Road, Direction};
use lane::{Turn, draw_arrow};

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
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

    // let roads = [
    //     Road { direction: Direction::North, x: (WINDOW_WIDTH / 2 - ROAD_WIDTH) as i32, y: 0, width: ROAD_WIDTH, height: WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2 },
    //     Road { direction: Direction::South, x: (WINDOW_WIDTH / 2 - ROAD_WIDTH) as i32, y: (WINDOW_HEIGHT / 2 + INTERSECTION_SIZE / 2) as i32, width: ROAD_WIDTH, height: WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2 },
    //     Road { direction: Direction::East, x: (WINDOW_WIDTH / 2 + INTERSECTION_SIZE / 2) as i32, y: (WINDOW_HEIGHT / 2 - ROAD_WIDTH) as i32, width: WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2, height: ROAD_WIDTH },
    //     Road { direction: Direction::West, x: 0, y: (WINDOW_HEIGHT / 2 - ROAD_WIDTH) as i32, width: WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2, height: ROAD_WIDTH },
    // ];

    let roads = [
        Road::new(Direction::North, (WINDOW_WIDTH / 2 - ROAD_WIDTH) as i32, 0, ROAD_WIDTH * 2, WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2, LANE_WIDTH),
        Road::new(Direction::South, (WINDOW_WIDTH / 2 - ROAD_WIDTH) as i32, (WINDOW_HEIGHT / 2 + INTERSECTION_SIZE / 2) as i32, ROAD_WIDTH * 2, WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2, LANE_WIDTH),
        Road::new(Direction::East, (WINDOW_WIDTH / 2 + INTERSECTION_SIZE / 2) as i32, (WINDOW_HEIGHT / 2 - ROAD_WIDTH) as i32, WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2, ROAD_WIDTH * 2, LANE_WIDTH),
        Road::new(Direction::West, 0, (WINDOW_HEIGHT / 2 - ROAD_WIDTH) as i32, WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2, ROAD_WIDTH * 2, LANE_WIDTH),
    ];

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

        draw_intersection(&mut canvas);

        for road in &roads {
            road.draw(&mut canvas, LANE_WIDTH, ARROW_SIZE);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn draw_road_arrows(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, road: &Road) {
    let turns = [Turn::UTurn, Turn::Left, Turn::SlightLeft, Turn::Straight, Turn::SlightRight, Turn::Right];
    for (i, turn) in turns.iter().enumerate() {
        let lane_center = road.x + LANE_WIDTH as i32 / 2 + i as i32 * LANE_WIDTH as i32;
        let arrow_y = match road.direction {
            Direction::North => road.y + road.height as i32 - ARROW_SIZE as i32 * 3,
            Direction::South => road.y + ARROW_SIZE as i32 * 3,
            _ => road.y + LANE_WIDTH as i32 / 2,
        };
        let arrow_x = match road.direction {
            Direction::East => road.x + ARROW_SIZE as i32 * 3,
            Direction::West => road.x + road.width as i32 - ARROW_SIZE as i32 * 3,
            _ => lane_center,
        };
        draw_arrow(canvas, arrow_x, arrow_y, &road.direction, turn, ARROW_SIZE);
    }
}

fn draw_intersection(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let intersection_rect = Rect::new(
        (WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2) as i32,
        (WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2) as i32,
        INTERSECTION_SIZE,
        INTERSECTION_SIZE
    );

    canvas.set_draw_color(Color::RGB(80, 80, 80));
    canvas.fill_rect(intersection_rect).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(intersection_rect).unwrap();
}
