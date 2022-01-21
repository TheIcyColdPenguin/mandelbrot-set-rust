use image::{ImageBuffer, Rgba};
use piston_window::{G2dTexture, PistonWindow, TextureContext};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<(u32, u32)> for Point {
    fn from(point: (u32, u32)) -> Point {
        Point {
            x: point.0 as f64,
            y: point.1 as f64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub start: Point,
    pub end: Point,
}

pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn magnitude(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn square(&self) -> Complex {
        Complex {
            real: self.real * self.real - self.imag * self.imag,
            imag: 2.0 * self.real * self.imag,
        }
    }

    pub fn next_iteration(&self, curr_complex: &Complex) -> Complex {
        let square = self.square();

        Complex {
            real: square.real + curr_complex.real,
            imag: square.imag + curr_complex.imag,
        }
    }
}

pub struct Innards {
    pub window: PistonWindow,
    pub texture: G2dTexture,
    pub texture_context: TextureContext<
        gfx_device_gl::Factory,
        gfx_device_gl::Resources,
        gfx_device_gl::CommandBuffer,
    >,
    pub canvas: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

pub struct App {
    pub is_moving: bool,
    pub resolution_scale: u8,
    pub zoom: i8,
    pub area: Option<Area>,
    pub modifiers: ModifierKey,
    pub innards: Innards,
}
