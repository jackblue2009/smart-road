
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const LANE_WIDTH: u32 = 40;
const ROAD_WIDTH: u32 = LANE_WIDTH * 3;
const INTERSECTION_SIZE: u32 = ROAD_WIDTH * 2;
const ARROW_SIZE: i16 = 20;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    SlightLeft,
    Straight,
    SlightRight,
    Right,
    UTurn,
}

struct Road {
    direction: Direction,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Traffic World", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let roads = [
        Road { direction: Direction::North, x: (WINDOW_WIDTH / 2 - ROAD_WIDTH) as i32, y: 0, width: ROAD_WIDTH, height: WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2 },
        Road { direction: Direction::South, x: (WINDOW_WIDTH / 2) as i32, y: (WINDOW_HEIGHT / 2 + INTERSECTION_SIZE / 2) as i32, width: ROAD_WIDTH, height: WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2 },
        Road { direction: Direction::East, x: (WINDOW_WIDTH / 2 + INTERSECTION_SIZE / 2) as i32, y: (WINDOW_HEIGHT / 2 - ROAD_WIDTH) as i32, width: WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2, height: ROAD_WIDTH },
        Road { direction: Direction::West, x: 0, y: (WINDOW_HEIGHT / 2) as i32, width: WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2, height: ROAD_WIDTH },
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

        canvas.set_draw_color(Color::RGB(0, 100, 0));
        canvas.clear();

        draw_intersection(&mut canvas);

        for road in &roads {
            draw_road(&mut canvas, road);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn draw_road(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, road: &Road) {
    // Draw road background
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.fill_rect(Rect::new(road.x, road.y, road.width, road.height)).unwrap();

    // Draw lane markings
    let is_vertical = matches!(road.direction, Direction::North | Direction::South);
    for i in 1..6 {
        let line_pos = if is_vertical {
            road.x + (i * LANE_WIDTH as i32)
        } else {
            road.y + (i * LANE_WIDTH as i32)
        };

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        if is_vertical {
            canvas.draw_line((line_pos, road.y), (line_pos, road.y + road.height as i32)).unwrap();
        } else {
            canvas.draw_line((road.x, line_pos), (road.x + road.width as i32, line_pos)).unwrap();
        }
    }

    // Draw road borders
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(Rect::new(road.x, road.y, road.width, road.height)).unwrap();

    // Draw arrows
    let turns = [Turn::UTurn, Turn::Left, Turn::SlightLeft, Turn::Straight, Turn::SlightRight, Turn::Right];
    for (i, turn) in turns.iter().enumerate() {
        let lane_center = road.x + LANE_WIDTH as i32 / 2 + i as i32 * LANE_WIDTH as i32;
        let arrow_y = match road.direction {
            Direction::North => road.y + road.height as i32 - ARROW_SIZE as i32 * 2,
            Direction::South => road.y + ARROW_SIZE as i32 * 2,
            _ => road.y + LANE_WIDTH as i32 / 2,
        };
        let arrow_x = match road.direction {
            Direction::East => road.x + ARROW_SIZE as i32 * 2,
            Direction::West => road.x + road.width as i32 - ARROW_SIZE as i32 * 2,
            _ => lane_center,
        };
        draw_arrow(canvas, arrow_x, arrow_y, &road.direction, turn);
    }
}

fn draw_intersection(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let intersection_rect = Rect::new(
        (WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2) as i32,
        (WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2) as i32,
        INTERSECTION_SIZE,
        INTERSECTION_SIZE
    );

    // Draw intersection background
    canvas.set_draw_color(Color::RGB(80, 80, 80));
    canvas.fill_rect(intersection_rect).unwrap();

    // Draw intersection border
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(intersection_rect).unwrap();
}

fn draw_arrow(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, direction: &Direction, turn: &Turn) {
    let (dx, dy) = match (direction, turn) {
        (Direction::North, Turn::UTurn) => (-ARROW_SIZE * 2, 0),
        (Direction::North, Turn::Left) => (-ARROW_SIZE * 2, -ARROW_SIZE * 2),
        (Direction::North, Turn::SlightLeft) => (-ARROW_SIZE, -ARROW_SIZE * 2),
        (Direction::North, Turn::Straight) => (0, -ARROW_SIZE * 2),
        (Direction::North, Turn::SlightRight) => (ARROW_SIZE, -ARROW_SIZE * 2),
        (Direction::North, Turn::Right) => (ARROW_SIZE * 2, -ARROW_SIZE * 2),
        (Direction::South, Turn::UTurn) => (ARROW_SIZE * 2, 0),
        (Direction::South, Turn::Left) => (ARROW_SIZE * 2, ARROW_SIZE * 2),
        (Direction::South, Turn::SlightLeft) => (ARROW_SIZE, ARROW_SIZE * 2),
        (Direction::South, Turn::Straight) => (0, ARROW_SIZE * 2),
        (Direction::South, Turn::SlightRight) => (-ARROW_SIZE, ARROW_SIZE * 2),
        (Direction::South, Turn::Right) => (-ARROW_SIZE * 2, ARROW_SIZE * 2),
        (Direction::East, Turn::UTurn) => (0, -ARROW_SIZE * 2),
        (Direction::East, Turn::Left) => (ARROW_SIZE * 2, -ARROW_SIZE * 2),
        (Direction::East, Turn::SlightLeft) => (ARROW_SIZE * 2, -ARROW_SIZE),
        (Direction::East, Turn::Straight) => (ARROW_SIZE * 2, 0),
        (Direction::East, Turn::SlightRight) => (ARROW_SIZE * 2, ARROW_SIZE),
        (Direction::East, Turn::Right) => (ARROW_SIZE * 2, ARROW_SIZE * 2),
        (Direction::West, Turn::UTurn) => (0, ARROW_SIZE * 2),
        (Direction::West, Turn::Left) => (-ARROW_SIZE * 2, ARROW_SIZE * 2),
        (Direction::West, Turn::SlightLeft) => (-ARROW_SIZE * 2, ARROW_SIZE),
        (Direction::West, Turn::Straight) => (-ARROW_SIZE * 2, 0),
        (Direction::West, Turn::SlightRight) => (-ARROW_SIZE * 2, -ARROW_SIZE),
        (Direction::West, Turn::Right) => (-ARROW_SIZE * 2, -ARROW_SIZE * 2),
    };

    let (x1, y1) = (x as i16, y as i16);
    let (x2, y2) = (x as i16 + dx, y as i16 + dy);

    canvas.thick_line(x1, y1, x2, y2, 2, Color::RGB(255, 255, 0)).unwrap();

    let angle = (dy as f64).atan2(dx as f64);
    let arrow_angle = std::f64::consts::PI / 6.0;
    let arrow_length = 10;

    let x3 = x2 - (arrow_length as f64 * (angle + arrow_angle).cos()) as i16;
    let y3 = y2 - (arrow_length as f64 * (angle + arrow_angle).sin()) as i16;
    let x4 = x2 - (arrow_length as f64 * (angle - arrow_angle).cos()) as i16;
    let y4 = y2 - (arrow_length as f64 * (angle - arrow_angle).sin()) as i16;

    canvas.thick_line(x2, y2, x3, y3, 2, Color::RGB(255, 255, 0)).unwrap();
    canvas.thick_line(x2, y2, x4, y4, 2, Color::RGB(255, 255, 0)).unwrap();
}

