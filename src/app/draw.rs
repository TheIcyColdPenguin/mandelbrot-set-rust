use crate::constants::constants::COLORS;
use crate::helpers::{dist_to_inf, screen_to_cart};
use crate::types::{App, Complex};

use ::image::Rgba;

impl App {
    pub fn draw(&mut self, pos: Option<[f64; 2]>) {
        let (width, height) = self.get_size();

        let area = self.area.unwrap_or_else(|| self.calc_world_size());
        let (width, height) = (width as u32, height as u32);
        let res_scale = self.resolution_scale as u32;
        let zero = Complex {
            real: 0.0,
            imag: 0.0,
        };
        let canvas = &mut self.innards.canvas;

        // only update julia pos when not drawing the julia set
        if pos.is_some() && self.is_mandelbrot_set {
            let pos = pos.unwrap();

            self.julia_pos = screen_to_cart(pos.into(), (width as u32, height as u32), &area)
        }

        for x in (0..(width - res_scale + 1)).step_by(res_scale as usize) {
            for y in (0..(height - res_scale + 1)).step_by(res_scale as usize) {
                let temp_col = if self.is_mandelbrot_set {
                    dist_to_inf(
                        &screen_to_cart((x, y).into(), (width, height), &area),
                        &zero,
                    )
                } else {
                    dist_to_inf(
                        &self.julia_pos,
                        &screen_to_cart((x, y).into(), (width, height), &area),
                    )
                };

                let col = COLORS[(temp_col % COLORS.len() as u64) as usize];

                for x_off in 0..res_scale {
                    for y_off in 0..res_scale {
                        canvas.put_pixel(x + x_off, y + y_off, Rgba([col.0, col.1, col.2, 255]));
                    }
                }
            }
        }
    }
}
