use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;

use crate::lane::{Lane, Turn};

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Road {
    pub direction: Direction,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub lanes: Vec<Lane>,
}

impl Road {
    pub fn new(direction: Direction, x: i32, y: i32, width: u32, height: u32, lane_width: u32) -> Self {
        let turns = [Turn::UTurn, Turn::Left, Turn::SlightLeft, Turn::Straight, Turn::SlightRight, Turn::Right];
        let mut lanes = Vec::new();

        for (i, turn) in turns.iter().enumerate() {
            let lane_center = x + lane_width as i32 / 2 + i as i32 * lane_width as i32;
            let arrow_y = match direction {
                Direction::North => y + height as i32 - 90,
                Direction::South => y + 90,
                _ => y + lane_width as i32 / 2,
            };
            let arrow_x = match direction {
                Direction::East => x + 90,
                Direction::West => x + width as i32 - 90,
                _ => lane_center,
            };
            lanes.push(Lane::new(*turn, arrow_x, arrow_y, direction));
        }

        Road {
            direction,
            x,
            y,
            width,
            height,
            lanes,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, lane_width: u32, arrow_size: i16) {
        // Draw road background
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();

        // Draw lane markings
        let is_vertical = matches!(self.direction, Direction::North | Direction::South);
        for i in 0..=6 {
            let line_pos = if is_vertical {
                self.x + (i * lane_width as i32)
            } else {
                self.y + (i * lane_width as i32)
            };

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            if is_vertical {
                canvas.draw_line((line_pos, self.y), (line_pos, self.y + self.height as i32)).unwrap();
            } else {
                canvas.draw_line((self.x, line_pos), (self.x + self.width as i32, line_pos)).unwrap();
            }
        }

        // Draw road borders
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();

        // Draw lanes
        for lane in &self.lanes {
            lane.draw(canvas, arrow_size);
        }
    }
}