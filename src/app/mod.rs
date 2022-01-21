mod draw;
mod input;
mod util;

use crate::types::{App, Innards};

use ::image::ImageBuffer;
use piston_window::{
    Button, ButtonEvent, Event, EventSettings, Events, G2dTexture, MouseButton, MouseCursorEvent,
    MouseRelativeEvent, PistonWindow, PressEvent, ReleaseEvent, RenderEvent, Size, Texture,
    TextureContext, TextureSettings, Window,
};

impl App {
    pub fn new(mut window: PistonWindow) -> App {
        let Size { width, height } = window.window.size();

        let canvas = ImageBuffer::new(width as u32, height as u32);
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };
        let texture: G2dTexture =
            Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

        App {
            is_moving: false,
            resolution_scale: 30,
            zoom: 7,
            area: None,
            innards: Innards {
                window,
                texture,
                texture_context,
                canvas,
            },
        }
    }

    fn render(&mut self, e: &Event) {
        use graphics::{clear, image};

        self.innards
            .texture
            .update(&mut self.innards.texture_context, &self.innards.canvas)
            .unwrap();

        self.innards.window.draw_2d(e, |c, gl, device| {
            self.innards.texture_context.encoder.flush(device);

            clear([1.0; 4], gl);
            image(&self.innards.texture, c.transform, gl);
        });
    }

    pub fn run(&mut self) {
        self.set_world_size();
        let mut events = Events::new(EventSettings::new());

        while let Some(event) = events.next(&mut self.innards.window) {
            // render out the drawn canvas
            if let Some(_) = event.render_args() {
                self.render(&event);
            }

            // Mouse Clicked
            if let Some(button) = event.press_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    self.is_moving = true;
                }
            };

            // Mouse Released
            if let Some(button) = event.release_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    self.is_moving = false;
                }
            };

            // only redraw when cursor is moved
            if let Some(_) = event.mouse_cursor_args() {
                self.draw();
            }

            // use inputs
            if let Some(args) = event.button_args() {
                self.manage_input(args);
                self.draw();
            }
            if let Some(args) = event.mouse_relative_args() {
                self.manage_pan(args);
                self.draw();
            }
        }
    }
}
