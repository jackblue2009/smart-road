use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use crate::road::Direction;

#[derive(Clone, Copy)]
pub enum Turn {
    Left,
    SlightLeft,
    Straight,
    SlightRight,
    Right,
    UTurn,
}

pub struct Lane {
    pub turn: Turn,
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

impl Lane {
    pub fn new(turn: Turn, x: i32, y: i32, direction: Direction) -> Self {
        Lane {
            turn,
            x,
            y,
            direction,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, arrow_size: i16) {
        draw_arrow(canvas, self.x, self.y, &self.direction, &self.turn, arrow_size);
    }
}

pub fn draw_arrow(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, direction: &Direction, turn: &Turn, arrow_size: i16) {
    let (dx, dy) = match (direction, turn) {
        (Direction::North, Turn::UTurn) => (-arrow_size, arrow_size),
        (Direction::North, Turn::Left) => (-arrow_size, -arrow_size),
        (Direction::North, Turn::SlightLeft) => (-arrow_size/2, -arrow_size),
        (Direction::North, Turn::Straight) => (0, -arrow_size),
        (Direction::North, Turn::SlightRight) => (arrow_size/2, -arrow_size),
        (Direction::North, Turn::Right) => (arrow_size, -arrow_size),
        (Direction::South, Turn::UTurn) => (arrow_size, -arrow_size),
        (Direction::South, Turn::Left) => (arrow_size, arrow_size),
        (Direction::South, Turn::SlightLeft) => (arrow_size/2, arrow_size),
        (Direction::South, Turn::Straight) => (0, arrow_size),
        (Direction::South, Turn::SlightRight) => (-arrow_size/2, arrow_size),
        (Direction::South, Turn::Right) => (-arrow_size, arrow_size),
        (Direction::East, Turn::UTurn) => (-arrow_size, -arrow_size),
        (Direction::East, Turn::Left) => (arrow_size, -arrow_size),
        (Direction::East, Turn::SlightLeft) => (arrow_size, -arrow_size/2),
        (Direction::East, Turn::Straight) => (arrow_size, 0),
        (Direction::East, Turn::SlightRight) => (arrow_size, arrow_size/2),
        (Direction::East, Turn::Right) => (arrow_size, arrow_size),
        (Direction::West, Turn::UTurn) => (arrow_size, arrow_size),
        (Direction::West, Turn::Left) => (-arrow_size, arrow_size),
        (Direction::West, Turn::SlightLeft) => (-arrow_size, arrow_size/2),
        (Direction::West, Turn::Straight) => (-arrow_size, 0),
        (Direction::West, Turn::SlightRight) => (-arrow_size, -arrow_size/2),
        (Direction::West, Turn::Right) => (-arrow_size, -arrow_size),
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