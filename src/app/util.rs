use crate::app::App;
use crate::helpers::linear_to_log;
use crate::types::{Area, Point};

use piston_window::{Size, Window};

impl App {
    pub fn get_size(&self) -> (f64, f64) {
        let Size { width, height } = self.innards.window.window.size();
        (width, height)
    }

    pub fn set_world_size(&mut self) {
        self.area = Some(self.calc_world_size());
    }

    pub fn calc_world_size(&self) -> Area {
        // first find the smaller edge of the screen so that everything fits on screen
        let (width, height) = self.get_size();

        let smaller_edge = if width < height { width } else { height };

        // now,this edge should have a range of [-2, 2], or a size of 2, starting at -2
        // therefore, the units/pixel will be size/smaller_edge

        let units_per_pixel = 4.0 / smaller_edge;

        // finally, set the boundaries

        let half_x_size = (units_per_pixel * width) / 2.0;
        let half_y_size = (units_per_pixel * height) / 2.0;

        Area {
            start: Point {
                x: -half_x_size,
                y: -half_y_size,
            },
            end: Point {
                x: half_x_size,
                y: half_y_size,
            },
        }
    }

    

    pub fn set_zoom(&mut self, new_zoom: i8) {
        if self.area.is_none() {
            self.set_world_size();
        }

        let area = self.area.unwrap();

        let (width, height) = self.get_size();
        self.zoom = new_zoom;
        let offset = linear_to_log(self.zoom as f64);

        let prev_center = Point {
            x: (area.start.x + area.end.x) / 2.0,
            y: (area.start.y + area.end.y) / 2.0,
        };

        let smaller_edge = if width < height { width } else { height };
        let units_per_pixel = 4.0 / smaller_edge;
        let offset_x = offset * (units_per_pixel * width);
        let offset_y = offset * (units_per_pixel * height);

        self.area = Some(Area {
            start: Point {
                x: prev_center.x - offset_x,
                y: prev_center.y - offset_y,
            },
            end: Point {
                x: prev_center.x + offset_x,
                y: prev_center.y + offset_y,
            },
        });
    }
}
