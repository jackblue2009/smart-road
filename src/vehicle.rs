use std::sync::atomic::{AtomicU32, Ordering};

use crate::road::{ROAD_HEIGHT, ROAD_WIDTH};
// use crate::traffic_light::TrafficLight;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;

static VEHICLE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

const VEHICLE_SIZE: u32 = 30;
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
#[derive(Clone, Copy, PartialEq)]
pub enum Lane {
    Middle = 0,
    Right = 1,
    Left = 2,
}

#[derive(Clone)]
pub struct Vehicle {
    id: u32,
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
    pub fn new(x: i32, y: i32, direction: u8, lane: Lane) -> Self {
        let color = match lane {
            Lane::Right => sdl2::pixels::Color::RGB(255, 255, 0),  // Yellow
            Lane::Middle => sdl2::pixels::Color::RGB(0, 255, 255), // Cyan
            Lane::Left => sdl2::pixels::Color::RGB(200, 150, 200), // Purple
            _ => unreachable!(),
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
        self.update_left_from_north(420.0, 280.0);
        self.update_left_from_south(300.0, 200.0);
        self.update_left_from_west(500.0, 200.0);
        self.update_left_from_east(420.0, 320.0);
        self.update_right_from_north(500.0, 400.0);
        self.update_right_from_south(380.0, 320.0);
        self.update_right_from_west(380.0, 280.0);
        self.update_right_from_east(300.0, 400.0);
        let (dx, dy) = self.get_movement_vector(vehicles);
        let next_x = self.x + dx;
        let next_y = self.y + dy;
        if self.can_move(next_x, next_y, vehicles) {
            self.x = next_x;
            self.y = next_y;

            if self.is_in_intersection() {
                println!("Vehicle {} is in intersection at {}X {}Y", self.id, self.x, self.y);
            }
        }
    }

    fn update_left_from_north(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle {
            return;
        }
        if self.x == target_x && self.y == target_y && self.direction == 0 {
            self.angle = 180.0;
        }
    }

    fn update_left_from_south(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle {
            return;
        }
        if self.x == target_x && self.y == target_y && self.direction == 1 {
            self.angle = 180.0;
        }
    }

    fn update_left_from_west(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle {
            return;
        }
        if self.x == target_x && self.y == target_y && self.direction == 2 {
            self.angle = 270.0;
        }
    }

    fn update_left_from_east(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Middle {
            return;
        }
        if self.x == target_x && self.y == target_y && self.direction == 3 {
            self.angle = -90.0;
        }
    }

    fn update_right_from_north(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left {
            return;
        }
        if self.x == target_x && self.y == target_y && self.direction == 0 {
            self.angle = 0.0;
        }
    }

    fn update_right_from_south(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left {
            return;
        }
        if self.x == target_x && self.y == target_y && self.direction == 1 {
            self.angle = 0.0;
        }
    }

    fn update_right_from_west(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left {
            return;
        }
        if self.x == target_x && self.y == target_y && self.direction == 2 {
            self.angle = 90.0;
        }
    }

    fn update_right_from_east(&mut self, target_x: f64, target_y: f64) {
        if self.lane != Lane::Left {
            return;
        }
        if self.x == target_x && self.y == target_y && self.direction == 3 {
            self.angle = 90.0;
        }
    }

    // fn update_angle_at_point(&mut self, target_x: f64, target_y: f64) {
    //     if self.x == target_x && self.y == target_y {
    //         match (self.direction, self.lane) {
    //             (0, Lane::Left) => self.angle = 180.0,   // North + Left = turn left
    //             (0, Lane::Right) => self.angle = -90.0, // North + Right = turn right
    //             (0, Lane::Middle) => self.angle = 0.0,  // North + Middle = straight
                
    //             (1, Lane::Left) => self.angle = 0.0,   // South + Left = turn left
    //             (1, Lane::Right) => self.angle = -90.0, // South + Right = turn right
    //             (1, Lane::Middle) => self.angle = 180.0,// South + Middle = straight
                
    //             (2, Lane::Left) => self.angle = 270.0,    // West + Left = turn left
    //             (2, Lane::Right) => self.angle = 180.0, // West + Right = turn right
    //             (2, Lane::Middle) => self.angle = -90.0,// West + Middle = straight
                
    //             (3, Lane::Left) => self.angle = 0.0,    // East + Left = turn left
    //             (3, Lane::Right) => self.angle = 180.0, // East + Right = turn right
    //             (3, Lane::Middle) => self.angle = 90.0, // East + Middle = straight

    //             (_, _) => self.angle = 0.0,
    //         }
    //     }
    // }

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
        let current_speed = self.slowing_down(vehicles);
        let rad = self.angle * PI / 180.0;
        let dx = current_speed * rad.cos();
        let dy = current_speed * rad.sin();
        (dx, dy)
    }

    fn slowing_down(&mut self, vehicles: &[Vehicle]) -> f64 {
        let slow_down_distance = 50.0; // Distance to start slowing
        let min_speed = 0.5; // Minimum speed when slowing down
        
        for other in vehicles {
            if std::ptr::eq(self, other) {
                continue;
            }
            
            let dx = other.x - self.x;
            let dy = other.y - self.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // Check if vehicle is ahead in same direction
            let is_ahead = match self.direction {
                0 => other.y < self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64, // North
                1 => other.y > self.y && (other.x - self.x).abs() < VEHICLE_SIZE as f64, // South
                2 => other.x < self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64, // West
                3 => other.x > self.x && (other.y - self.y).abs() < VEHICLE_SIZE as f64, // East
                _ => false,
            };

            if is_ahead && distance < slow_down_distance {
                // Calculate reduced speed based on distance
                let speed_factor = (distance / slow_down_distance).max(min_speed);
                return VEHICLE_SPEED * speed_factor;
            }
        }
        
        VEHICLE_SPEED // Return normal speed if no vehicle ahead
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
        // Define intersection boundaries at road ends
        let intersection_left = 400.0;
        let intersection_right = 400.0;
        let intersection_top = 300.0; 
        let intersection_bottom = 300.0;

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
        if self.direction == 0 && self.lane == Lane::Left {
            if self.x == 500.0 {
                self.angle = 90.0;
            }
        }
        if self.direction == 0 && self.lane == Lane::Right {
            if self.x == 300.0 {
                self.angle = -90.0;
            }
        }
        if self.direction == 0 && self.lane == Lane::Middle {
            self.angle = -90.0;
        }
        if self.direction == 1 && self.lane == Lane::Left {
            if self.x == 300.0 {
                self.angle = 90.0;
            }
        }
        if self.direction == 1 && self.lane == Lane::Right {
            if self.x == 500.0 {
                self.angle = -90.0;
            }
        }
        if self.direction == 1 && self.lane == Lane::Middle {
            self.angle = 90.0;
        }
        if self.direction == 2 && self.lane == Lane::Left {
            if self.y == 500.0 {
                self.angle = 0.0;
            }
        }
        if self.direction == 2 && self.lane == Lane::Right {
            if self.y == 300.0 {
                self.angle = 180.0;
            }
        }
        if self.direction == 2 && self.lane == Lane::Middle {
            self.angle = 0.0;
        }
        if self.direction == 3 && self.lane == Lane::Left {
            if self.y == 300.0 {
                self.angle = 0.0;
            }
        }
        if self.direction == 3 && self.lane == Lane::Right {
            if self.y == 500.0 {
                self.angle = 180.0;
            }
        }
        if self.direction == 3 && self.lane == Lane::Middle {
            self.angle = 0.0;
        }
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
