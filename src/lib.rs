use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::BlendMode;
//use sdl2::ttf::Sdl2TtfContext;

pub fn draw_panel(canvas: &mut Canvas<Window>, passed_vehicles: u32) {
    let ttf_context = sdl2::ttf::init().unwrap();
    let title_font = ttf_context.load_font("./src/assets/fonts/Roboto-Bold.ttf", 32).unwrap();
    let regular_font = ttf_context.load_font("./src/assets/fonts/Roboto-Regular.ttf", 24).unwrap();

    canvas.set_blend_mode(BlendMode::Blend);
    let panel_color = sdl2::pixels::Color::RGBA(0, 0, 0, 128);
    let border_color = sdl2::pixels::Color::RGB(255, 255, 255);
    let title_color = sdl2::pixels::Color::RGB(255, 255, 255);

    // Drawing the panel
    canvas.set_draw_color(panel_color);
    let panel_rect = Rect::new(
        200,
        150,
        400,
        300,
    );
    let _ = canvas.fill_rect(panel_rect);

    // Drawing the title text
    let surface = title_font
        .render("Simulation Complete")
        .blended(title_color)
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let text_rect = Rect::new(
        300,
        160,
        200,
        40,
    );
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

    // Drawing the border
    canvas.set_draw_color(border_color);
    let _ = canvas.draw_rect(panel_rect);
    canvas.present();
}