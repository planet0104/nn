extern crate rand;
extern crate sdl2;
extern crate scraper;
extern crate serde_json;

use serde_json::{Value, Error};
use serde_json::map::Map;
 
mod cnn;
mod data;
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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("边缘检测", 500, 500)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut controller = controller::Controller::new();
    println!("开始训练网络");
    // match controller.train_network(){
    //     Ok(_) => println!("网络训练成功."),
    //     Err(err) => println!("网络训练失败! {}", err)
    // }
    controller.render(&mut canvas);

    parse_html();

    return;

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

fn parse_html(){
    /*

	hzbh.main('繁', 繁:[17,'0:(162,18) (186,36) (138,96) (96,144) (30,204)#1:(138,96) (420,96) (378,78) (336,96)#2:(144,138) (108,336) (84,354) (108,336) (444,336) (402,324) (366,336)#3:(138,162) (360,162) (390,138) (360,162) (330,360)#4:(192,168) (246,204) (264,228) (270,246)#5:(24,252) (462,252) (420,234) (384,252)#6:(192,252) (246,276) (264,300) (270,324)#7:(528,18) (552,30) (510,96) (474,144) (444,186)#8:(498,114) (726,114) (684,96) (648,114)#9:(654,114) (636,162) (612,216) (582,264) (546,306) (492,354) (438,390)#10:(486,132) (522,210) (552,258) (588,300) (630,336) (660,360) (714,390)#11:(312,360) (348,366) (198,456) (162,468) (198,456) (402,444)#12:(468,384) (498,396) (348,474) (150,564) (114,576) (150,564) (576,540)#13:(480,474) (552,516) (594,552) (618,588)#14:(390,552) (390,708) (378,732) (348,762) (270,702)#15:(234,594) (276,612) (192,672) (120,714) (54,744)#16:(480,606) (540,636) (618,684) (690,738)']});hzbh.flash('繁','fj/fan7');
    */
    let doc = String::from_utf8_lossy(ma_html);
   // println!("{:#?}", doc);
   let s = doc.split("hzbh.main(");
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
                for i in 0..count.parse().unwrap(){
                    string = string.replace(&format!("{}:", i), "");
                }
                println!("{}", string);
                
            }else{
                println!("没有找到花括号");
            }
       }else{
           println!("没有找到);");
       }
   }else{
       println!("没有找到hzbh.main(");
   }
   
}
