use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Transformed;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::PressEvent;
use piston::MouseButton;
use piston::Button;

mod color_util;
mod app;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("spinning-square", [500, 500])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let graphics = GlGraphics::new(opengl);
    let mut app = app::App::new(graphics);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                color_util::print_color("PRINT", app.color);
                app.color = color_util::reset_color();
            }
            if button == Button::Mouse(MouseButton::Right) {
                app.rotate_clock_wise = !app.rotate_clock_wise;
            }
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

