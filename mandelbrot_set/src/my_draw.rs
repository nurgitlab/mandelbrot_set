 pub fn get_n_for_pixel(x: f64, y: f64, n: i32) -> i32 {
    let mut x1 = x;
    let mut y1 = y;
    let mut k = n;

    while x1 * x1 + y1 * y1 < 4.0 && k > 0 {
        let temp_x = x1 * x1 - y1 * y1;
        y1 = 2.0 * x1 * y1;
        x1 = temp_x;

        x1 += x;
        y1 += y;

        k-= 1; 
    }

    return k; 
}

pub fn get_color(n: u32, max_iter: u32) -> [u8; 3] {
    if n == max_iter {
        return [0, 0, 0];  
    }

    let t = n as f32 / max_iter as f32;
    
    let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
    let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
    let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
    
    [r, g, b]
}