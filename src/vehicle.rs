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

// Add this new enum at the top of the file
#[derive(Clone, Copy)]
pub enum Lane {
    Right = 0,
    Middle = 1,
    Left = 2,
}

#[derive(Clone)]
pub struct Vehicle {
    x: f64,
    y: f64,
    angle: f64,
    direction: u8,
    // route: u8,
    lane: Lane,
    color: sdl2::pixels::Color,
}

#[allow(unused_variables, dead_code)]
impl Vehicle {
    /// Creates a new vehicle instance with specified starting position, angle, direction and route
    /// 
    /// # Arguments
    /// * `x` - Initial x coordinate position
    /// * `y` - Initial y coordinate position  
    /// * `angle` - Starting angle in degrees
    /// * `direction` - Vehicle direction (0: North, 1: South, 2: West, 3: East)
    /// * `lane` - Lane type (0: Straight, 1: Right turn, 2: Left turn)
    /// 
    /// # Returns
    /// New Vehicle instance with color based on route type:
    /// - Yellow for straight
    /// - Cyan for right turns
    /// - Purple for left turns
    pub fn new(x: i32, y: i32, angle: f64, direction: u8, lane: Lane) -> Self {
        let color = match lane {
            // 0 => sdl2::pixels::Color::RGB(255, 255, 0), // Straight: Yellow
            // 1 => sdl2::pixels::Color::RGB(0, 255, 255), // Right: Cyan
            // 2 => sdl2::pixels::Color::RGB(200, 150, 200), // Left: Purple
            Lane::Right => sdl2::pixels::Color::RGB(255, 255, 0),  // Yellow
            Lane::Middle => sdl2::pixels::Color::RGB(0, 255, 255), // Cyan
            Lane::Left => sdl2::pixels::Color::RGB(200, 150, 200), // Purple
            _ => unreachable!(),
        };

        Vehicle {
            x: x as f64,
            y: y as f64,
            angle,
            direction,
            // route,
            lane,
            color,
        }
    }

    /// Updates vehicle position based on current state and surrounding vehicles
    /// 
    /// # Arguments
    /// * `vehicles` - Slice containing all vehicles for collision detection
    /// 
    /// Calculates next position, checks if movement is possible, and updates position/angle
    /// if vehicle is in intersection
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

    /// Calculates movement vector based on current direction and intersection status
    /// 
    /// # Returns
    /// Tuple (dx, dy) representing movement delta:
    /// - In intersection: Uses angle-based vector calculation
    /// - Outside intersection: Uses fixed directional movement
    fn get_movement_vector(&self) -> (f64, f64) {
        // if self.is_in_intersection() {
        //     let rad = self.angle * PI / 90.0;
        //     let dx = VEHICLE_SPEED * rad.cos();
        //     let dy = VEHICLE_SPEED * rad.sin();
        //     return (dx, dy);
        // } else {
        //     match self.direction {
        //         0 => (0.0, -VEHICLE_SPEED), // North (up)
        //         1 => (0.0, VEHICLE_SPEED),  // South (down)
        //         2 => (-VEHICLE_SPEED, 0.0), // West (left)
        //         3 => (VEHICLE_SPEED, 0.0),  // East (right)
        //         _ => (0.0, 0.0),
        //     }
        // }
        if self.is_in_intersection() {
            let rad = self.angle * PI / 180.0;
            let lane_offset = match self.lane {
                Lane::Right => -20.0,
                Lane::Middle => 0.0,
                Lane::Left => 20.0,
            };
            
            let mut dx = VEHICLE_SPEED * rad.cos();
            let mut dy = VEHICLE_SPEED * rad.sin();
            
            // Apply lane offset only in intersection
            match self.direction {
                0 => dx += lane_offset * 0.1,
                1 => dx -= lane_offset * 0.1,
                2 => dy -= lane_offset * 0.1,
                3 => dy += lane_offset * 0.1,
                _ => (),
            }
            
            (dx, dy)
        } else {
            // Simple directional movement when not in intersection
            match self.direction {
                0 => (0.0, -VEHICLE_SPEED), // North
                1 => (0.0, VEHICLE_SPEED),  // South 
                2 => (-VEHICLE_SPEED, 0.0), // West
                3 => (VEHICLE_SPEED, 0.0),  // East
                _ => (0.0, 0.0),
            }
        }
    }

