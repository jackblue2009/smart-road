use std::sync::atomic::{AtomicU32, Ordering};

use crate::road::{ROAD_HEIGHT, ROAD_WIDTH};
// use crate::traffic_light::TrafficLight;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;
use rand::Rng;

static VEHICLE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

const VEHICLE_SIZE: u32 = 30;
const VEHICLE_SPEED: f64 = 2.0;
const SAFETY_DISTANCE: f64 = 40.0;
const STOPPING_DISTANCE: f64 = 30.0; // Distance at which to start slowing down

const NORTH_STOP_POS: f64 = 158.0;
const SOUTH_STOP_POS: f64 = 440.0;
const WEST_STOP_POS: f64 = 260.0;
const EAST_STOP_POS: f64 = 540.0;

// #[allow(dead_code)]
// const TURNING_RADIUS: f64 = 30.0;
// #[allow(dead_code)]
// const INTERSECTION_CENTER_X: f64 = 400.0;
// #[allow(dead_code)]
// const INTERSECTION_CENTER_Y: f64 = 300.0;

// Add this new enum at the top of the file
#[derive(Clone, Copy, PartialEq)]
pub enum Lane {
    Middle = 0,
    Right = 1,
    Left = 2,
}

#[derive(Clone)]
pub struct Vehicle {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub direction: u8,
    pub lane: Lane,
    pub color: sdl2::pixels::Color,
    pub border_color: sdl2::pixels::Color,
    pub intersection_entry_time: Option<std::time::Instant>,
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
    pub fn new(x: i32, y: i32, direction: u8, lane: Lane) -> Self {
        let color = match lane {
            Lane::Right => sdl2::pixels::Color::RGB(255, 255, 0),  // Yellow
            Lane::Middle => sdl2::pixels::Color::RGB(0, 255, 255), // Cyan
            Lane::Left => sdl2::pixels::Color::RGB(200, 150, 200), // Purple
        };

        let init_angle = match direction {
            0 => 270.0,     // North facing
            1 => 90.0,      // South facing
            2 => 180.0,     // West facing
            3 => 0.0,       // East facing
            _ => 0.0,
        };

        Vehicle {
            id: VEHICLE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            x: x as f64,
            y: y as f64,
            angle: init_angle,
            direction,
            lane,
            color,
            border_color: sdl2::pixels::Color::RGB(0, 255, 0),
            intersection_entry_time: None,
        }
    }

    // fn check_intersection_priority(&self, vehicles: &[Vehicle]) -> bool {
    //     let approaching_intersection = match self.direction {
    //         0 => self.y <= 198.0 + 50.0 && self.y > 198.0,  // North
    //         1 => self.y >= 406.0 - 50.0 && self.y < 406.0,  // South
    //         2 => self.x <= 304.0 + 50.0 && self.x > 304.0,  // West
    //         3 => self.x >= 502.0 - 50.0 && self.x < 502.0,  // East
    //         _ => false,
    //     };

    //     // if !approaching_intersection {
    //     //     println!("Vehicle {} is not approaching intersection", self.id);
    //     //     return false;
    //     // }

    //     for other in vehicles {
    //         if std::ptr::eq(self, other) {
    //             continue;
    //         }

    //         if other.is_in_intersection() {
    //             println!("Vehicle {} is in intersection or has priority", self.id);
    //             return false;
    //         }

    //         if let (Some(self_time), Some(other_time)) = (self.intersection_entry_time, other.intersection_entry_time) {
    //             if other_time < self_time {
    //                 println!("Vehicle {} has priority over vehicle {}", self.id, other.id);
    //                 return false;
    //             }
    //         }
    //     }

    //     true
    // }

