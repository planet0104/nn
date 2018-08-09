extern crate rand;
extern crate sdl2;
extern crate scraper;
extern crate serde_json;

use serde_json::{Value, Error};
use serde_json::map::Map;
use sdl2::pixels::Color;
 
mod cnn;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::{Duration, Instant};
use scraper::{Selector, Html};
use scraper::node::Node;
use scraper::element_ref::ElementRef;
use std::collections::HashMap;

mod vector_2d;
mod controller;

const ma_html:&[u8] = include_bytes!("../ma.html");
const fan_html:&[u8] = include_bytes!("../fan.html");
//字典查询 http://dict.r12345.com/0x91D1.html (可以去到笔顺编号)
//笔顺图片查询: http://bishun.shufaji.com/0x91D1.html
// 1.渲染保存图片 2.从指定位置开始，取每一个笔画
//免费字体:https://www.jianshu.com/p/9da5250a9503
//

///撇
const STROKE_POINTS_PIE:[(i32,i32); 25] = [(25,26),(25,27),(24,28),(24,29),(23,30),(23,31),(22,32),(22,33),(21,34),(20,35),(20,36),(19,37),(18,38),(17,39),(16,40),(16,41),(15,42),(14,43),(13,44),(12,44),(11,45),(10,46),(9,47),(8,47),(7,48)];
///斜捺
const STROKE_POINTS_NA:[(i32,i32); 23] = [(27,26),(27,27),(28,28),(28,29),(29,30),(29,31),(30,32),(30,33),(31,34),(32,35),(32,36),(33,37),(34,38),(35,39),(36,40),(37,41),(38,42),(39,43),(40,44),(41,45),(42,45),(43,46),(44,47)];
//平捺
const STROKE_POINTS_PING_NA:[(i32,i32); 26] = [(2,4),(3,4),(4,5),(5,6),(6,6),(7,7),(8,7),(9,8),(10,8),(11,9),(12,9),(13,10),(14,10),(15,11),(16,11),(17,12),(18,12),(19,12),(20,12),(21,12),(22,12),(23,12),(24,12),(25,12),(26,12),(27,12)];

//斜提
//const STROKE_POINTS_XIE_TI:[(i32,i32); 25] = [(11,43),(12,43),(13,42),(14,42),(15,41),(16,41),(17,40),(18,40),(19,39),(20,39),(21,38),(22,38),(23,38),(24,37),(25,37),(26,36),(27,36),(28,35),(29,35),(30,34),(31,34),(32,33),(33,33),(34,32),(35,32)];
const STROKE_POINTS_XIE_TI:[(i32,i32); 36] = [(6,84),(7,83),(8,82),(8,81),(9,80),(10,79),(11,78),(11,77),(12,76),(13,75),(14,74),(14,73),(15,72),(16,71),(17,70),(17,69),(18,68),(19,67),(20,66),(20,65),(21,64),(22,63),(23,62),(23,61),(24,60),(25,59),(26,58),(27,57),(27,56),(28,55),(29,54),(30,53),(30,52),(31,51),(32,50),(33,49)];

//横撇1
const STROKE_POINTS_HENG_PIE1:[(i32,i32); 35] = [(12,19),(13,19),(14,19),(15,19),(16,19),(17,19),(18,19),(19,19),(20,19),(21,19),(22,19),(23,19),(24,19),(25,19),(26,19),(27,19),(28,19),(29,19),(30,19),(31,19),(32,19),(33,19),(34,19),(35,19),(36,19),(35,20),(34,20),(33,21),(32,22),(31,23),(30,23),(29,24),(28,25),(27,25),(26,26)];

//横撇2
const STROKE_POINTS_HENG_PIE2:[(i32,i32); 54] = [(12,32),(13,32),(14,32),(15,32),(16,32),(17,32),(18,32),(19,32),(20,32),(21,32),(22,32),(23,32),(24,32),(25,32),(26,32),(27,32),(28,32),(29,32),(30,32),(31,32),(32,32),(33,32),(34,32),(35,32),(36,32),(37,32),(35,33),(34,33),(33,34),(32,35),(31,35),(30,36),(29,37),(28,38),(27,38),(26,39),(25,40),(24,40),(23,41),(22,41),(21,42),(20,42),(19,42),(18,43),(17,43),(16,44),(15,44),(14,44),(13,45),(12,45),(11,46),(10,47),(9,48),(8,48)];

