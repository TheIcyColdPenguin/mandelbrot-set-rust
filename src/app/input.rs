use crate::helpers::linear_to_log;
use crate::types::{App, Area, Point};

use piston_window::{Button, ButtonArgs, ButtonState, Key};

impl App {
    pub fn manage_input(&mut self, args: ButtonArgs) -> bool {
        if args.state != ButtonState::Press {
            return false;
        }

        if let Button::Keyboard(key) = args.button {
            match key {
                x if x == Key::Plus || x == Key::Equals => self.set_zoom(self.zoom + 1),
                x if x == Key::Minus || x == Key::Underscore => self.set_zoom(self.zoom - 1),
                x if [
                    Key::D1,
                    Key::D2,
                    Key::D3,
                    Key::D4,
                    Key::D5,
                    Key::D6,
                    Key::D7,
                    Key::D8,
                ]
                .contains(&x) =>
                {
                    // subtract 0x30 to get the actual number from the key code
                    self.resolution_scale = (10 - (x as u8 - 0x30)) * 2;
                }
                Key::D9 => self.resolution_scale = 1,
                _ => return false,
            }
        }

        true
    }
    pub fn manage_pan(&mut self, pos: [f64; 2]) -> bool {
        if self.is_moving {
            let offset = linear_to_log(self.zoom as f64);
            let movement = Point {
                x: offset * -1.39e-2 * pos[0],
                y: offset * -1.39e-2 * pos[1],
            };

            let prev_area = self.area.unwrap();

            self.area = Some(Area {
                start: Point {
                    x: prev_area.start.x + movement.x,
                    y: prev_area.start.y + movement.y,
                },
                end: Point {
                    x: prev_area.end.x + movement.x,
                    y: prev_area.end.y + movement.y,
                },
            });
            true
        } else {
            false
        }
    }
}
