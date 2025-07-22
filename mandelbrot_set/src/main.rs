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

    let mut app = App { x: 400.0, y: 300.0 };

    while let Some(e) = window.next() {
        // Keyboard input handling
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => app.y -= 10.0,
                Key::Down => app.y += 10.0,
                Key::Left => app.x -= 10.0,
                Key::Right => app.x += 10.0,
                _ => {}
            }

            println!("Координаты квадрата: ({}, {})", app.x, app.y);

        }


        // Drawing the squares
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g); // Белый фон
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // Red square
                [app.x - 0.5, app.y - 0.5, 4.0, 4.0],
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

        window.set_title(format!("Number: {}", 42));
    }
}