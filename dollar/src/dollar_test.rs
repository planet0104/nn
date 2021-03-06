use piston_window::*;

//多笔画测试
pub fn point_cloud_plus_demo() {
    use pdollarplus::{PDollarPlusRecognizer, Point};
    use std::io;
    let mut pdollarplus = PDollarPlusRecognizer::new();
    println!("内置个数:{}", pdollarplus.point_clouds().len());
    //原始点的数量 66

    let mut points: Vec<Point> = vec![];
    //let mut points = vec![Point::new(38,470),Point::new(36,476),Point::new(36,482),Point::new(37,489),Point::new(39,496),Point::new(42,500),Point::new(46,503),Point::new(50,507),Point::new(56,509),Point::new(63,509),Point::new(70,508),Point::new(75,506),Point::new(79,503),Point::new(82,499),Point::new(85,493),Point::new(87,487),Point::new(88,480),Point::new(88,474),Point::new(87,468)];
    let mut draw = false;
    let mut stroke_id = 0;

    let mut window: PistonWindow = WindowSettings::new("dollar", [700, 500])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            let mut current_stroke_id = 1;
            let mut i = 1;

            while i < points.len() {
                ellipse(
                    [255.0, 0.0, 0.0, 255.0],
                    [points[i - 1].x, points[i - 1].y, 2.0, 2.0],
                    context.transform,
                    graphics,
                );

                if current_stroke_id == points[i].id {
                    line(
                        [0.0, 0.0, 0.0, 255.0],
                        0.5,
                        [points[i - 1].x, points[i - 1].y, points[i].x, points[i].y],
                        context.transform,
                        graphics,
                    );
                } else {
                    current_stroke_id = points[i].id;
                    i += 1;
                }
                i += 1;
            }
        });

        if let Some(button) = event.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true;
                stroke_id += 1;
            }
            if button == Button::Mouse(MouseButton::Right) {
                //开始识别
                let result = pdollarplus.recognize(points.clone());
                println!(
                    "结果: {} ({}) in {} ms.",
                    result.name, result.score, result.ms
                );
                stroke_id = 0;
                points.clear();
            }
            //中间键添加自定义
            if button == Button::Mouse(MouseButton::Middle) {
                let mut name = String::new();
                println!("输入自定义名称:");
                match io::stdin().read_line(&mut name) {
                    Ok(_n) => {
                        name = name.replace("\r\n", "");
                        println!("正在添加:{}", name);
                        pdollarplus.add_gesture(&name, points.clone());
                        println!("添加完成:{}", name);
                    }
                    Err(error) => println!("error: {}", error),
                }
            }
        };
        if let Some(button) = event.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
            }
        };
        if draw {
            if let Some(pos) = event.mouse_cursor_args() {
                points.push(Point::new(pos[0], pos[1], stroke_id));
            };
        }
    }
}

//多笔画测试
fn point_cloud_demo() {
    use pdollar::{PDollarRecognizer, Point};
    use std::io;
    let mut pdollar = PDollarRecognizer::new();
    println!("内置个数:{}", pdollar.point_clouds().len());
    //原始点的数量 66

    let mut points: Vec<Point> = vec![];
    //let mut points = vec![Point::new(38,470),Point::new(36,476),Point::new(36,482),Point::new(37,489),Point::new(39,496),Point::new(42,500),Point::new(46,503),Point::new(50,507),Point::new(56,509),Point::new(63,509),Point::new(70,508),Point::new(75,506),Point::new(79,503),Point::new(82,499),Point::new(85,493),Point::new(87,487),Point::new(88,480),Point::new(88,474),Point::new(87,468)];
    let mut draw = false;
    let mut stroke_id = 0;

    let mut window: PistonWindow = WindowSettings::new("dollar", [700, 500])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            let mut current_stroke_id = 1;
            let mut i = 1;

            while i < points.len() {
                ellipse(
                    [255.0, 0.0, 0.0, 255.0],
                    [points[i - 1].x, points[i - 1].y, 2.0, 2.0],
                    context.transform,
                    graphics,
                );

                if current_stroke_id == points[i].id {
                    line(
                        [0.0, 0.0, 0.0, 255.0],
                        0.5,
                        [points[i - 1].x, points[i - 1].y, points[i].x, points[i].y],
                        context.transform,
                        graphics,
                    );
                } else {
                    current_stroke_id = points[i].id;
                    i += 1;
                }
                i += 1;
            }
        });

        if let Some(button) = event.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true;
                stroke_id += 1;
            }
            if button == Button::Mouse(MouseButton::Right) {
                //开始识别
                let result = pdollar.recognize(points.clone());
                println!(
                    "结果: {} ({}) in {} ms.",
                    result.name, result.score, result.ms
                );
                stroke_id = 0;
                points.clear();
            }
            //中间键添加自定义
            if button == Button::Mouse(MouseButton::Middle) {
                let mut name = String::new();
                println!("输入自定义名称:");
                match io::stdin().read_line(&mut name) {
                    Ok(_n) => {
                        name = name.replace("\r\n", "");
                        println!("正在添加:{}", name);
                        pdollar.add_gesture(&name, points.clone());
                        println!("添加完成:{}", name);
                    }
                    Err(error) => println!("error: {}", error),
                }
            }
        };
        if let Some(button) = event.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
            }
        };
        if draw {
            if let Some(pos) = event.mouse_cursor_args() {
                points.push(Point::new(pos[0], pos[1], stroke_id));
            };
        }
    }
}

