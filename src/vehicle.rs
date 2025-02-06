//use std::sync::atomic::{AtomicU32, Ordering};
use std::f64::consts::PI;
use rand::Rng;

use crate::road::{ROAD_HEIGHT};
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

//static VEHICLE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

const VEHICLE_SIZE: u32 = 30;
const VEHICLE_SPEED: f64 = 2.0;
const SAFETY_DISTANCE: f64 = 35.0;
const STOPPING_DISTANCE: f64 = 30.0; // Distance at which to start slowing down

const NORTH_STOP_POS: f64 = 158.0;
const SOUTH_STOP_POS: f64 = 440.0;
const WEST_STOP_POS: f64 = 260.0;
const EAST_STOP_POS: f64 = 540.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Lane {
    Middle = 0,
    Right = 1,
    Left = 2,
}

#[derive(Clone)]
pub struct Vehicle {
    // pub id: u32,
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub direction: u8,
    pub lane: Lane,
    pub color: sdl2::pixels::Color,
    pub border_color: sdl2::pixels::Color,
    /// When a vehicle first enters the intersection, we record the time.
    pub intersection_entry_time: Option<std::time::Instant>,
    pub spawn_time: std::time::Instant,
    pub close_call_count: u32,
    pub is_in_collision: bool,
}

impl Vehicle {
    /// Creates a new vehicle instance with specified starting position, direction and lane.
    ///
    /// # Arguments
    /// * `x` - Initial x coordinate position
    /// * `y` - Initial y coordinate position
    /// * `direction` - Vehicle direction (0: North, 1: South, 2: West, 3: East)
    /// * `lane` - Lane type (Middle: straight, Right: right turn, Left: left turn)
    ///
    /// # Returns
    /// New Vehicle instance with color based on the lane:
    /// - Yellow for right turns
    /// - Cyan for straight (middle)
    /// - Purple for left turns
    pub fn new(x: i32, y: i32, direction: u8, lane: Lane) -> Self {
        let color = match lane {
            Lane::Right => sdl2::pixels::Color::RGB(255, 255, 0),   // Yellow
            Lane::Middle => sdl2::pixels::Color::RGB(0, 255, 255),    // Cyan
            Lane::Left => sdl2::pixels::Color::RGB(200, 150, 200),    // Purple
        };

        let init_angle = match direction {
            0 => 270.0,     // North facing
            1 => 90.0,      // South facing
            2 => 180.0,     // West facing
            3 => 0.0,       // East facing
            _ => 0.0,
        };

        Vehicle {
            // id: VEHICLE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            x: x as f64,
            y: y as f64,
            angle: init_angle,
            direction,
            lane,
            color,
            border_color: sdl2::pixels::Color::RGB(0, 255, 0),
            intersection_entry_time: None,
            spawn_time: std::time::Instant::now(),
            close_call_count: 0,
            is_in_collision: false,
        }
    }

    /// Determines if the given vehicle has priority to move into the intersection.
    ///
    /// A vehicle approaching the intersection (using a 50 unit buffer) must yield to any vehicle that
    /// has already entered. Once in the intersection, the vehicle's recorded entry time determines its
    /// priority.
    pub fn check_intersection_priority(&self, vehicles: &[Vehicle]) -> bool {
        let approaching_intersection = match self.direction {
            0 => self.y <= 198.0 + 50.0 && self.y > 198.0, // North inbound
            1 => self.y >= 406.0 - 50.0 && self.y < 406.0, // South inbound
            2 => self.x <= 304.0 + 50.0 && self.x > 304.0, // West inbound
            3 => self.x >= 502.0 - 50.0 && self.x < 502.0, // East inbound
            _ => false,
        };

        //println!("Approaching intersection: {}", approaching_intersection);

        // The vehicle that is not yet near the intersection can continue normally.
        if !approaching_intersection {
            return true;
        }

        // If the vehicle is already in the intersection, it has priority.
        if self.is_in_intersection() {
            return true;
        }

        // Determine this vehicle's "entry time" (if it hasn't been set, use the current time).
        let now = std::time::Instant::now();
        let self_time = self.intersection_entry_time.unwrap_or(now);

        // If any other vehicle in the intersection has an earlier entry time, then this vehicle must wait.
        for other in vehicles {
            if std::ptr::eq(self, other) {
                continue;
            }
            if other.is_in_intersection() {
                if let Some(other_time) = other.intersection_entry_time {
                    if other_time < self_time {
                        return false;
                    }
                } else {
                    // If the other vehicle is in the intersection but its time is not set,
                    // assume it came in first.
                    return false;
                }
            }
        }
        true
    }