    /// Determines if vehicle can safely move to next position
    /// 
    /// # Arguments
    /// * `next_x` - Proposed next x coordinate
    /// * `next_y` - Proposed next y coordinate
    /// * `vehicles` - Slice of all vehicles for collision checking
    /// 
    /// # Returns
    /// Boolean indicating if movement is safe
    fn can_move(
        &self,
        next_x: f64,
        next_y: f64,
        vehicles: &[Vehicle],
    ) -> bool {
        if self.is_collision(next_x, next_y, vehicles) {
            return false;
        }

        true
    }

    /// Checks for potential collisions with other vehicles
    /// 
    /// # Arguments
    /// * `next_x` - Proposed next x coordinate
    /// * `next_y` - Proposed next y coordinate
    /// * `vehicles` - Slice of all vehicles
    /// 
    /// # Returns
    /// Boolean indicating if collision would occur:
    /// - Considers safety distance for vehicles ahead
    /// - Checks stopping distance for all nearby vehicles
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

    /// Determines if vehicle is in traffic light zone
    /// 
    /// # Arguments
    /// * `x` - Current x coordinate
    /// * `y` - Current y coordinate
    /// 
    /// # Returns
    /// Boolean indicating if vehicle is in traffic light decision zone
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

    /// Checks if vehicle is within intersection boundaries
    /// 
    /// # Returns
    /// Boolean indicating if vehicle is inside the intersection area
    /// defined by ROAD_WIDTH around intersection center (400, 300)
    fn is_in_intersection(&self) -> bool {
        // // Define intersection boundaries based on the full road width
        // let intersection_left = 400.0 - (ROAD_WIDTH as f64 * 0.75);   // Left boundary (400 - 240)
        // let intersection_right = 400.0 + (ROAD_WIDTH as f64 * 0.75);  // Right boundary (400 + 240)
        // let intersection_top = 300.0 - (ROAD_WIDTH as f64 * 0.75);    // Top boundary (300 - 240)
        // let intersection_bottom = 300.0 + (ROAD_WIDTH as f64 * 0.75); // Bottom boundary (300 + 240)

        // println!("Intersection boundaries Detected: Left: {}, Right: {}, Top: {}, Bottom: {}", intersection_left, intersection_right, intersection_top, intersection_bottom);

        // // Check if vehicle is within the intersection boundaries
        // self.x > intersection_left 
        //     && self.x < intersection_right
        //     && self.y > intersection_top 
        //     && self.y < intersection_bottom
        // Define intersection boundaries at road ends
        let intersection_left = 400.0 - (ROAD_WIDTH as f64 * 0.5);   
        let intersection_right = 400.0 + (ROAD_WIDTH as f64 * 0.5);  
        let intersection_top = 300.0 - (ROAD_WIDTH as f64 * 0.5);    
        let intersection_bottom = 300.0 + (ROAD_WIDTH as f64 * 0.5); 

        self.x > intersection_left 
            && self.x < intersection_right
            && self.y > intersection_top 
            && self.y < intersection_bottom
    }

    /// Determines if vehicle is approaching intersection
    /// 
    /// # Returns
    /// Boolean indicating if vehicle is within approach zone
    /// (2 lane widths before intersection)
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

