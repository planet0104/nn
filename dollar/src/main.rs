extern crate piston_window;

use piston_window::*;
mod dollar;
mod ndollar;
use dollar::{DollarRecognizer, Point};

fn main() {
    let doller = DollarRecognizer::new();
    println!("{}", doller.unistrokes().len());
    //原始点的数量 66

    let mut points: Vec<Point> = vec![];
    let mut draw = false;

    let mut window: PistonWindow = WindowSettings::new("dollar", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            for i in 1..points.len() {
                ellipse(
                    [255.0, 0.0, 0.0, 255.0],
                    [points[i - 1].x, points[i - 1].y, 1.0, 1.0],
                    context.transform,
                    graphics,
                );
                line(
                    [0.0, 0.0, 0.0, 255.0],
                    0.5,
                    [points[i - 1].x, points[i - 1].y, points[i].x, points[i].y],
                    context.transform,
                    graphics,
                );
            }
        });

        if let Some(button) = event.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true;
                println!("鼠标按下");
            }
        };
        if let Some(button) = event.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
                println!("鼠标释放");
                if points.len() >= 10 {
                    let result = doller.recognize(points.clone(), false);
                    println!(
                        "结果: {} ({}) in {} ms.",
                        result.name, result.score, result.ms
                    );
                } else {
                    // fewer than 10 points were inputted
                    println!("点太少，再试一次。");
                }
                points.clear();
            }
        };
        if draw {
            if let Some(pos) = event.mouse_cursor_args() {
                points.push(Point::new(pos[0], pos[1]));
            };
        }
    }
}
