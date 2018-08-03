extern crate rand;
extern crate sdl2;

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

mod vector_2d;
mod controller;

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
    match controller.train_network(){
        Ok(_) => println!("网络训练成功."),
        Err(err) => println!("网络训练失败! {}", err)
    }
    controller.render(&mut canvas);

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
