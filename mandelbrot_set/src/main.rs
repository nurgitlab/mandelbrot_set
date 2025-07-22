mod my_draw;
use piston_window::*;

struct App {
    x: f64,  // Position of the red square
    y: f64,
}


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Mondelbrot Set", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let app = App { x: 400.0, y: 300.0 };
    let mut k = 1;

    let mut z = my_draw::get_n_for_pixel(0.0, 0.0, k);
    println!("Initial z value: {}", z);
    while let Some(e) = window.next() {
        // Keyboard input handling
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => k+=1,
                Key::Down => k-=1,
                _ => {}
            }

            println!("Square position: ({}, {})", app.x, app.y);
        }


        // Drawing the squares
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g); // Белый фон
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // Red square
                [app.x - 0.5, app.y - 0.5, 1.0, 1.0],
                c.transform,
                g,
            );
        });

         window.draw_2d(&e, |c, g, _| {
            rectangle(
                [1.0, 1.0, 0.0, 1.0], // Yellow square
                [app.x - 75.0, app.y - 75.0, 50.0, 50.0],
                c.transform,
                g,
            );
        });

        window.set_title(format!("k=: {}", k));
    }
}