use glutin_window::OpenGL;
use graphics::Transformed;
use graphics::types::Scalar;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use crate::color_util;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    pub(crate) color: [f32; 4],
    pub(crate) rotate_clock_wise: bool,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        let new_app = Self {
            gl,
            rotation: 0.0,
            color: [0.0, 0.0, 0.0, 1.0],
            rotate_clock_wise: true,
        };
        new_app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let square_size: Scalar = args.window_size[0] / 3.0;
        let transform: Scalar = square_size / -2.0;

        let square = graphics::rectangle::square(0.0, 0.0, square_size);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        let rotation = self.rotation;
        let new_color = self.color;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(BACKGROUND, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(transform, transform);

            // Draw a box rotating around the middle of the screen.
            graphics::rectangle(new_color, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        if self.rotate_clock_wise {
            self.rotation += 2.0 * args.dt;
        } else {
            self.rotation -= 2.0 * args.dt;
        }
        // Can args.dt be used to update every 2 second?
        self.color = color_util::rainbow_effect(self.color);
    }
}