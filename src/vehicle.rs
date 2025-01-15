use crate::road::{ROAD_HEIGHT, ROAD_WIDTH};
// use crate::traffic_light::TrafficLight;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;

const VEHICLE_SIZE: u32 = 20;
const VEHICLE_SPEED: f64 = 2.0;
const SAFETY_DISTANCE: f64 = 15.0;
const STOPPING_DISTANCE: f64 = 30.0; // Distance at which to start slowing down

#[allow(dead_code)]
const TURNING_RADIUS: f64 = 30.0;
#[allow(dead_code)]
const INTERSECTION_CENTER_X: f64 = 400.0;
#[allow(dead_code)]
const INTERSECTION_CENTER_Y: f64 = 300.0;

#[derive(Clone)]
pub struct Vehicle {
    x: f64,
    y: f64,
    angle: f64,
    direction: u8,
    route: u8,
    color: sdl2::pixels::Color,
}

#[allow(unused_variables, dead_code)]
impl Vehicle {
    pub fn new(x: i32, y: i32, angle: f64, direction: u8, route: u8) -> Self {
        let color = match route {
            0 => sdl2::pixels::Color::RGB(255, 255, 0), // Straight: Yellow
            1 => sdl2::pixels::Color::RGB(0, 255, 255), // Right: Cyan
            2 => sdl2::pixels::Color::RGB(200, 150, 200), // Left: Purple
            _ => unreachable!(),
        };

        Vehicle {
            x: x as f64,
            y: y as f64,
            angle,
            direction,
            route,
            color,
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        let (dx, dy) = self.get_movement_vector();
        let next_x = self.x + dx;
        let next_y = self.y + dy;

        if self.can_move(next_x, next_y, vehicles) {
            self.x = next_x;
            self.y = next_y;

            if self.is_in_intersection() {
                self.update_angle();
            }
        }
    }

    fn get_movement_vector(&self) -> (f64, f64) {
        if self.is_in_intersection() {
            let rad = self.angle * PI / 180.0;
            let dx = VEHICLE_SPEED * rad.cos();
            let dy = VEHICLE_SPEED * rad.sin();
            return (dx, dy);
        } else {
            match self.direction {
                0 => (0.0, -VEHICLE_SPEED), // North (up)
                1 => (0.0, VEHICLE_SPEED),  // South (down)
                2 => (-VEHICLE_SPEED, 0.0), // West (left)
                3 => (VEHICLE_SPEED, 0.0),  // East (right)
                _ => (0.0, 0.0),
            }
        }
    }

    fn can_move(
        &self,
        next_x: f64,
        next_y: f64,
        vehicles: &[Vehicle],
    ) -> bool {
        //println!("Checking movement to position: ({}, {})", next_x, next_y);
        // if self.is_collision(next_x, next_y, vehicles) {
        //     println!("Movement blocked by collision");
        //     return false;
        // }
        if self.is_collision(next_x, next_y, vehicles) {
            return false;
        }

        true
    }

    fn is_collision(&self, next_x: f64, next_y: f64, vehicles: &[Vehicle]) -> bool {
        for other in vehicles {
            if std::ptr::eq(self, other) {
                continue;
            }
            let dx = next_x - other.x;
            let dy = next_y - other.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Check if vehicle is ahead in the same direction
            let is_ahead = match self.direction {
                0 => other.y < self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64, // Moving north
                1 => other.y > self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64, // Moving south
                2 => other.x < self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64, // Moving west
                3 => other.x > self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64, // Moving east
                _ => false,
            };

            println!(
                "Self pos: ({}, {}), Other pos: ({}, {}), Distance: {}",
                next_x, next_y, other.x, other.y, distance
            );

            if is_ahead && distance < SAFETY_DISTANCE {
                return true;
            }

            if distance < STOPPING_DISTANCE {
                return true;
            }
        }
        false
    }

    fn is_at_traffic_light(&self, x: f64, y: f64) -> bool {
        let light_zone = ROAD_WIDTH as f64 / 2.0;
        match self.direction {
            0 => y <= 300.0 + light_zone && y > 300.0 - light_zone, // Northbound
            1 => y >= 300.0 - light_zone && y < 300.0 + light_zone, // Southbound
            2 => x <= 400.0 + light_zone && x > 400.0 - light_zone, // Westbound
            3 => x >= 400.0 - light_zone && x < 400.0 + light_zone, // Eastbound
            _ => unreachable!(),
        }
    }

    fn is_in_intersection(&self) -> bool {
        // self.x > 400.0 - ROAD_WIDTH as f64 / 2.0
        //     && self.x < 400.0 + ROAD_WIDTH as f64 / 2.0
        //     && self.y > 300.0 - ROAD_WIDTH as f64 / 2.0
        //     && self.y < 300.0 + ROAD_WIDTH as f64 / 2.0
        // Each direction has 3 lanes, total of 6 lanes per road
        // ROAD_WIDTH = 240 means each lane is 40 units wide (240/6)
        
        // Define intersection boundaries based on the full road width
        let intersection_left = 400.0 - ROAD_WIDTH as f64;   // Left boundary (400 - 240)
        let intersection_right = 400.0 + ROAD_WIDTH as f64;  // Right boundary (400 + 240)
        let intersection_top = 300.0 - ROAD_WIDTH as f64;    // Top boundary (300 - 240)
        let intersection_bottom = 300.0 + ROAD_WIDTH as f64; // Bottom boundary (300 + 240)

        // Check if vehicle is within the intersection boundaries
        self.x > intersection_left 
            && self.x < intersection_right
            && self.y > intersection_top 
            && self.y < intersection_bottom
    }

    fn is_approaching_intersection(&self) -> bool {
        let lane_width = ROAD_WIDTH as f64 / 6.0; // Each lane is 40 units wide
        let approach_distance = lane_width * 2.0;  // Two lane widths of approach distance
        
        match self.direction {
            0 => self.y > 300.0 + ROAD_WIDTH as f64 && self.y < 300.0 + ROAD_WIDTH as f64 + approach_distance, // Northbound
            1 => self.y < 300.0 - ROAD_WIDTH as f64 && self.y > 300.0 - ROAD_WIDTH as f64 - approach_distance, // Southbound
            2 => self.x > 400.0 + ROAD_WIDTH as f64 && self.x < 400.0 + ROAD_WIDTH as f64 + approach_distance, // Westbound
            3 => self.x < 400.0 - ROAD_WIDTH as f64 && self.x > 400.0 - ROAD_WIDTH as f64 - approach_distance, // Eastbound
            _ => false,
        }
    }

    fn update_angle(&mut self) {
        // if self.route == 1 { // Right turn
        //     self.angle += 2.0;  // Faster turn rate
        // } else if self.route == 2 { // Left turn
        //     self.angle -= 2.0;  // Faster turn rate
        // }
        // self.angle = self.angle.rem_euclid(360.0);
        if !self.is_in_intersection() {
            return;
        }

        println!(
            "Vehicle at ({}, {}), Direction: {}, Route: {}, Current angle: {}",
            self.x, self.y, self.direction, self.route, self.angle
        );
        let turn_speed = 2.0;

        match self.direction {
            // 0 => {
            //     match self.route {
            //         0 => (), // Straight: maintain -90 degrees
            //         1 => {
            //             // Right turn to East: -90 to 0 degrees
            //             if self.angle < 0.0 {
            //                 self.angle += turn_speed;
            //             }
            //         }
            //         2 => {
            //             // Left turn to West: -90 to 180 degrees
            //             if self.angle > -180.0 {
            //                 self.angle -= turn_speed;
            //             }
            //         }
            //         _ => unreachable!(),
            //     }
            // } // North
            // 1 => {
            //     match self.route {
            //         0 => (), // Straight: maintain 90 degrees
            //         1 => {
            //             // Right turn to West: 90 to 180 degrees
            //             if self.angle < 180.0 {
            //                 self.angle += turn_speed;
            //             }
            //         }
            //         2 => {
            //             // Left turn to East: 90 to 0 degrees
            //             if self.angle > 0.0 {
            //                 self.angle -= turn_speed;
            //             }
            //         }
            //         _ => unreachable!(),
            //     }
            // } // South
            // 2 => {
            //     match self.route {
            //         0 => (), // Straight: maintain 180 degrees
            //         1 => {
            //             // Right turn to North: 180 to -90 degrees
            //             if self.angle > -90.0 {
            //                 self.angle -= turn_speed;
            //             }
            //         }
            //         2 => {
            //             // Left turn to South: 180 to 90 degrees
            //             if self.angle > 90.0 {
            //                 self.angle -= turn_speed;
            //             }
            //         }
            //         _ => unreachable!(),
            //     }
            // } // East
            // 3 => {
            //     match self.route {
            //         0 => (), // Straight: maintain 0 degrees
            //         1 => {
            //             // Right turn to South: 0 to 90 degrees
            //             if self.angle < 90.0 {
            //                 self.angle += turn_speed;
            //             }
            //         }
            //         2 => {
            //             // Left turn to North: 0 to -90 degrees
            //             if self.angle > -90.0 {
            //                 self.angle -= turn_speed;
            //             }
            //         }
            //         _ => unreachable!(),
            //     }
            // } // West
            // _ => (),
            0 => { // Moving North
                // Left turn to West
                if self.angle > -180.0 {
                    self.angle -= turn_speed;
                }
            }
            1 => { // Moving South
                // Left turn to East
                if self.angle > 0.0 {
                    self.angle -= turn_speed;
                }
            }
            2 => { // Moving West
                // Left turn to South
                if self.angle < 90.0 {
                    self.angle += turn_speed;
                }
            }
            3 => { // Moving East
                // Left turn to North
                if self.angle > -90.0 {
                    self.angle -= turn_speed;
                }
            }
            _ => (),
        }

        // check to remove
        // Normalize angle to stay within -180 to 180 degrees
        self.angle = self.angle.rem_euclid(360.0);
        if self.angle > 180.0 {
            self.angle -= 360.0;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.x < 0.0 || self.x > ROAD_HEIGHT as f64 || self.y < 0.0 || self.y > ROAD_HEIGHT as f64
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let rect = Rect::new(
            self.x as i32 - VEHICLE_SIZE as i32 / 2,
            self.y as i32 - VEHICLE_SIZE as i32 / 2,
            VEHICLE_SIZE,
            VEHICLE_SIZE,
        );
        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect)
    }
}
