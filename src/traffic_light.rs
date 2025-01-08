use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const LIGHT_SIZE: u32 = 20;
const CYCLE_DURATION: u32 = 300; // 5 seconds at 60 FPS

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct TrafficLight {
    x: i32,
    y: i32,
    timer: u32,
    is_green: bool,
    offset: u32,
}

impl TrafficLight {
    pub fn new(x: i32, y: i32, offset: u32) -> Self {
        TrafficLight {
            x,
            y,
            timer: 0,
            is_green: false,
            offset,
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
        if self.timer >= CYCLE_DURATION {
            self.timer = 0;
            self.is_green = !self.is_green;
        }
    }

    pub fn is_green(&self) -> bool {
        self.is_green
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let color = if self.is_green {
            sdl2::pixels::Color::RGB(0, 255, 0)
        } else {
            sdl2::pixels::Color::RGB(255, 0, 0)
        };

        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(self.x, self.y, LIGHT_SIZE, LIGHT_SIZE))
    }
}