    /// Updates vehicle position based on current state and surrounding vehicles.
    ///
    /// This method computes the next movement vector, checks collision, and then moves the vehicle if safe.
    /// If the vehicle does not have priority to enter the intersection, it will not move and its border color
    /// is set to orange.
    pub fn update(&mut self, vehicles: &[Vehicle]) {
        // Use the new intersection priority algorithm.
        if !self.check_intersection_priority(vehicles) {
            self.border_color = sdl2::pixels::Color::RGB(255, 165, 0);
            return;
        }

        // Check turning updates depending on lane and direction.
        self.update_left_from_north(420.0, 277.0);
        self.update_left_from_south(300.0, 200.0);
        self.update_left_from_west(500.0, 200.0);
        self.update_left_from_east(424.0, 320.0);
        self.update_right_from_north(500.0, 400.0);
        self.update_right_from_south(380.0, 325.0);
        self.update_right_from_west(375.0, 280.0);
        self.update_right_from_east(300.0, 400.0);

        let (dx, dy) = self.get_movement_vector(vehicles);
        let next_x = self.x + dx;
        let next_y = self.y + dy;

        if self.is_collision(next_x, next_y, vehicles) {
            if !self.is_in_collision { // Increment only if not already in collision
                self.close_call_count += 1;
                println!("Close call! {} at ({}, {})", self.close_call_count, self.x, self.y);
                self.is_in_collision = true; // Set collision state to true
            }
            self.border_color = sdl2::pixels::Color::RGB(255, 0, 0);
        } else {
            self.is_in_collision = false; // Reset collision state to false
        }

        if self.can_move(next_x, next_y, vehicles) {
            self.x = next_x;
            self.y = next_y;
            self.border_color = sdl2::pixels::Color::RGB(0, 255, 0);
        }

        // When a vehicle enters the intersection, record its entry time once.
        if self.is_in_intersection() {
            //println!("Vehicle {} entered intersection at {:?}", self.id, std::time::Instant::now());
            if self.intersection_entry_time.is_none() {
                self.intersection_entry_time = Some(std::time::Instant::now());
            }
        } else {
            // Reset the entry time once outside the intersection.
            self.intersection_entry_time = None;
        }
    }