    /// Updates vehicle angle for turning movements in intersection
    /// 
    /// Modifies vehicle angle based on:
    /// - Current direction
    /// - Turn speed (2.0 degrees per update)
    /// - Normalizes final angle to -180 to 180 degree range
    fn update_angle(&mut self) {
        // if !self.is_in_intersection() {
        //     return;
        // }
        // println!(
        //     "Vehicle at ({}, {}), Direction: {}, Route: {}, Current angle: {}",
        //     self.x, self.y, self.direction, self.route, self.angle
        // );
        // let turn_speed = 2.0;
        // let target_angle = match self.direction {
        //     0 => -90.0, // North to West
        //     1 => 90.0,  // South to East
        //     2 => 90.0,  // West to South
        //     3 => -90.0, // East to North
        //     _ => 0.0,
        // };

        // // Calculate turn center points based on direction
        // let (center_x, center_y) = match self.direction {
        //     0 => (400.0 - ROAD_WIDTH as f64 / 4.0, 300.0 - ROAD_WIDTH as f64 / 4.0),
        //     1 => (400.0 + ROAD_WIDTH as f64 / 4.0, 300.0 + ROAD_WIDTH as f64 / 4.0),
        //     2 => (400.0 - ROAD_WIDTH as f64 / 4.0, 300.0 + ROAD_WIDTH as f64 / 4.0),
        //     3 => (400.0 + ROAD_WIDTH as f64 / 4.0, 300.0 - ROAD_WIDTH as f64 / 4.0),
        //     _ => (self.x, self.y),
        // };

        // // Calculate distance from turn center
        // let dx = self.x - center_x;
        // let dy = self.y - center_y;
        // let distance = (dx * dx + dy * dy).sqrt();

        // // Adjust angle based on position relative to turn center
        // if distance > TURNING_RADIUS {
        //     match self.direction {
        //         0 => if self.angle > target_angle { self.angle -= turn_speed },
        //         1 => if self.angle < target_angle { self.angle += turn_speed },
        //         2 => if self.angle < target_angle { self.angle += turn_speed },
        //         3 => if self.angle > target_angle { self.angle -= turn_speed },
        //         _ => (),
        //     }
        // }

        // // Fixed turning angles based on entry direction
        // match self.direction {
        //     0 => { // From North to East
        //         if self.angle < 90.0 {
        //             self.angle += turn_speed;
        //         }
        //     },
        //     1 => { // From South to West
        //         if self.angle > -90.0 {
        //             self.angle -= turn_speed;
        //         }
        //     },
        //     2 => { // From West to North
        //         if self.angle > -90.0 {
        //             self.angle -= turn_speed;
        //         }
        //     },
        //     3 => { // From East to South
        //         if self.angle < 90.0 {
        //             self.angle += turn_speed;
        //         }
        //     },
        //     _ => (),
        // }

        // match self.direction {
        //     0 => { // Moving North
        //         // Left turn to West
        //         if self.angle > -90.0 {
        //             self.angle -= turn_speed;
        //         }
        //     }
        //     1 => { // Moving South
        //         // Left turn to East
        //         if self.angle > 0.0 {
        //             self.angle -= turn_speed;
        //         }
        //     }
        //     2 => { // Moving West
        //         // Left turn to South
        //         if self.angle < 90.0 {
        //             self.angle += turn_speed;
        //         }
        //     }
        //     3 => { // Moving East
        //         // Left turn to North
        //         if self.angle > -90.0 {
        //             self.angle -= turn_speed;
        //         }
        //     }
        //     _ => (),
        // }
        if !self.is_in_intersection() {
            return;
        }
        let turn_speed = 2.0;
        let target_angle = match (self.direction, self.lane) {
            // From North (moving south)
            (1, Lane::Right) => 0.0,    // Turn right to East
            (1, Lane::Middle) => 90.0,  // Continue South
            (1, Lane::Left) => 180.0,   // Turn left to West
            
            // From East (moving west)
            (2, Lane::Right) => 90.0,   // Turn right to South
            (2, Lane::Middle) => 180.0, // Continue West
            (2, Lane::Left) => 270.0,   // Turn left to North
            
            // From South (moving north)
            (0, Lane::Right) => 180.0,  // Turn right to West
            (0, Lane::Middle) => 270.0, // Continue North
            (0, Lane::Left) => 0.0,     // Turn left to East
            
            // From West (moving east)
            (3, Lane::Right) => 270.0,  // Turn right to North
            (3, Lane::Middle) => 0.0,   // Continue East
            (3, Lane::Left) => 90.0,    // Turn left to South
            
            _ => self.angle,
        };

        // Smooth angle transition
        let angle_diff = target_angle - self.angle;
        if angle_diff.abs() > turn_speed {
            if angle_diff > 0.0 {
                self.angle += turn_speed;
            } else {
                self.angle -= turn_speed;
            }
        }

        // check to remove
        // Normalize angle to stay within -180 to 180 degrees
        self.angle = self.angle.rem_euclid(360.0);
        // if self.angle > 180.0 {
        //     self.angle -= 360.0;
        // }
    }

    /// Checks if vehicle has completed its journey
    /// 
    /// # Returns
    /// Boolean indicating if vehicle has left the simulation bounds
    pub fn is_finished(&self) -> bool {
        self.x < 0.0 || self.x > ROAD_HEIGHT as f64 || self.y < 0.0 || self.y > ROAD_HEIGHT as f64
    }

    /// Renders vehicle on the canvas
    /// 
    /// # Arguments
    /// * `canvas` - SDL2 canvas to draw on
    /// 
    /// # Returns
    /// Result indicating if drawing was successful
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
