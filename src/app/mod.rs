mod draw;
mod input;
mod util;

use crate::types::{App, Complex, Innards};

use ::image::ImageBuffer;
use piston_window::{
    keyboard::ModifierKey, Button, ButtonEvent, Event, EventSettings, Events, G2dTexture,
    MouseButton, MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent, PistonWindow, PressEvent,
    ReleaseEvent, RenderEvent, Size, Texture, TextureContext, TextureSettings, Window,
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
            is_mandelbrot_set: true,
            julia_pos: Complex {
                real: 0.0,
                imag: 0.0,
            },
            resolution_scale: 30,
            zoom: 7,
            area: None,
            modifiers: ModifierKey::NO_MODIFIER,
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
            self.modifiers.event(&event);

            // render out the drawn canvas
            if let Some(_) = event.render_args() {
                self.render(&event);
            }

            // Mouse Clicked
            if let Some(button) = event.press_args() {
                self.manage_mouse_click(button);
            };

            // Mouse Released
            if let Some(button) = event.release_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    self.is_moving = false;
                }
            };

            // only redraw when cursor is moved
            if let Some(pos) = event.mouse_cursor_args() {
                self.draw(Some(pos));
            }

            // zoom on scroll
            if let Some(scroll) = event.mouse_scroll_args() {
                self.manage_scroll(scroll);
                self.draw(None);
            }

            // use inputs
            if let Some(args) = event.button_args() {
                let updated = self.manage_input(args);
                if updated {
                    self.draw(None);
                }
            }
            if let Some(args) = event.mouse_relative_args() {
                let updated = self.manage_pan(args);
                if updated {
                    self.draw(None);
                }
            }
        }
    }
}
