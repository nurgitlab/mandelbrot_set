 pub fn get_n_for_pixel(x: f64, y: f64, n: i32) -> i32 {
    let mut x1 = x;
    let mut y1 = y;
    let mut ans = n;

    while (x1 * x1 + y1 * y1 <= 4.0) && (ans > 0) {
        let temp_x = x1 * x1 - y1 * y1;
        y1 = 2.0 * x1 * y1;
        x1 = temp_x;

        ans-= 1; 
    }

    return ans; 
}