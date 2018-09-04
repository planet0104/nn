extern crate reqwest;
extern crate bincode;

mod dollar;
mod ndollar;
mod pdollar;
mod pdollarplus;

use std::fs::File;
use std::io::prelude::*;
use bincode::{serialize, deserialize};
use dollar::DollarRecognizer;

//http://depts.washington.edu/madlab/proj/dollar/
fn main() {
    //unistroke_demo();
    //multistroke_demo();
    //point_cloud_demo();
    //point_cloud_plus_demo();

    /*
    APP名称：拾字
    1、主页由候选区、写字区组成。
    2、候选区，可以切换文章。（文章可以写入、选字、内置分类文章、服务器同步文章）
    3、写字区，可以回退、发音、展示笔画动画。
    4、可查看练字记录，总共写对了多少字。

    收集一年级，到六年级的语文书上的汉字，古诗，文章
    练字app游戏模式，往下掉字！！！填充成语练字，填充成语游戏模式

    程序界面用HTML+JS实现
    rust通过执行js来操作。

    笔画点数组，需要进行填充，笔画点数少于60，要在两点之间插入新的点。

    每个笔画单独进行识别。
    每写一笔之前，创建一个空的单笔识别器，并加入该笔画，然后判定用户写完的笔画和此笔画匹配度，匹配度为得分。
    所有笔画写完以后，创建一个空的多笔画识别器，将文字笔画加入，然后对比用户的笔画，如果不匹配，说明写的不对。

    写字板中文章预览，写对一次标黑色，写错一字整个字标红色，写错一划，单独笔画标红。
    */

    let strokes = get_strokes_from_file('边');
    // use dollar::{DollarRecognizer, Point};
    // let mut doller = DollarRecognizer::new();

    let mut stroke_id = 0;
    use pdollarplus::{PDollarPlusRecognizer, Point};
    let mut pdollarplus = PDollarPlusRecognizer::new();
    for i in 0..strokes.len(){
        pdollarplus.add_gesture(&format!("第{}笔", i), strokes[i].iter().map(|p|{Point::new(p[0] as f64, p[1] as f64, 1)}).collect());
    }

    let mut points: Vec<Point> = vec![];
    let mut draw = false;

    // let mut window: PistonWindow = WindowSettings::new("dollar", [700, 500])
    //     .exit_on_esc(true)
    //     .build()
    //     .unwrap();
    // while let Some(event) = window.next() {
    //     window.draw_2d(&event, |context, graphics| {
    //         clear([1.0; 4], graphics);

    //         //绘制字体
    //         for point in &font_points{
    //             ellipse(
    //                 [150.0, 0.0, 0.0, point.2],
    //                 [point.0 as f64 * 5.0, point.1 as f64 * 5.0, 1.0, 1.0],
    //                 context.transform,
    //                 graphics,
    //             );
    //         }

    //         //虚拟线
    //         for pc in pdollarplus.point_clouds(){
    //             for p in &pc.points{
    //                 ellipse(
    //                     [0.0, 0.0, 255.0, 255.0],
    //                     [p.x*300.0+300.0, p.y*300.0+200.0, 5.0, 5.0],
    //                     context.transform,
    //                     graphics,
    //                 );
    //             }
    //         }

    //         let mut current_stroke_id = 1;
    //         let mut i = 1;

    //         while i < points.len() {
    //             ellipse(
    //                 [255.0, 0.0, 0.0, 255.0],
    //                 [points[i - 1].x, points[i - 1].y, 2.0, 2.0],
    //                 context.transform,
    //                 graphics,
    //             );

    //             if current_stroke_id == points[i].id {
    //                 line(
    //                     [0.0, 0.0, 0.0, 255.0],
    //                     0.5,
    //                     [points[i - 1].x, points[i - 1].y, points[i].x, points[i].y],
    //                     context.transform,
    //                     graphics,
    //                 );
    //             } else {
    //                 current_stroke_id = points[i].id;
    //                 i += 1;
    //             }
    //             i += 1;
    //         }
    //     });

    //     if let Some(button) = event.press_args() {
    //         if button == Button::Mouse(MouseButton::Left) {
    //             draw = true;
    //             stroke_id += 1;
    //         }
    //         if button == Button::Mouse(MouseButton::Right) {
    //             //开始识别
    //             let result = pdollarplus.recognize(points.clone());
    //             println!(
    //                 "结果: {} ({}) in {} ms.",
    //                 result.name, result.score, result.ms
    //             );
    //             stroke_id = 0;
    //             points.clear();
    //         }
    //     };
    //     if let Some(button) = event.release_args() {
    //         if button == Button::Mouse(MouseButton::Left) {
    //             draw = false;
    //         }
    //     };
    //     if draw {
    //         if let Some(pos) = event.mouse_cursor_args() {
    //             points.push(Point::new(pos[0], pos[1], stroke_id));
    //         };
    //     }
    // }

}

// fn test_aaa(){
//     let strokes = get_strokes_from_file('七');
//     let mut window: PistonWindow = WindowSettings::new("dollar", [900, 900])
//         .exit_on_esc(true)
//         .build()
//         .unwrap();
//     while let Some(event) = window.next() {
//         window.draw_2d(&event, |context, graphics| {
//             clear([1.0; 4], graphics);
//             for points in &strokes{
//                 for i in 1..points.len(){
//                     // line(
//                     //     [0.0, 0.0, 0.0, 255.0],
//                     //     1.0,
//                     //     [points[i - 1][0].into(), points[i - 1][1].into(), points[i][0].into(), points[i][1].into()],
//                     //     context.transform,
//                     //     graphics,
//                     // );
//                     ellipse(
//                         [0.0, 255.0, 0.0, 255.0],
//                         [points[i][0].into(), points[i][1].into(), 5.0, 5.0],
//                         context.transform,
//                         graphics,
//                     );
//                 }
//                 ellipse(
//                     [0.0, 0.0, 255.0, 255.0],
//                     [points[0][0].into(), points[0][1].into(), 5.0, 5.0],
//                     context.transform,
//                     graphics,
//                 );
//             }
//         });
//     }
// }

//笔画点数少于60，要在两点之间插入新的点
// fn resample(strokes: Vec<Vec<[i32;2]>>) ->Vec<Vec<[i32;2]>>{
//     use dollar::{resample, Point};
//     let mut new_strokes = vec![];
//     for points in strokes{
//         let mut new_points = vec![];
//         for point in points{
//             new_points.push(Point::new(point[0], point[1]));
//         }
//         let re = resample(new_points, 64);
//         new_strokes.push(re.iter().map(|rp|{
//             [rp.x  as i32, rp.y as i32]
//         }).collect());
//     }

//     new_strokes
// }

fn get_strokes_from_file(ch:char) -> Vec<Vec<[i32;2]>>{
    let mut file = File::open(format!("strokes/{}.stroke", ch)).unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    let mut decoded:Vec<Vec<[i32;2]>> = deserialize(&contents[..]).unwrap();
    decoded
}