    fn update_left_from_north(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle || self.direction != 0 {
            return;
        }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 180.0;
        }
    }

    fn update_left_from_south(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle || self.direction != 1 {
            return;
        }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 180.0;
        }
    }

    fn update_left_from_west(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle || self.direction != 2 {
            return;
        }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 270.0;
        }
    }

    fn update_left_from_east(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle || self.direction != 3 {
            return;
        }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = -90.0;
        }
    }

    fn update_right_from_north(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left || self.direction != 0 {
            return;
        }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 0.0;
        }
    }

    fn update_right_from_south(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left || self.direction != 1 {
            return;
        }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 0.0;
        }
    }

    fn update_right_from_west(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left || self.direction != 2 {
            return;
        }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 90.0;
        }
    }

    fn update_right_from_east(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left || self.direction != 3 {
            return;
        }
        if (self.x - target_x).abs() < 5.0 && (self.y - target_y).abs() < 5.0 {
            self.angle = 90.0;
        }
    }

    /// Calculates movement vector based on current angle and velocity.
    ///
    /// Uses the current velocity—which may be reduced when near the intersection—to compute dx and dy.
    fn get_movement_vector(&mut self, vehicles: &[Vehicle]) -> (f64, f64) {
        let current_speed = self.get_velocity(vehicles);
        let rad = self.angle * PI / 180.0;
        let dx = current_speed * rad.cos();
        let dy = current_speed * rad.sin();
        (dx, dy)
    }

    /// Returns the current vehicle velocity.
    ///
    /// When approaching an intersection, the vehicle slows down (30% speed); within the intersection,
    /// a randomized speed multiplier is applied.
    pub fn get_velocity(&self, _vehicles: &[Vehicle]) -> f64 {
        let slow_down_factor = 0.3;
        let approach_buffer = 50.0;
        let rate = rand::thread_rng().gen_range(0.55..=1.95);
        let should_slow_down = match self.direction {
            2 => self.x <= 304.0 + approach_buffer && self.x > 304.0,
            3 => self.x >= 502.0 - approach_buffer && self.x < 502.0,
            0 => self.y <= 198.0 + approach_buffer && self.y > 198.0,
            1 => self.y >= 406.0 - approach_buffer && self.y < 406.0,
            _ => false,
        };
        let base_speed = if self.is_in_intersection() {
            VEHICLE_SPEED * rand::thread_rng().gen_range(1.55..=4.45)
        } else {
            VEHICLE_SPEED * rate
        };
        if should_slow_down {
            base_speed * slow_down_factor
        } else {
            base_speed
        }
    }

    /// Determines if the vehicle can move safely to the next position.
    ///
    /// Checks for imminent collisions or blocking positions (stop positions).
    fn can_move(&mut self, next_x: f64, next_y: f64, vehicles: &[Vehicle]) -> bool {
        if self.is_collision(next_x, next_y, vehicles) {
            return false;
        }

        // Check for count vehicles in the intersection.
        let vehicles_in_intersection = vehicles.iter()
            .filter(|v| {
                v.is_in_intersection()
            }).count();
        for other in vehicles {
            //println!("Vehicles in intersection: {}", vehicles_in_intersection);
            //println!("Checking for blocking positions...");
            if vehicles_in_intersection >= 3 {
                if other.is_in_intersection() && !self.is_in_intersection() {
                    //println!("Vehicles in intersection: {}", vehicles_in_intersection);
                    if next_x == WEST_STOP_POS && self.direction == 3 {
                        println!("Blocking position at West Stop Position");
                        return false;
                    } else if next_x == EAST_STOP_POS && self.direction == 2 {
                        println!("Blocking position at East Stop Position");
                        return false;
                    } else if next_y == SOUTH_STOP_POS && self.direction == 0 {
                        println!("Blocking position at South Stop Position");
                        return false;
                    } else if next_y == NORTH_STOP_POS && self.direction == 1 {
                        println!("Blocking position at North Stop Position");
                        return false;
                    }
                }
            }
        }
        // Check for vehicles that are blocking stop positions (e.g., positions near the intersection).
        // let threshold = 1.0;
        // for _other in vehicles {
        //     if (next_x - WEST_STOP_POS).abs() < threshold && self.direction == 3 {
        //         return false;
        //     } else if (next_x - EAST_STOP_POS).abs() < threshold && self.direction == 2 {
        //         return false;
        //     } else if (next_y - SOUTH_STOP_POS).abs() < threshold && self.direction == 0 {
        //         return false;
        //     } else if (next_y - NORTH_STOP_POS).abs() < threshold && self.direction == 1 {
        //         return false;
        //     }
        // }
        true
    }

    /// Checks for potential collisions with other vehicles.
    ///
    /// If another vehicle is within the safety or stopping distance ahead, a collision is assumed.
    fn is_collision(&mut self, next_x: f64, next_y: f64, vehicles: &[Vehicle]) -> bool {
        for other in vehicles {
            if std::ptr::eq(self, other) {
                continue;
            }
            let dx = next_x - other.x;
            let dy = next_y - other.y;
            let distance = (dx * dx + dy * dy).sqrt();

            let is_ahead = match self.direction {
                0 => other.y < self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64,
                1 => other.y > self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64,
                2 => other.x < self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64,
                3 => other.x > self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64,
                _ => false,
            };

            // Check if vehicles are moving in the same direction
            let same_direction = self.direction == other.direction;

            // Only count close calls for vehicles moving in different directions
            if !same_direction && distance < SAFETY_DISTANCE {
                if !self.is_in_collision { // Increment only if not already in collision
                    self.close_call_count += 1;
                    println!("Close call! {} at ({}, {})", self.close_call_count, self.x, self.y);
                    self.is_in_collision = true; // Set collision state to true
                }
            }

            if !same_direction && distance < STOPPING_DISTANCE {
                return true;
            }

            if is_ahead && distance < SAFETY_DISTANCE {
                return true;
            }
            if distance < STOPPING_DISTANCE {
                return true;
            }
        }
        false
    }

    /// Checks if the vehicle is within the intersection boundaries.
    ///
    /// The intersection is defined as the rectangle bounded by [304, 502] in the horizontal
    /// and [198, 406] in the vertical direction.
    pub fn is_in_intersection(&self) -> bool {
        let intersection_left = 304.0;
        let intersection_right = 502.0;
        let intersection_top = 198.0;
        let intersection_bottom = 406.0;

        self.x > intersection_left &&
        self.x < intersection_right &&
        self.y > intersection_top &&
        self.y < intersection_bottom
    }

    /// Checks if the vehicle has completed its journey (i.e. left the simulation bounds).
    pub fn is_finished(&self) -> bool {
        self.x < 0.0 || self.x > ROAD_HEIGHT as f64 || self.y < 0.0 || self.y > ROAD_HEIGHT as f64
    }

    /// Renders the vehicle on the canvas.
    ///
    /// Draws the vehicle rectangle with its color, border, and a direction arrow indicating
    /// the current velocity vector.
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

    // /// Draws additional visual debugging information such as safety boundaries.
    // pub fn draw_borders(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
    //     // Draw safety distance boundary
    //     canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 165, 0, 100));
    //     let safety_rect = Rect::new(
    //         (self.x - SAFETY_DISTANCE) as i32,
    //         (self.y - SAFETY_DISTANCE) as i32,
    //         (SAFETY_DISTANCE * 2.0) as u32,
    //         (SAFETY_DISTANCE * 2.0) as u32,
    //     );
    //     canvas.draw_rect(safety_rect)?;

    //     // Draw stopping distance boundary
    //     canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 0, 0, 100));
    //     let stopping_rect = Rect::new(
    //         (self.x - STOPPING_DISTANCE) as i32,
    //         (self.y - STOPPING_DISTANCE) as i32,
    //         (STOPPING_DISTANCE * 2.0) as u32,
    //         (STOPPING_DISTANCE * 2.0) as u32,
    //     );
    //     canvas.draw_rect(stopping_rect)?;

    //     // Draw actual vehicle collision boundary
    //     canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 255, 0, 255));
    //     let vehicle_rect = Rect::new(
    //         self.x as i32 - VEHICLE_SIZE as i32,
    //         self.y as i32 - VEHICLE_SIZE as i32,
    //         VEHICLE_SIZE * 2,
    //         VEHICLE_SIZE * 2,
    //     );
    //     canvas.draw_rect(vehicle_rect)?;

    //     Ok(())
    // }

    /// Draws the direction arrow for the vehicle.
    pub fn draw_direction_arrow(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let arrow_length = SAFETY_DISTANCE;
        let rad = self.angle * PI / 180.0;
        let tip_x = self.x + arrow_length * rad.cos();
        let tip_y = self.y + arrow_length * rad.sin();

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 255));
        canvas.draw_line(
            Point::new(self.x as i32, self.y as i32),
            Point::new(tip_x as i32, tip_y as i32),
        )?;

        // Calculate arrow head
        let head_angle: f32 = 30.0;
        let head_length = 15.0;
        let left_angle = rad + head_angle.to_radians() as f64;
        let right_angle = rad - head_angle.to_radians() as f64;
        let left_x = tip_x - head_length * left_angle.cos();
        let left_y = tip_y - head_length * left_angle.sin();
        let right_x = tip_x - head_length * right_angle.cos();
        let right_y = tip_y - head_length * right_angle.sin();

        canvas.draw_line(
            Point::new(tip_x as i32, tip_y as i32),
            Point::new(left_x as i32, left_y as i32),
        )?;
        canvas.draw_line(
            Point::new(tip_x as i32, tip_y as i32),
            Point::new(right_x as i32, right_y as i32),
        )?;

        Ok(())
    }

    // /// Update the vehicle's glow effect by lightening its color.
    // pub fn update_glow(&mut self) -> Result<(), String> {
    //     self.color = sdl2::pixels::Color::RGB(
    //         (self.color.r as u16 + 40).min(255) as u8,
    //         (self.color.g as u16 + 40).min(255) as u8,
    //         (self.color.b as u16 + 40).min(255) as u8,
    //     );
    //     Ok(())
    // }
}