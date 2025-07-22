use minifb::{Window, WindowOptions};
mod my_draw;

fn main() {
    let width = 680;
    let height: usize = 680;
    let k = 1200; // Count of iterations for Mandelbrot set

    let mut buffer: Vec<u32> = vec![0; width * height];

     for y in 0..height {
            for x in 0..width {
                let x_scaled = (x as f64 / 240.0) - 2.0; // Scale x to [-2, 1]
                let y_scaled = (y as f64 / 240.0)  - 1.5; // Scale y to [-1.5, 1.5]

                let z = my_draw::get_n_for_pixel(x_scaled, y_scaled, k);

                let color = my_draw::get_color(z as u32, 1024);

                let r = color[0] as u32;
                let g = color[1] as u32;
                let b = color[2] as u32;
                buffer[y * width + x] = (r << 16) | (g << 8) | b;
            }
    }

    let mut window = Window::new(
        "Pixel Drawing",
        width,
        height,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}