//横撇3
const STROKE_POINTS_HENG_PIE3:[(i32,i32); 62] = [(12,20),(13,20),(14,20),(15,20),(16,20),(17,20),(18,20),(19,20),(20,20),(21,20),(22,20),(23,20),(24,20),(25,20),(26,20),(27,20),(28,20),(29,20),(30,20),(31,20),(32,20),(33,20),(34,20),(35,20),(35,21),(35,22),(34,23),(34,24),(34,25),(34,26),(33,27),(33,28),(32,29),(32,30),(31,31),(31,32),(30,33),(30,34),(29,35),(29,36),(28,37),(28,38),(27,39),(26,40),(25,40),(24,41),(23,41),(22,42),(21,43),(20,44),(19,44),(18,45),(17,46),(16,46),(15,47),(14,47),(13,48),(12,48),(11,49),(10,49),(9,50),(8,50)];

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("nn", 520, 520)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut controller = controller::Controller::new();

    // match controller.train_network(){
    //     Ok(_) => println!("网络训练成功."),
    //     Err(err) => println!("网络训练失败! {}", err)
    // }

    controller.clear();
    for point in STROKE_POINTS_XIE_TI.iter(){
        controller.add_point(Point::new(point.0, point.1));
    }
    controller.set_drawing(false);
    println!("{:?}", controller.vectors());

    controller.render(&mut canvas);

    // let (key, draw) = parse_html(&String::from_utf8_lossy(fan_html)).unwrap();
    // canvas.set_draw_color(Color::RGB(255, 255, 255));
    // //canvas.clear();
    // let scale = 0.5;
    // canvas.set_scale(scale, scale).unwrap();
    // for i in 0..draw.len(){
    //     println!("第{}笔:{:?}", i, draw[i]);
    //     canvas.draw_lines(draw[i].as_slice()).unwrap();
    // }
    // canvas.present();

    'mainloop: loop {
            for event in sdl_context.event_pump().unwrap().poll_iter() {
                match event {
                    Event::Quit{..} |
                    Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                        break 'mainloop,
                    Event::MouseButtonDown {..} => {
                        controller.set_drawing(true);
                        controller.render(&mut canvas);
                    }
                    Event::MouseMotion {x, y, ..} => {         
                        if controller.drawing(){
                            controller.add_point(Point::new(x, y));
                            controller.render(&mut canvas);
                        }
                    }
                    Event::MouseButtonUp{..} =>{
                        controller.set_drawing(false);
                        controller.render(&mut canvas);
                    }
                    _ => {}
                }
            }
    }
}

fn parse_html<'a>(html: &str)->Option<(String, Vec<Vec<Point>>)>{
    /*

	hzbh.main('繁', 繁:[17,'0:(162,18) (186,36) (138,96) (96,144) (30,204)#1:(138,96) (420,96) (378,78) (336,96)#2:(144,138) (108,336) (84,354) (108,336) (444,336) (402,324) (366,336)#3:(138,162) (360,162) (390,138) (360,162) (330,360)#4:(192,168) (246,204) (264,228) (270,246)#5:(24,252) (462,252) (420,234) (384,252)#6:(192,252) (246,276) (264,300) (270,324)#7:(528,18) (552,30) (510,96) (474,144) (444,186)#8:(498,114) (726,114) (684,96) (648,114)#9:(654,114) (636,162) (612,216) (582,264) (546,306) (492,354) (438,390)#10:(486,132) (522,210) (552,258) (588,300) (630,336) (660,360) (714,390)#11:(312,360) (348,366) (198,456) (162,468) (198,456) (402,444)#12:(468,384) (498,396) (348,474) (150,564) (114,576) (150,564) (576,540)#13:(480,474) (552,516) (594,552) (618,588)#14:(390,552) (390,708) (378,732) (348,762) (270,702)#15:(234,594) (276,612) (192,672) (120,714) (54,744)#16:(480,606) (540,636) (618,684) (690,738)']});hzbh.flash('繁','fj/fan7');
    */
   let s = html.split("hzbh.main(");
   if let Some(s) = s.skip(1).next(){
       let mut s = s.split(");");
       if let Some(s) = s.next(){
            let s = s.split("{");
            if let Some(s) = s.skip(1).next(){
                //繁:[17, '0:(x,y)..#2:(x,y)..#3..']}
                
                let mut map = s.split(":[");
                let key = map.next().unwrap();
                let mut value = map.next().unwrap().trim_right_matches("']}").split(",'");
                let count = value.next().unwrap();
                let mut string = String::from(value.next().unwrap());
                println!("汉字={}", key);
                println!("笔画数={}", count);
                let mut result = vec![];
                string.replace_range(0..2, "");
                for i in 1..count.parse().unwrap(){
                    string = string.replace(&format!("#{}:", i), "#");
                }
                let arr = string.split("#");
                
                let mut i = 0;
                for b in arr{
                    i+=1;
                    let mut points:Vec<Point> = b.split(" ").map(|p|{
                        let xy:Vec<&str> = p.trim_right_matches(")")
                        .trim_left_matches("(").split(",").collect();
                        Point::new(xy[0].parse().unwrap(), xy[1].parse().unwrap())
                    }).collect();
                    //横线末尾的沟删掉
                    if points.len()>=4 &&
                        points[points.len()-1].y == points[points.len()-3].y{
                        points.pop();
                        points.pop();
                    }
                    //折线中间的突起去掉
                    let mut new_points:Vec<Point> = vec![];
                    for point in points{
                        let len = new_points.len();
                        if len>=2 && new_points[len-2].x == point.x &&
                                        new_points[len-2].y == point.y{
                            new_points.pop();
                        }else{
                            new_points.push(point);
                        }
                    }

                    result.push(new_points);
                }

                return Some((key.to_string(), result));
                
            }else{
                println!("没有找到花括号");
            }
       }else{
           println!("没有找到);");
       }
   }else{
       println!("没有找到hzbh.main(");
   }
   
   None
}