    /// Updates vehicle position based on current state and surrounding vehicles
    /// 
    /// # Arguments
    /// * `vehicles` - Slice containing all vehicles for collision detection
    /// 
    /// Calculates next position, checks if movement is possible, and updates position/angle
    /// if vehicle is in intersection
    pub fn update(&mut self, vehicles: &[Vehicle]) {

        self.update_left_from_north(420.0, 277.0);
        self.update_left_from_south(300.0, 200.0);
        self.update_left_from_west(500.0, 200.0);
        self.update_left_from_east(424.0, 320.0);
        self.update_right_from_north(500.0, 400.0);
        self.update_right_from_south(380.0, 325.0);
        self.update_right_from_west(375.0, 280.0);
        self.update_right_from_east(300.0, 400.0);

        // let has_priority = self.check_intersection_priority(vehicles);
        // self.has_priority = has_priority;

        let (dx, dy) = self.get_movement_vector(vehicles);
        let next_x = self.x + dx;
        let next_y = self.y + dy;

        if self.is_collision(next_x, next_y, vehicles) {
            self.border_color = sdl2::pixels::Color::RGB(255, 0, 0);
        }

        // if !has_priority {
        //     self.border_color = sdl2::pixels::Color::RGB(255, 165, 0);  // Orange for waiting
        //     return;
        // }

        if self.can_move(next_x, next_y, vehicles) {
            self.x = next_x;
            self.y = next_y;
            //println!("Vehicle {} moved to {}X {}Y", self.id, self.x, self.y);
            self.border_color = sdl2::pixels::Color::RGB(0, 255, 0);
        }

        if self.is_in_intersection() {
            self.intersection_entry_time = Some(std::time::Instant::now());
            println!("Vehicle {} is in intersection at {}X {}Y", self.id, self.x, self.y);
        } else if !self.is_in_intersection() {
            self.intersection_entry_time = None;
        }
    }

