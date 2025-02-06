use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::BlendMode;
use sdl2::ttf::Sdl2TtfContext; // Import Sdl2TtfContext

pub fn draw_panel(canvas: &mut Canvas<Window>,
    passed_vehicles: u32,
    max_velocity: f64,
    min_velocity: f64,
    max_time: String,
    min_time: String,
    close_calls: u32,
    ttf_context: &Sdl2TtfContext) {
    // Load the fonts.  The context is passed in, so it's valid.
    let title_font = ttf_context.load_font("./src/assets/fonts/Roboto-Bold.ttf", 32).unwrap();
    let regular_font = ttf_context.load_font("./src/assets/fonts/Roboto-Regular.ttf", 24).unwrap();

    canvas.set_blend_mode(BlendMode::Blend);
    let panel_color = sdl2::pixels::Color::RGBA(0, 0, 0, 128);
    let border_color = sdl2::pixels::Color::RGB(255, 255, 255);
    let title_color = sdl2::pixels::Color::RGB(255, 255, 255);

    // Drawing the panel
    canvas.set_draw_color(panel_color);
    let panel_rect = Rect::new(180, 100, 450, 400);
    let _ = canvas.fill_rect(panel_rect);

    let texture_creator = canvas.texture_creator();

    // Drawing the title text
    let surface = title_font
        .render("Simulation Complete")
        .blended(title_color)
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let text_rect = Rect::new(300, 160, 200, 40);
    canvas.copy(&texture, None, Some(text_rect)).unwrap();

    //  Drawing the vehicles passed text
    let vehicles_text = format!("Max number of vehicles passed intersection: {}", passed_vehicles);
    let vehicles_surface = regular_font
        .render(&vehicles_text)
        .blended(title_color)
        .unwrap();
    let vehicles_texture = texture_creator
        .create_texture_from_surface(&vehicles_surface)
        .unwrap();
    let vehicles_rect = Rect::new(250, 220, 300, 30);
    canvas.copy(&vehicles_texture, None, Some(vehicles_rect)).unwrap();

    // Drawing max velocity of all vehicles text
    let max_velocity_text = format!("Max velocity of all vehicles passed intersection: {} Units", max_velocity);
    let max_velocity_surface = regular_font
        .render(&max_velocity_text)
        .blended(title_color)
        .unwrap();
    let max_velocity_texture = texture_creator
        .create_texture_from_surface(&max_velocity_surface)
        .unwrap();
    let max_velocity_rect = Rect::new(250, 260, 300, 30);
    canvas.copy(&max_velocity_texture, None, Some(max_velocity_rect)).unwrap();

    // Drawing min velocity of all vehicles text
    let min_velocity_text = format!("Min velocity of all vehicles passed intersection: {} Units", min_velocity);
    let min_velocity_surface = regular_font
        .render(&min_velocity_text)
        .blended(title_color)
        .unwrap();
    let min_velocity_texture = texture_creator
        .create_texture_from_surface(&min_velocity_surface)
        .unwrap();
    let min_velocity_rect = Rect::new(250, 300, 300, 30);
    canvas.copy(&min_velocity_texture, None, Some(min_velocity_rect)).unwrap();

    // Drawing max time that the vehicle took to pass the intersection text
    let max_time_text = format!("Max time that the vehicle took to pass the intersection: {} seconds", max_time);
    let max_time_surface = regular_font
        .render(&max_time_text)
        .blended(title_color)
        .unwrap();
    let max_time_texture = texture_creator
        .create_texture_from_surface(&max_time_surface)
        .unwrap();
    let max_time_rect = Rect::new(250, 340, 300, 30);
    canvas.copy(&max_time_texture, None, Some(max_time_rect)).unwrap();

    // Drawing min time that the vehicle took to pass the intersection text
    let min_time_text = format!("Min time that the vehicle took to pass the intersection: {} seconds", min_time);
    let min_time_surface = regular_font
        .render(&min_time_text)
        .blended(title_color)
        .unwrap();
    let min_time_texture = texture_creator
        .create_texture_from_surface(&min_time_surface)
        .unwrap();
    let min_time_rect = Rect::new(250, 380, 300, 30);
    canvas.copy(&min_time_texture, None, Some(min_time_rect)).unwrap();

    // Drawing the close calls when two vehicles were close to each (less than safe distance) other text
    let close_calls_text = format!("Close calls when two vehicles were close to each other (less than safe distance): {}", close_calls);
    let close_calls_surface = regular_font
        .render(&close_calls_text)
        .blended(title_color)
        .unwrap();
    let close_calls_texture = texture_creator
        .create_texture_from_surface(&close_calls_surface)
        .unwrap();
    let close_calls_rect = Rect::new(200, 420, 400, 30);
    canvas.copy(&close_calls_texture, None, Some(close_calls_rect)).unwrap();

    // Drawing the border
    canvas.set_draw_color(border_color);
    let _ = canvas.draw_rect(panel_rect);
    canvas.present(); // Important: Present the canvas to show changes

}