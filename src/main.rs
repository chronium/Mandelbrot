extern crate minifb;

use minifb::{Key, WindowOptions};

const WIDTH: usize = 640 * 2;
const HEIGHT: usize = 360 * 2;

#[inline]
fn map(v: f64, vmin: f64, vmax: f64, rmin: f64, rmax: f64) -> f64 {
    rmin + (rmax - rmin) * ((v - vmin) / (vmax - vmin))
}

#[inline]
fn color(r: f64, g: f64, b: f64) -> u32 {
    (0xFF << 24 | (((r * 255f64) as u32) << 16) | (((g * 255f64) as u32) << 8) |
     ((b * 255f64) as u32)) as u32
}

#[inline]
fn clamp(value: f64) -> f64 {
    value.max(0f64).min(1f64)
}

#[inline]
fn mandelbrot(px: usize, py: usize) -> u32 {
    let x0: f64 = map(px as f64, 0f64, WIDTH as f64, -2.5f64, 1f64);
    let y0: f64 = map(py as f64, 0f64, HEIGHT as f64, -1f64, 1f64);
    let mut x = 0f64;
    let mut y = 0f64;
    let mut iter: f64 = 0f64;
    let max_iter: f64 = 64f64;
    while x * x + y * y < 4f64 && iter < max_iter {
        let xt: f64 = x * x - y * y + x0;
        y = 2f64 * x * y + y0;
        x = xt;
        iter += 1f64;
    }

    if iter < max_iter {
        let col = clamp(iter / max_iter);
        return color(0f64, col, 0f64);
    }
    0
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = match minifb::Window::new("Mandelbrot fractal - ESC to exit",
                                               WIDTH,
                                               HEIGHT,
                                               WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let mut i: usize = 0;

    while i < WIDTH * HEIGHT {
        let x: usize = i % WIDTH;
        let y: usize = i / WIDTH;

        let ind: usize = x + y * WIDTH;
        buffer[ind] = mandelbrot(x, y);

        i += 1;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer);
    }
}
