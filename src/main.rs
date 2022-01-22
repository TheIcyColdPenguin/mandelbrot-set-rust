#![windows_subsystem = "windows"]

use opengl_graphics::OpenGL;

use piston_window::{PistonWindow, WindowSettings};

mod app;
mod constants;
mod helpers;
mod types;

use types::App;

fn main() {
    let opengl = OpenGL::V4_1;

    let window: PistonWindow = WindowSettings::new("Mandelbrot", [500, 500])
        .exit_on_esc(true)
        .fullscreen(true)
        .vsync(true)
        .resizable(false)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut app = App::new(window);

    app.run();
}
