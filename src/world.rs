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
        // let mut traffic_lights = Vec::new();
        // traffic_lights.push(TrafficLight::new(
        //     380 - ROAD_WIDTH as i32 / 2,
        //     320 - ROAD_WIDTH as i32,
        //     0,
        // )); // Top left
        // traffic_lights.push(TrafficLight::new(
        //     400 + ROAD_WIDTH as i32 / 2,
        //     260 + ROAD_WIDTH as i32,
        //     75,
        // )); // Bottom right
        // traffic_lights.push(TrafficLight::new(
        //     420 - ROAD_WIDTH as i32,
        //     380 - ROAD_WIDTH as i32 / 2,
        //     150,
        // )); // Bottom left
        // traffic_lights.push(TrafficLight::new(
        //     360 + ROAD_WIDTH as i32,
        //     200 + ROAD_WIDTH as i32 / 2,
        //     225,
        // )); // Top right

        World {
            vehicles: Vec::new(),
            // traffic_lights,
            last_vehicle_spawn_time: Instant::now(),
            vehicle_spawn_cooldown: Duration::from_millis(500),
            max_vehicles: 8,
        }
    }

    pub fn update(&mut self) {
        // for light in &mut self.traffic_lights {
        //     light.update();
        // }

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
        // let lane_width = ROAD_WIDTH / 2;
        let lane_width = ROAD_WIDTH / 6;

        // // Left lane (downward traffic)
        // canvas.fill_rect(Rect::new(400 - ROAD_WIDTH as i32 / 2, 0, lane_width, 600))?;

        // // Right lane (upward traffic)
        // canvas.fill_rect(Rect::new(400, 0, lane_width, 600))?;

        // Vertical Road
        // Left side (downward traffic) - 3 lanes
        for i in 0..3 {
            canvas.fill_rect(Rect::new(400 - ROAD_WIDTH as i32 / 2 + (i * lane_width as i32), 0, lane_width, 600))?;
        }

        // Right side (upward traffic) - 3 lanes
        for i in 0..3 {
            canvas.fill_rect(Rect::new(400 + (i * lane_width as i32), 0, lane_width, 600))?;
        }

        // // Top lane (leftward traffic)
        // canvas.fill_rect(Rect::new(0, 300 - ROAD_WIDTH as i32 / 2, 800, lane_width))?;

        // // Bottom lane (rightward traffic)
        // canvas.fill_rect(Rect::new(0, 300, 800, lane_width))?;

        // Horizontal Road
        // Top side (leftward traffic) - 3 lanes
        for i in 0..3 {
            canvas.fill_rect(Rect::new(0, 300 - ROAD_WIDTH as i32 / 2 + (i * lane_width as i32), 800, lane_width))?;
        }

        // Bottom side (rightward traffic) - 3 lanes
        for i in 0..3 {
            canvas.fill_rect(Rect::new(0, 300 + (i * lane_width as i32), 800, lane_width))?;
        }

        // // Draw lane markings
        // canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255)); // White color for lines

        // // Vertical center lines (split into two parts to skip intersection)
        // canvas.fill_rect(Rect::new(400 - 2, 0, 4, 300 - ROAD_WIDTH as u32 / 2))?; // Upper part

        // canvas.fill_rect(Rect::new(
        //     400 - 2,
        //     300 + ROAD_WIDTH as i32 / 2,
        //     4,
        //     300 - ROAD_WIDTH as u32 / 2,
        // ))?; // Lower part

        // // Horizontal center lines (split into two parts to skip intersection)
        // canvas.fill_rect(Rect::new(0, 300 - 2, 400 - ROAD_WIDTH as u32 / 2, 4))?; // Left part

        // canvas.fill_rect(Rect::new(
        //     400 + ROAD_WIDTH as i32 / 2,
        //     300 - 2,
        //     400 - ROAD_WIDTH as u32 / 2,
        //     4,
        // ))?; // Right part

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
            // 400 - ROAD_WIDTH as i32 / 2,  // Start from leftmost lane
            // 100,                          // Some distance from the top
            // ROAD_WIDTH / 2 as u32,            // Span across 3 lanes
            // 4                             // Line thickness
            280,                          // Start from rightmost lane
            176,                          // Some distance from the bottom
            ROAD_WIDTH as u32 / 2,        // Span across right lanes
            4                             // Line thickness
        ))?;

        // South end horizontal connectors
        canvas.fill_rect(Rect::new(
            // 400 - ROAD_WIDTH as i32 / 2,  // Start from leftmost lane
            // 500,                          // Some distance from the bottom
            // ROAD_WIDTH / 2 as u32,            // Span across 3 lanes
            // 4                             // Line thickness
            400,                          // Start from rightmost lane
            420,                          // Some distance from the bottom
            ROAD_WIDTH as u32 / 2,        // Span across right lanes
            4                             // Line thickness
        ))?;

        // East end horizontal connectors
        canvas.fill_rect(Rect::new(
            // 700,                          // Some distance from right edge
            // 300 - ROAD_WIDTH as i32 / 2,  // Start from top lane
            // 4,                            // Line thickness
            // ROAD_WIDTH / 2 as u32             // Span across 3 lanes
            520,                          // Some distance from right edge
            180,                          // Start from rightmost lane
            4,                            // Line thickness
            ROAD_WIDTH as u32 / 2         // Span across right lanes
        ))?;

        // West end horizontal connectors
        canvas.fill_rect(Rect::new(
            // 100,                          // Some distance from left edge
            // 300 - ROAD_WIDTH as i32 / 2,  // Start from top lane
            // 4,                            // Line thickness
            // ROAD_WIDTH / 2 as u32             // Span across 3 lanes
            277,                          // Some distance from left edge
            300,                          // Start from rightmost lane
            4,                            // Line thickness
            ROAD_WIDTH as u32 / 2         // Span across right lanes
        ))?;

        // Draw vehicles
        for vehicle in &self.vehicles {
            vehicle.draw(canvas)?;
        }

        // // Draw traffic lights
        // for light in &self.traffic_lights {
        //     light.draw(canvas)?;
        // }

        Ok(())
    }

    pub fn handle_key_event(&mut self, keycode: Keycode) {
        // Prevent vehicle spam by checking time since last spawn
        if Instant::now().duration_since(self.last_vehicle_spawn_time) < self.vehicle_spawn_cooldown
        {
            return;
        }

        // Prevent exceeding max vehicle limit
        if self.vehicles.len() >= self.max_vehicles {
            return;
        }

        let mut rng = rand::thread_rng();
        let route = rng.gen_range(0..3);
        let lane_width = ROAD_WIDTH / 6;

        // Define routing options for each direction
        let routing_options = [
            // Up arrow (spawn from South, moving North)
            VehicleRouting {
                // spawn_x: 400 + (ROAD_WIDTH / 4) as i32, // Right lane
                // spawn_y: 600,
                // spawn_angle: -90.0,
                // spawn_direction: 0,
                // allowed_routes: &[0, 1, 2],
                // spawn_x: 400 + (ROAD_WIDTH / 6 + route as u32 * ROAD_WIDTH / 6) as i32, // Right lanes based on route
                // spawn_y: 600,
                // spawn_angle: -90.0,
                // spawn_direction: 0,
                // allowed_routes: &[0, 1, 2],
                spawn_x: 400 + ROAD_WIDTH as i32 / 2 - (lane_width as i32 * (route + 1) - lane_width as i32 / 2),
                spawn_y: 600,
                spawn_angle: -90.0,
                spawn_direction: 0,
                allowed_routes: &[0, 1, 2],
            },
            // Down arrow (spawn from North, moving South)
            VehicleRouting {
                // spawn_x: 400 - (ROAD_WIDTH / 4) as i32, // Left lane
                // spawn_y: 0,
                // spawn_angle: 90.0,
                // spawn_direction: 1,
                // allowed_routes: &[0, 1, 2],
                // spawn_x: 400 - (ROAD_WIDTH / 6 + route as u32 * ROAD_WIDTH / 6) as i32, // Left lanes based on route
                // spawn_y: 0,
                // spawn_angle: 90.0,
                // spawn_direction: 1,
                // allowed_routes: &[0, 1, 2],
                spawn_x: 400 - ROAD_WIDTH as i32 / 2 + (lane_width as i32 * route + lane_width as i32 / 2),
                spawn_y: 0,
                spawn_angle: 90.0,
                spawn_direction: 1,
                allowed_routes: &[0, 1, 2],
            },
            // Left arrow (spawn from East, moving West)
            VehicleRouting {
                // spawn_x: 800,
                // spawn_y: 300 - (ROAD_WIDTH / 4) as i32, // Top lane
                // spawn_angle: 180.0,
                // spawn_direction: 2,
                // allowed_routes: &[0, 1, 2],
                // spawn_x: 800,
                // spawn_y: 300 - (ROAD_WIDTH / 6 + route as u32 * ROAD_WIDTH / 6) as i32, // Top lanes based on route
                // spawn_angle: 180.0,
                // spawn_direction: 2,
                // allowed_routes: &[0, 1, 2],
                spawn_x: 800,
                spawn_y: 300 - ROAD_WIDTH as i32 / 2 + (lane_width as i32 * route + lane_width as i32 / 2),
                spawn_angle: 180.0,
                spawn_direction: 2,
                allowed_routes: &[0, 1, 2],
            },
            // Right arrow (spawn from West, moving East)
            VehicleRouting {
                // spawn_x: 0,
                // spawn_y: 300 + (ROAD_WIDTH / 4) as i32, // Bottom lane
                // spawn_angle: 0.0,
                // spawn_direction: 3,
                // allowed_routes: &[0, 1, 2],
                // spawn_x: 0,
                // spawn_y: 300 + (ROAD_WIDTH / 6 + route as u32 * ROAD_WIDTH / 6) as i32, // Bottom lanes based on route
                // spawn_angle: 0.0,
                // spawn_direction: 3,
                // allowed_routes: &[0, 1, 2],
                spawn_x: 0,
                spawn_y: 300 + ROAD_WIDTH as i32 / 2 - (lane_width as i32 * (route + 1) - lane_width as i32 / 2),
                spawn_angle: 0.0,
                spawn_direction: 3,
                allowed_routes: &[0, 1, 2],
            },
        ];

        // Match keycode to appropriate routing
        match keycode {
            Keycode::Up => {
                let route = routing_options[0].allowed_routes
                    [rng.gen_range(0..routing_options[0].allowed_routes.len())];
                self.vehicles.push(Vehicle::new(
                    routing_options[0].spawn_x,
                    routing_options[0].spawn_y,
                    routing_options[0].spawn_angle,
                    routing_options[0].spawn_direction,
                    route,
                ));
                self.last_vehicle_spawn_time = Instant::now();
            }
            Keycode::Down => {
                let route = routing_options[1].allowed_routes
                    [rng.gen_range(0..routing_options[1].allowed_routes.len())];
                self.vehicles.push(Vehicle::new(
                    routing_options[1].spawn_x,
                    routing_options[1].spawn_y,
                    routing_options[1].spawn_angle,
                    routing_options[1].spawn_direction,
                    route,
                ));
                self.last_vehicle_spawn_time = Instant::now();
            }
            Keycode::Left => {
                let route = routing_options[2].allowed_routes
                    [rng.gen_range(0..routing_options[2].allowed_routes.len())];
                self.vehicles.push(Vehicle::new(
                    routing_options[2].spawn_x,
                    routing_options[2].spawn_y,
                    routing_options[2].spawn_angle,
                    routing_options[2].spawn_direction,
                    route,
                ));
                self.last_vehicle_spawn_time = Instant::now();
            }
            Keycode::Right => {
                let route = routing_options[3].allowed_routes
                    [rng.gen_range(0..routing_options[3].allowed_routes.len())];
                self.vehicles.push(Vehicle::new(
                    routing_options[3].spawn_x,
                    routing_options[3].spawn_y,
                    routing_options[3].spawn_angle,
                    routing_options[3].spawn_direction,
                    route,
                ));
                self.last_vehicle_spawn_time = Instant::now();
            }
            Keycode::R => {
                // Random direction spawn
                let routing = &routing_options[rng.gen_range(0..4)];
                let route = routing.allowed_routes[rng.gen_range(0..routing.allowed_routes.len())];
                self.vehicles.push(Vehicle::new(
                    routing.spawn_x,
                    routing.spawn_y,
                    routing.spawn_angle,
                    routing.spawn_direction,
                    route,
                ));
                self.last_vehicle_spawn_time = Instant::now();
            }
            _ => {}
        }
    }

    fn spawn_vehicle(&mut self, direction: u8) {
        let (x, y, angle) = match direction {
            0 => (420, 580, 180.0), // Bottom to top
            1 => (380, 20, 0.0),    // Top to bottom
            2 => (20, 320, 90.0),   // Left to right
            3 => (780, 280, -90.0), // Right to left
            _ => unreachable!(),
        };

        //let route = rand::thread_rng().gen_range(0..3);
        let route = 1;
        self.vehicles
            .push(Vehicle::new(x, y, angle, direction, route));
    }
}
