#[macro_use]
extern crate stdweb;
extern crate bincode;
#[macro_use]
extern crate lazy_static;

mod controller;

use std::rc::Rc;
use std::cell::RefCell;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, IEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::html_element::InputElement;
use stdweb::web::INode;
use stdweb::web::{document, CanvasRenderingContext2d, Element};
use stdweb::web::html_element::ImageElement;
use stdweb::web::TextAlign;
use stdweb::web::TextBaseline;
use stdweb::web::set_timeout;
use stdweb::web::window;

use controller::{Controller, Context2D};

struct CtrlContext2d{
    controller: Controller,
    context: CanvasRenderingContext2d,
    brush: ImageElement,
}

impl CtrlContext2d {
    // fn animate(&mut self, time: f64, rc: Rc<RefCell<Self>>) {
    //     window().request_animation_frame(move |time| {
    //         rc.borrow_mut().animate(time, rc.clone());
    //     });
    // }
}

impl Context2D for CtrlContext2d{
    fn set_font(&self, font: &str){
        self.context.set_font(font);
    }
    fn set_fill_style_color(&self, color: &str){
        self.context.set_fill_style_color(color);
    }
    fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64){
        self.context.fill_rect(x, y, width, height);
    }
    fn set_stroke_style_color(&self, color: &str){
        self.context.set_stroke_style_color(color);
    }
    fn set_line_width(&self, line_width: f64){
        self.context.set_line_width(line_width);
    }
    fn begin_path(&self){
        self.context.begin_path();
    }
    fn move_to(&self, x: f64, y: f64){
        self.context.move_to(x, y);
    }
    fn line_to(&self, x: f64, y: f64){
        self.context.line_to(x, y);
    }
    fn stroke(&self){
        self.context.stroke();
    }
    fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64){
        self.context.stroke_rect(x, y, width, height);
    }
    fn set_text_align(&self, text_align: &str){
        match text_align {
            "center" => self.context.set_text_align(TextAlign::Center),
            "end" => self.context.set_text_align(TextAlign::End),
            "left" => self.context.set_text_align(TextAlign::Left),
            "right" => self.context.set_text_align(TextAlign::Right),
            "start" => self.context.set_text_align(TextAlign::Start),
            _ => ()
        };
    }
    fn set_text_baseline(&self, text_baseline: &str){
        match text_baseline {
            "alphabetic" => self.context.set_text_baseline(TextBaseline::Alphabetic),
            "bottom" => self.context.set_text_baseline(TextBaseline::Bottom),
            "hanging" => self.context.set_text_baseline(TextBaseline::Hanging),
            "ideographic" => self.context.set_text_baseline(TextBaseline::Ideographic),
            "middle" => self.context.set_text_baseline(TextBaseline::Middle),
            "top" => self.context.set_text_baseline(TextBaseline::Top),
            _ => ()
        };
    }
    fn fill_text(&self, text: &str, x: f64, y: f64, max_width: Option<f64>){
        self.context.fill_text(text, x, y, max_width);
    }
    fn canvas_width(&self) -> u32{
        self.context.get_canvas().width()
    }
    fn canvas_height(&self) -> u32{
        self.context.get_canvas().height()
    }
    fn set_line_dash(&self, segments: Vec<f64>){
        self.context.set_line_dash(segments);
    }
    fn save(&self){
        self.context.save();
    }
    fn restore(&self){
        self.context.restore();
    }
    fn scale(&self, x: f64, y: f64){
        self.context.scale(x, y);
    }
    fn translate(&self, x: f64, y: f64){
        self.context.translate(x, y);
    }
    fn darw_brush(&self, x:f64, y:f64){
        self.context.draw_image(self.brush.clone(), x, y).unwrap();
    }
    fn brush_height(&self) -> f64{
        self.brush.height() as f64
    }

    fn start_animation(&self, ctx: Rc<RefCell<Context2D>>, delay:u32){
        self.controller.render(self);
        if self.controller.update(){
            
        }
        
    }

    fn get_controller_mut(&mut self) -> &mut Controller{
        &mut self.controller
    }

    // fn start_animation(&self, cb: fn(), delay:u32){
    //     let u = {let mut ctrl = controller.borrow_mut();
    //         let u = ctrl.update();
    //         ctrl.render();
    //         u
    //     };
    //     if u{
    //         set_timeout(move ||{
    //             controller.borrow_mut().animate();
    //         }, delay);
    //     }
        
    //     // let step = ||{
    //     //     let u = CONTROLLER.update();
    //     //     CONTROLLER.render();
    //     //     if u{
    //     //         set_timeout(step, delay);        
    //     //     }
    //     // };
    //     // set_timeout(step, delay);
    // }
}

fn main() {
    stdweb::initialize();
    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let bursh = ImageElement::new();
    bursh.set_src("brush.png");

    let context2d = Rc::new(RefCell::new(CtrlContext2d{
        context: canvas.get_context().unwrap(),
        brush: bursh,
        controller: Controller::new()
    }));
    context2d.borrow_mut().controller.init(context2d.clone());

    // let state = Rc::new(RefCell::new(State{
    //     controller:Controller::new(}));
    
    //state.borrow_mut().controller;

    //let controller = 
    //controller.borrow_mut().init(controller.clone());
    
    stdweb::event_loop();
}
