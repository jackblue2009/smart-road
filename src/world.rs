use crate::road::{ROAD_HEIGHT, ROAD_WIDTH};
// use crate::traffic_light::TrafficLight;
use crate::vehicle::Vehicle;
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

use crate::vehicle::Lane;

// Define allowed routes for each spawn direction
#[derive(Debug, Clone, Copy)]
struct VehicleRouting {
    spawn_x: i32,
    spawn_y: i32,
    spawn_angle: f64,
    spawn_direction: u8,
    allowed_routes: &'static [u8], // 0: straight, 1: right, 2: left
}

pub struct World {
    vehicles: Vec<Vehicle>,
    // traffic_lights: Vec<TrafficLight>,
    last_vehicle_spawn_time: Instant,
    vehicle_spawn_cooldown: Duration,
    max_vehicles: usize,
}

#[allow(dead_code)]
impl World {
    pub fn new() -> Self {
        World {
            vehicles: Vec::new(),
            // traffic_lights,
            last_vehicle_spawn_time: Instant::now(),
            vehicle_spawn_cooldown: Duration::from_millis(500),
            max_vehicles: 8,
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.vehicles.len() {
            let (current, others) = self.vehicles.split_at_mut(i);
            if let Some((vehicle, rest)) = others.split_first_mut() {
                let mut collision_check = current.to_vec();
                collision_check.extend_from_slice(rest);
                vehicle.update(&collision_check);
            }
        }

        self.vehicles.retain(|v| !v.is_finished());
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // Draw roads
        canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
        let lane_width = ROAD_WIDTH / 6;

        // Vertical Road
        // Left side (downward traffic) - 3 lanes
        for i in 0..3 {
            canvas.fill_rect(Rect::new(400 - ROAD_WIDTH as i32 / 2 + (i * lane_width as i32), 0, lane_width, 600))?;
        }

        // Right side (upward traffic) - 3 lanes
        for i in 0..3 {
            canvas.fill_rect(Rect::new(400 + (i * lane_width as i32), 0, lane_width, 600))?;
        }

        // Horizontal Road
        // Top side (leftward traffic) - 3 lanes
        for i in 0..3 {
            canvas.fill_rect(Rect::new(0, 300 - ROAD_WIDTH as i32 / 2 + (i * lane_width as i32), 800, lane_width))?;
        }

        // Bottom side (rightward traffic) - 3 lanes
        for i in 0..3 {
            canvas.fill_rect(Rect::new(0, 300 + (i * lane_width as i32), 800, lane_width))?;
        }

        // Draw lane markings
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

        // Vertical lane markings
        for i in 1..6 {
            let x = 400 - ROAD_WIDTH as i32 / 2 + (i * lane_width as i32);
            canvas.fill_rect(Rect::new(x - 2, 0, 4, 300 - ROAD_WIDTH as u32 / 2))?;
            canvas.fill_rect(Rect::new(x - 2, 300 + ROAD_WIDTH as i32 / 2, 4, 300 - ROAD_WIDTH as u32 / 2))?;
        }

        // Horizontal lane markings
        for i in 1..6 {
            let y = 300 - ROAD_WIDTH as i32 / 2 + (i * lane_width as i32);
            canvas.fill_rect(Rect::new(0, y - 2, 400 - ROAD_WIDTH as u32 / 2, 4))?;
            canvas.fill_rect(Rect::new(400 + ROAD_WIDTH as i32 / 2, y - 2, 400 - ROAD_WIDTH as u32 / 2, 4))?;
        }

        // Draw horizontal connecting lines at lane ends
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

        // North end horizontal connectors
        canvas.fill_rect(Rect::new(
            280,                          // Start from rightmost lane
            176,                          // Some distance from the bottom
            ROAD_WIDTH as u32 / 2,        // Span across right lanes
            4                             // Line thickness
        ))?;

        // South end horizontal connectors
        canvas.fill_rect(Rect::new(
            400,                          // Start from rightmost lane
            420,                          // Some distance from the bottom
            ROAD_WIDTH as u32 / 2,        // Span across right lanes
            4                             // Line thickness
        ))?;

        // East end horizontal connectors
        canvas.fill_rect(Rect::new(
            520,                          // Some distance from right edge
            180,                          // Start from rightmost lane
            4,                            // Line thickness
            ROAD_WIDTH as u32 / 2         // Span across right lanes
        ))?;

        // West end horizontal connectors
        canvas.fill_rect(Rect::new(
            277,                          // Some distance from left edge
            300,                          // Start from rightmost lane
            4,                            // Line thickness
            ROAD_WIDTH as u32 / 2         // Span across right lanes
        ))?;

        // Draw vehicles
        for vehicle in &self.vehicles {
            vehicle.draw(canvas)?;
        }

        Ok(())
    }

    pub fn handle_key_event(&mut self, keycode: Keycode) {
        let mut rng = rand::thread_rng();
        // let route = rng.gen_range(0..3);
        // // let route = (self.vehicles.len() % 3) as u8;
        // let lane_width = ROAD_WIDTH / 6;
        // if Instant::now().duration_since(self.last_vehicle_spawn_time) < self.vehicle_spawn_cooldown
        //     || self.vehicles.len() >= self.max_vehicles
        // {
        //     return;
        // }
        // let lane_width = ROAD_WIDTH / 6;
        // let route = (self.vehicles.len() % 3) as u8;  // 0: right, 1: straight, 2: left

        if Instant::now().duration_since(self.last_vehicle_spawn_time) < self.vehicle_spawn_cooldown
        || self.vehicles.len() >= self.max_vehicles
        {
            return;
        }

        let lane_width = ROAD_WIDTH / 6;

        // Define spawn positions for each lane and direction
        let spawn_config = match keycode {
            // From North
            Keycode::Down => Some((
                420 - ROAD_WIDTH as i32 / 3 + lane_width as i32 * (self.vehicles.len() % 3) as i32,  // Align with lanes
                0,
                90.0,
                1,
            )),
            
            // From East
            Keycode::Left => Some((
                800,
                300 - ROAD_WIDTH as i32 / 3 + lane_width as i32 * (self.vehicles.len() % 3) as i32,
                180.0,
                2,
            )),
            
            // From South
            Keycode::Up => Some((
                420 + lane_width as i32 * (self.vehicles.len() % 3) as i32,
                600,
                270.0,
                0,
            )),
            
            // From West
            Keycode::Right => Some((
                0,
                300 + lane_width as i32 * (self.vehicles.len() % 3) as i32,
                0.0,
                3,
            )),
            
            _ => None,
        };

        if let Some((x, y, angle, direction)) = spawn_config {
            let lane = match self.vehicles.len() % 3 {
                0 => Lane::Right,
                1 => Lane::Middle,
                2 => Lane::Left,
                _ => unreachable!(),
            };
            
            self.vehicles.push(Vehicle::new(x, y, angle, direction, lane));
            self.last_vehicle_spawn_time = Instant::now();
        }

        // // Define routing options for each direction
        // let routing_options = [
        //     // Up arrow (spawn from South, moving North)
        //     VehicleRouting {
        //         spawn_x: 400 + ROAD_WIDTH as i32 / 2 - (lane_width as i32 * (3-route as i32) - lane_width as i32 / 2),
        //         spawn_y: 600,
        //         spawn_angle: -90.0,
        //         spawn_direction: 0,
        //         allowed_routes: &[0, 1, 2],
        //     },
        //     // Down arrow (spawn from North, moving South)
        //     VehicleRouting {
        //         spawn_x: 400 - ROAD_WIDTH as i32 / 2 + (lane_width as i32 * route as i32 + lane_width as i32 / 2),
        //         spawn_y: 0,
        //         spawn_angle: 90.0,
        //         spawn_direction: 1,
        //         allowed_routes: &[0, 1, 2],
        //     },
        //     // Left arrow (spawn from East, moving West)
        //     VehicleRouting {
        //         spawn_x: 800,
        //         spawn_y: 300 - ROAD_WIDTH as i32 / 2 + (lane_width as i32 * route as i32 + lane_width as i32 / 2),
        //         spawn_angle: 180.0,
        //         spawn_direction: 2,
        //         allowed_routes: &[0, 1, 2],
        //     },
        //     // Right arrow (spawn from West, moving East)
        //     VehicleRouting {
        //         spawn_x: 0,
        //         spawn_y: 300 + ROAD_WIDTH as i32 / 2 - (lane_width as i32 * (3-route as i32) - lane_width as i32 / 2),
        //         spawn_angle: 0.0,
        //         spawn_direction: 3,
        //         allowed_routes: &[0, 1, 2],
        //     },
        // ];

        // // Match keycode to appropriate routing
        // match keycode {
        //     Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
        //         let direction_index = match keycode {
        //             Keycode::Up => 0,
        //             Keycode::Down => 1,
        //             Keycode::Left => 2,
        //             Keycode::Right => 3,
        //             _ => unreachable!()
        //         };
        //         // Cycle through lanes 0,1,2 (right, straight, left)
        //         let route = (self.vehicles.len() % 3) as u8;
        //         let routing = &routing_options[direction_index];
        //         self.vehicles.push(Vehicle::new(
        //             routing.spawn_x,
        //             routing.spawn_y,
        //             routing.spawn_angle,
        //             routing.spawn_direction,
        //             Lane::Middle
        //         ));
        //         self.last_vehicle_spawn_time = Instant::now();
        //     }
        //     Keycode::R => {
        //         // Random direction spawn
        //         let routing = &routing_options[rng.gen_range(0..4)];
        //         let route = routing.allowed_routes[rng.gen_range(0..routing.allowed_routes.len())];
        //         self.vehicles.push(Vehicle::new(
        //             routing.spawn_x,
        //             routing.spawn_y,
        //             routing.spawn_angle,
        //             routing.spawn_direction,
        //             Lane::Middle
        //         ));
        //         self.last_vehicle_spawn_time = Instant::now();
        //     }
        //     _ => {}
        // }
    }

    fn spawn_vehicle(&mut self, direction: u8) {
        let (x, y, angle) = match direction {
            0 => (420, 580, 180.0), // Bottom to top
            1 => (380, 20, 0.0),    // Top to bottom
            2 => (20, 320, 90.0),   // Left to right
            3 => (780, 280, -90.0), // Right to left
            _ => unreachable!(),
        };

        let route = rand::thread_rng().gen_range(0..3);
        //let route = 1;
        self.vehicles
            .push(Vehicle::new(x, y, angle, direction, Lane::Middle));
    }
}
