
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct Intersection {
    pub position: (i32, i32),
    pub size: u32,
    pub lane_width: u32,
}

impl Intersection {
    pub fn new(x: i32, y: i32, size: u32, lane_width: u32) -> Self {
        Intersection {
            position: (x, y),
            size,
            lane_width,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        // Draw main intersection box
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Rect::new(
            self.position.0,
            self.position.1,
            self.size,
            self.size,
        )).unwrap();

        // Draw road markings
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        
        // North road
        self.draw_road_vertical(canvas, self.position.0 + (self.size as i32 / 2) - ((self.lane_width * 3) as i32 / 2), 
                              self.position.1 - (self.size as i32), self.lane_width * 3, self.size as i32);
        
        // South road
        self.draw_road_vertical(canvas, self.position.0 + (self.size as i32 / 2) - ((self.lane_width * 3) as i32 / 2), 
                              self.position.1, self.lane_width * 3, self.size as i32);
        
        // West road
        self.draw_road_horizontal(canvas, self.position.0 - (self.size as i32), 
                                self.position.1 + (self.size as i32 / 2) - ((self.lane_width * 3) as i32 / 2), 
                                self.size as i32, self.lane_width * 3);
        
        // East road
        self.draw_road_horizontal(canvas, self.position.0, 
                                self.position.1 + (self.size as i32 / 2) - ((self.lane_width * 3) as i32 / 2), 
                                self.size as i32, self.lane_width * 3);

        // Draw lane separators
        self.draw_lane_markings(canvas);
    }

    fn draw_road_vertical(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, width: u32, height: i32) {
        canvas.fill_rect(Rect::new(x, y, width, height as u32)).unwrap();
    }

    fn draw_road_horizontal(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, width: i32, height: u32) {
        canvas.fill_rect(Rect::new(x, y, width as u32, height)).unwrap();
    }

    fn draw_lane_markings(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        
        // Vertical lane separators
        for i in 1..3 {
            let x = self.position.0 + (self.size as i32 / 2) - ((self.lane_width * 3) as i32 / 2) + (i * self.lane_width as i32);
            // North lanes
            canvas.fill_rect(Rect::new(x - 2, self.position.1 - (self.size as i32), 4, self.size as u32)).unwrap();
            // South lanes
            canvas.fill_rect(Rect::new(x - 2, self.position.1, 4, self.size as u32)).unwrap();
        }

        // Horizontal lane separators
        for i in 1..3 {
            let y = self.position.1 + (self.size as i32 / 2) - ((self.lane_width * 3) as i32 / 2) + (i * self.lane_width as i32);
            // West lanes
            canvas.fill_rect(Rect::new(self.position.0 - (self.size as i32), y - 2, self.size as u32, 4)).unwrap();
            // East lanes
            canvas.fill_rect(Rect::new(self.position.0, y - 2, self.size as u32, 4)).unwrap();
        }
    }
}