//多笔画测试
fn multistroke_demo() {
    use ndollar::{NDollarRecognizer, Point};
    use std::io;
    let mut ndollar = NDollarRecognizer::new(false);
    println!("内置个数:{}", ndollar.multistrokes().len());
    //原始点的数量 66

    let mut strokes: Vec<Vec<Point>> = vec![];
    let mut points: Vec<Point> = vec![];
    //let mut points = vec![Point::new(38,470),Point::new(36,476),Point::new(36,482),Point::new(37,489),Point::new(39,496),Point::new(42,500),Point::new(46,503),Point::new(50,507),Point::new(56,509),Point::new(63,509),Point::new(70,508),Point::new(75,506),Point::new(79,503),Point::new(82,499),Point::new(85,493),Point::new(87,487),Point::new(88,480),Point::new(88,474),Point::new(87,468)];
    let mut draw = false;

    let mut window: PistonWindow = WindowSettings::new("dollar", [700, 500])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            for points in &strokes {
                for i in 1..points.len() {
                    ellipse(
                        [255.0, 0.0, 0.0, 255.0],
                        [points[i - 1].x, points[i - 1].y, 2.0, 2.0],
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
            }

            for i in 1..points.len() {
                ellipse(
                    [255.0, 0.0, 0.0, 255.0],
                    [points[i - 1].x, points[i - 1].y, 2.0, 2.0],
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
            }
            if button == Button::Mouse(MouseButton::Right) {
                //开始识别
                let result = ndollar.recognize(strokes.clone(), false, false, false);
                println!(
                    "结果: {} ({}) in {} ms.",
                    result.name, result.score, result.ms
                );
                strokes.clear();
            }
            //中间键添加自定义
            if button == Button::Mouse(MouseButton::Middle) {
                if strokes.len() <= 1 {
                    println!("至少输入两笔!");
                } else {
                    let mut name = String::new();
                    println!("输入自定义名称:");
                    match io::stdin().read_line(&mut name) {
                        Ok(_n) => {
                            name = name.replace("\n", "");
                            println!("正在添加{}...", name);
                            ndollar.add_gesture(&name, false, strokes.clone());
                            println!("添加完成{}.", name);
                        }
                        Err(error) => println!("error: {}", error),
                    }
                }
            }
        };
        if let Some(button) = event.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
                if points.len() >= 10 {
                    strokes.push(points.clone());
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

//单笔画测试
fn unistroke_demo() {
    use std::io;
    use dollar::{DollarRecognizer, Point};
    let mut doller = DollarRecognizer::new();
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
                points.clear();
                println!("鼠标按下");
            }
            //右键添加
            if button == Button::Mouse(MouseButton::Right) {
                let mut name = String::new();
                println!("输入自定义名称:");
                match io::stdin().read_line(&mut name) {
                    Ok(_n) => {
                        name = name.replace("\r\n", "");
                        println!("正在添加:{}", name);
                        doller.add_gesture(&name, points.clone());
                        println!("添加完成:{}", name);
                    }
                    Err(error) => println!("error: {}", error),
                }
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
            }
        };
        if draw {
            if let Some(pos) = event.mouse_cursor_args() {
                points.push(Point::new(pos[0], pos[1]));
            };
        }
    }
}