    fn update_left_from_north(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle || self.direction != 0 {
            return;
        }
        // if self.x == target_x && self.y == target_y {
        //     self.angle = 180.0;
        // }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 180.0;
        }
    }

    fn update_left_from_south(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle || self.direction != 1 {
            return;
        }
        // if self.x == target_x && self.y == target_y {
        //     self.angle = 180.0;
        // }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 180.0;
        }
    }

    fn update_left_from_west(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle || self.direction != 2 {
            return;
        }
        // if self.x == target_x && self.y == target_y {
        //     self.angle = 270.0;
        // }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 270.0;
        }
    }

    fn update_left_from_east(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle || self.direction != 3 {
            return;
        }
        // if self.x == target_x && self.y == target_y {
        //     self.angle = -90.0;
        // }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = -90.0;
        }
    }

    fn update_right_from_north(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left || self.direction != 0 {
            return;
        }
        // if self.x == target_x && self.y == target_y {
        //     self.angle = 0.0;
        // }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 0.0;
        }
    }

    fn update_right_from_south(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left || self.direction != 1 {
            return;
        }
        // if self.x == target_x && self.y == target_y {
        //     self.angle = 0.0;
        // }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 0.0;
        }
    }

    fn update_right_from_west(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left || self.direction != 2 {
            return;
        }
        // if self.x == target_x && self.y == target_y {
        //     self.angle = 90.0;
        // }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 90.0;
        }
    }

    fn update_right_from_east(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left || self.direction != 3 {
            return;
        }
        // if self.x == target_x && self.y == target_y {
        //     self.angle = 90.0;
        // }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 90.0;
        }
    }

    /// Calculates movement vector based on current direction and intersection status
    /// 
    /// # Returns
    /// Tuple (dx, dy) representing movement delta:
    /// - In intersection: Uses angle-based vector calculation
    /// - Outside intersection: Uses fixed directional movement
    fn get_movement_vector(&mut self, vehicles: &[Vehicle]) -> (f64, f64) {
        // match self.direction {
        //     0 => (0.0, -VEHICLE_SPEED), // North
        //     1 => (0.0, VEHICLE_SPEED),  // South 
        //     2 => (-VEHICLE_SPEED, 0.0), // West
        //     3 => (VEHICLE_SPEED, 0.0),  // East
        //     _ => (0.0, 0.0),
        // }
        let current_speed = self.get_velocity(vehicles);
        let rad = self.angle * PI / 180.0;
        let dx = current_speed * rad.cos();
        let dy = current_speed * rad.sin();
        (dx, dy)
    }

    // pub fn get_velocity(&mut self, vehicles: &[Vehicle]) -> f64 {
    //     let mut max_velocity = 0.0;

    //     for vehicle in vehicles {
    //         // Get current speed from slowing_down logic
    //         let current_speed = match vehicle.is_in_intersection() {
    //             true => VEHICLE_SPEED * rand::thread_rng().gen_range(0.15..=0.95),
    //             false => VEHICLE_SPEED
    //         };

    //         // Calculate actual velocity components using angle
    //         let rad = vehicle.angle * PI / 180.0;
    //         let dx = current_speed * rad.cos();
    //         let dy = current_speed * rad.sin();

    //         // Get total velocity magnitude
    //         let velocity = (dx * dx + dy * dy).sqrt();

    //         if velocity > max_velocity {
    //             max_velocity = velocity;
    //         }
    //     }

    //     max_velocity
    // }

    pub fn get_velocity(&mut self, vehicles: &[Vehicle]) -> f64 {
        let slow_down_factor = 0.3; // Reduces speed to 30% when near intersection
        let approach_buffer = 50.0;  // Distance before intersection to start slowing
        // Check if approaching intersection based on direction
        let should_slow_down = match self.direction {
            2 => self.x <= 304.0 + approach_buffer && self.x > 304.0, // West inbound
            3 => self.x >= 502.0 - approach_buffer && self.x < 502.0, // East inbound
            0 => self.y <= 198.0 + approach_buffer && self.y > 198.0, // North inbound
            1 => self.y >= 406.0 - approach_buffer && self.y < 406.0, // South inbound
            _ => false,
        };
        let base_speed = match self.is_in_intersection() {
            true => VEHICLE_SPEED * rand::thread_rng().gen_range(0.15..=0.95),
            false => VEHICLE_SPEED
        };
        if should_slow_down {
            base_speed * slow_down_factor
        } else {
            base_speed
        }
    }

    // fn slowing_down(&mut self, vehicles: &[Vehicle]) -> f64 {
    //     let slow_down_distance = 60.0; // Distance to start slowing
    //     let min_speed = 0.5; // Minimum speed when slowing down
    //     let rate = rand::thread_rng().gen_range(0.15..=0.95);
    //     let stop_threshold = 2.0 * VEHICLE_SIZE as f64;

    //     for other in vehicles {
    //         if !other.is_in_intersection() && self.is_in_intersection() {
    //             return VEHICLE_SPEED * rate; // Significant slowdown when approaching occupied intersection
    //         }
    //     }

    //     for other in vehicles {
    //         if std::ptr::eq(self, other) {
    //             continue;
    //         }
    //         let dx = other.x - self.x;
    //         let dy = other.y - self.y;
    //         let distance = (dx * dx + dy * dy).sqrt();

    //         // Check if vehicle is ahead in same direction
    //         // let is_ahead = match self.direction {
    //         //     0 => other.y < self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64, // North
    //         //     1 => other.y > self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64, // South
    //         //     2 => other.x < self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64, // West
    //         //     3 => other.x > self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64, // East
    //         //     _ => false,
    //         // };
    //         let is_ahead = match self.direction {
    //             0 => other.y < self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64 * 2.0, // North
    //             1 => other.y > self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64 * 2.0, // South
    //             2 => other.x < self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64 * 2.0, // West
    //             3 => other.x > self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64 * 2.0, // East
    //             _ => false,
    //         };

    //         // if is_ahead && distance < SAFETY_DISTANCE {
    //         //     // Calculate reduced speed based on distance
    //         //     let speed_factor = (distance / slow_down_distance).max(min_speed);
    //         //     return VEHICLE_SPEED * 0.3;
    //         // }
    //         if is_ahead {
    //             // Complete stop when vehicle is within 2 units
    //             if distance < stop_threshold {
    //                 return 0.0;
    //             }
    //             // Gradual slowdown when approaching
    //             if distance < SAFETY_DISTANCE {
    //                 let speed_factor = ((distance - stop_threshold) / slow_down_distance).max(min_speed);
    //                 return VEHICLE_SPEED * speed_factor * 0.3;
    //             }
    //         }
    //     }

    //     VEHICLE_SPEED // Return normal speed if no vehicle ahead
    // }

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

        // Define intersection boundaries
        let intersection_left = 304.0;
        let intersection_right = 502.0;
        let intersection_top = 198.0;
        let intersection_bottom = 406.0;

        // Check if vehicle has passed the intersection
        let has_passed_intersection = match self.direction {
            0 => self.y < intersection_top,     // Northbound
            1 => self.y > intersection_bottom,  // Southbound
            2 => self.x < intersection_left,    // Westbound
            3 => self.x > intersection_right,   // Eastbound
            _ => false
        };

        // If vehicle has passed intersection, ignore intersection rules
        if has_passed_intersection {
            return true;
        }

        // Count vehicles in intersection
        let vehicles_in_intersection = vehicles.iter()
            .filter(|v| v.is_in_intersection())
            .count();

        for other in vehicles {
            if vehicles_in_intersection >= 3 && !self.is_in_intersection() {
                if next_x == WEST_STOP_POS && self.direction == 3 {
                    return false;
                } else if next_x == EAST_STOP_POS && self.direction == 2 {
                    return false;
                } else if next_y == SOUTH_STOP_POS && self.direction == 0 {
                    return false;
                } else if next_y == NORTH_STOP_POS && self.direction == 1 {
                    return false;
                }
            }
        }
        // for other in vehicles {
        //     if vehicles_in_intersection >= 3 && !self.is_in_intersection() {
        //         if next_x == WEST_STOP_POS && self.direction == 3 {
        //             return false;
        //         } else if next_x == EAST_STOP_POS && self.direction == 2 {
        //             return false;
        //         } else if next_y == SOUTH_STOP_POS && self.direction == 0 {
        //             return false;
        //         } else if next_y == NORTH_STOP_POS && self.direction == 1 {
        //             return false;
        //         }
        //     }
        // }

        // for other in vehicles {
        //     if std::ptr::eq(self, other) {
        //         continue;
        //     }

        //     let other_in_intersection = other.is_in_intersection();
        //     let self_in_intersection = self.is_in_intersection();

        //     // Compare entry times when both vehicles have timestamps
        //     if let (Some(self_time), Some(other_time)) = (self.intersection_entry_time, other.intersection_entry_time) {
        //         // If other vehicle entered first and is still in intersection, yield
        //         if other_time < self_time && other_in_intersection {
        //             return false;
        //         }
        //     }

        //     // Prevent new entry if vehicle already in intersection
        //     if !self_in_intersection && other_in_intersection {
        //         let distance = ((next_x - other.x).powi(2) + (next_y - other.y).powi(2)).sqrt();
        //         if distance < SAFETY_DISTANCE {
        //             return false;
        //         }
        //     }
        // }

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
                0 => other.y < self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64 * 1.0, // Moving north
                1 => other.y > self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64 * 1.0, // Moving south
                2 => other.x < self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64 * 1.0, // Moving west
                3 => other.x > self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64 * 1.0, // Moving east
                _ => false,
            };

            // println!(
            //     "Self pos: ({}, {}), Other pos: ({}, {}), Distance: {}",
            //     next_x, next_y, other.x, other.y, distance
            // );

            if is_ahead && distance < SAFETY_DISTANCE {
                return true;
            }

            if distance < STOPPING_DISTANCE {
                return true;
            }
        }
        false
    }

    /// Checks if vehicle is within intersection boundaries
    /// 
    /// # Returns
    /// Boolean indicating if vehicle is inside the intersection area
    /// defined by ROAD_WIDTH around intersection center (400, 300)
    pub fn is_in_intersection(&self) -> bool {
        // Define intersection boundaries at road ends
        let intersection_left = 304.0;
        let intersection_right = 502.0;
        let intersection_top = 198.0; 
        let intersection_bottom = 406.0;

        self.x > intersection_left 
            && self.x < intersection_right
            && self.y > intersection_top 
            && self.y < intersection_bottom

        // if is_in && self.intersection_entry_time.is_none() {
        //     self.intersection_entry_time = Some(std::time::Instant::now());
        // } else if !is_in {
        //     self.intersection_entry_time = None;
        // }

        // is_in
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
        let _ = canvas.fill_rect(rect);

        canvas.set_draw_color(self.border_color);
        canvas.draw_rect(rect)?;

        self.draw_direction_arrow(canvas)?;

        Ok(())
    }

    pub fn draw_borders(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 255, 0, 255));
        let safety_rect = Rect::new(
            self.x as i32 - VEHICLE_SIZE as i32 / 2,
            self.y as i32 - VEHICLE_SIZE as i32 / 2,
            VEHICLE_SIZE,
            VEHICLE_SIZE,
        );
        canvas.draw_rect(safety_rect)?;

        // let stopping_rect = Rect::new(
        //     (self.x - STOPPING_DISTANCE) as i32,
        //     (self.y - STOPPING_DISTANCE) as i32,
        //     (STOPPING_DISTANCE * 2.0) as u32,
        //     (STOPPING_DISTANCE * 2.0) as u32,
        // );
        // canvas.draw_rect(stopping_rect)?;

        Ok(())
    }

    pub fn update_border_color(&mut self, r: u8, g: u8, b: u8) -> sdl2::pixels::Color {
        let border_color = sdl2::pixels::Color::RGB(r, g, b);
        border_color
    }

    pub fn draw_direction_arrow(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let arrow_length = SAFETY_DISTANCE;
        let arrow_width = 15.0;

        let rad = self.angle * PI / 180.0;
        let tip_x = self.x + arrow_length * rad.cos();
        let tip_y = self.y + arrow_length * rad.sin();

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 255));
        let _ = canvas.draw_line(
            Point::new(self.x as i32, self.y as i32),
            Point::new(tip_x as i32, tip_y as i32),
        );

        // Calculate and draw arrow head
        let head_angle: f32 = 30.0; // Angle of arrow head lines
        let head_length = 15.0; // Length of arrow head lines

        let left_angle = rad + head_angle.to_radians() as f64;
        let right_angle = rad - head_angle.to_radians() as f64;

        let left_x = tip_x - head_length * left_angle.cos();
        let left_y = tip_y - head_length * left_angle.sin();
        let right_x = tip_x - head_length * right_angle.cos();
        let right_y = tip_y - head_length * right_angle.sin();

        // Draw arrow head lines
        canvas.draw_line(
            Point::new(tip_x as i32, tip_y as i32),
            Point::new(left_x as i32, left_y as i32)
        )?;
        canvas.draw_line(
            Point::new(tip_x as i32, tip_y as i32),
            Point::new(right_x as i32, right_y as i32)
        )?;

        Ok(())
    }

    pub fn update_glow(&mut self) -> Result<(), String> {
        // let glow_sizes = [4, 8, 12, 16];
        // let alphas = [200, 150, 100, 50];

        // for (size, alpha) in glow_sizes.iter().zip(alphas.iter()) {
        //     let glow_rect = Rect::new(
        //         self.x as i32 - (VEHICLE_SIZE as i32 + size) / 2,
        //         self.y as i32 - (VEHICLE_SIZE as i32 + size) / 2,
        //         VEHICLE_SIZE + *size as u32,
        //         VEHICLE_SIZE + *size as u32,
        //     );

        //     let glow_color = sdl2::pixels::Color::RGBA(
        //         self.color.r,
        //         self.color.g,
        //         self.color.b,
        //         *alpha,
        //     );
        // }

        self.color = sdl2::pixels::Color::RGB(
            (self.color.r as u16 + 40).min(255) as u8,
            (self.color.g as u16 + 40).min(255) as u8,
            (self.color.b as u16 + 40).min(255) as u8
        );

        Ok(())
    }
}
