extern crate rand;
extern crate sdl2;
extern crate scraper;
 
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
    let doc = Html::parse_document(&String::from_utf8_lossy(fan_html));
   // println!("{:#?}", doc);

    let mut hzcanvas = doc.select(&Selector::parse("#hzcanvas").unwrap()).next().unwrap();
    let childs = hzcanvas.children().next().unwrap().children();
    let mut count = 0;
    for child in childs{
        count += 1;
        
        //println!("{:?}", node.as_element().unwrap().attr("style"));

        let eref = ElementRef::wrap(child).unwrap();
        let element = eref.value();
        let mut style = HashMap::new();
        for s in element.attr("style").unwrap().split(";"){
            
            //println!("s={}", s);
            if s.len()>0{
                let mut pair = s.split(":");
                style.insert(pair.next().unwrap(), pair.next().unwrap());
            }
        }
        let text = eref.inner_html();
        if text.len()>0{
            println!("text={} left={:?},top={:?}", text, style.get(&"left"), style.get(&"top"));
        }
    }
    
}
