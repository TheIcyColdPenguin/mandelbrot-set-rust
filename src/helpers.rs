use crate::types::{Area, Complex, Point};

use crate::constants::constants;

pub fn dist_to_inf(c: &Complex, z_initial: &Complex) -> u64 {
    let mut n: u64 = 0;

    let mut z_next = z_initial.next_iteration(c);

    while z_next.magnitude() < constants::INFINITY && n < constants::MAX_ITERATIONS {
        z_next = z_next.next_iteration(c);
        n += 1;
    }

    if n < constants::MAX_ITERATIONS {
        let log_zn = z_next.magnitude().ln() / 2.0;
        let nu = (log_zn / constants::LN_16) / constants::LN_16;
        n = n + 1 - nu.floor() as u64;
    };

    n
}

pub fn map_range(
    n: f64,
    initial_range_start: f64,
    initial_range_stop: f64,
    new_range_start: f64,
    new_range_end: f64,
) -> f64 {
    let initial_range = initial_range_stop - initial_range_start;
    let new_range = new_range_end - new_range_start;

    ((n - initial_range_start) / initial_range).mul_add(new_range, new_range_start)
}

pub fn screen_to_cart(
    Point { x, y }: Point,
    (screen_width, screen_height): (u32, u32),
    Area { start, end }: &Area,
) -> Complex {
    Complex {
        real: map_range(x, 0.0, screen_width as f64, start.x, end.x),
        imag: map_range(y, 0.0, screen_height as f64, start.y, end.y),
    }
}

pub fn linear_to_log(val: f64) -> f64 {
    map_range(
        val,
        100.0,
        0.0,
        constants::LN_1E_NEGATIVE_10,
        constants::LN_2_3,
    )
    .exp()
}
