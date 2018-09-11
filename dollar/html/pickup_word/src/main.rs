#![recursion_limit="256"]
#[macro_use]
extern crate stdweb;
extern crate bincode;
#[macro_use]
extern crate lazy_static;

mod controller;

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, PointerDownEvent, PointerUpEvent, PointerOutEvent, PointerMoveEvent, IEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::html_element::InputElement;
use stdweb::web::INode;
use stdweb::web::{document, CanvasRenderingContext2d, Element};
use stdweb::web::html_element::ImageElement;
use stdweb::web::{TextAlign, FillRule};
use stdweb::web::TextBaseline;
use stdweb::web::set_timeout;
use stdweb::web::window;

use controller::Controller;
use controller::Interface;

thread_local!{
    static CONTROLLER: RefCell<Option<Controller>> = RefCell::new(None);
}
// lazy_static! {
//     static ref CONTROLLER: Option<Controller> = None;
// }

struct CtrlInterface{
    context: CanvasRenderingContext2d,
    brush: ImageElement,
    error_icon: ImageElement,
    ok_icon: ImageElement
}

impl Interface for CtrlInterface{
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

    fn next_frame(&self, delay:u32){
        set_timeout(move ||{
            CONTROLLER.with(|c| {
                c.borrow_mut().as_mut().unwrap().animate(delay);
            });
        }, delay);
    }

    fn log(&self, s:&str){
        js!(console.log(@{s}));
    }

    fn fill_circle(&self, x: f64, y: f64, radius: f64){
        self.context.begin_path();
        self.context.arc(x, y, radius, 0.0, 360.0, false);
        self.context.fill(FillRule::NonZero);
    }

    fn rotate(&self, angle: f64){
        self.context.rotate(angle);
    }
    fn window_width(&self) -> i32{
        window().inner_width()
    }
    fn window_height(&self) -> i32{
        window().inner_height()
    }
    fn set_canvas_width(&self, width: u32){
        self.context.get_canvas().set_width(width);
    }
    fn set_canvas_height(&self, height: u32){
        self.context.get_canvas().set_height(height);
    }
    fn log_div(&self, s:&str){
        js!( document.getElementById("log").innerHTML = @{s}+"<br/>"+document.getElementById("log").innerHTML );
    }
    fn show_error_icon(&self, x:f64, y:f64){
        self.context.draw_image(self.error_icon.clone(), x, y).unwrap();
    }
    fn show_ok_icon(&self, x:f64, y:f64){
        self.context.draw_image(self.ok_icon.clone(), x, y).unwrap();
    }
}

fn main() {
    stdweb::initialize();
    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let brush = ImageElement::new();
    brush.set_src("brush.png");
    let error_icon = ImageElement::new();
    error_icon.set_src("error.png");
    let ok_icon = ImageElement::new();
    ok_icon.set_src("ok.png");

    let controller = Controller::new(Box::new(CtrlInterface{
        context: canvas.get_context().unwrap(),
        brush,
        error_icon,
        ok_icon
    }));

    CONTROLLER.with(|c| {
        *c.borrow_mut() = Some(controller);
        c.borrow_mut().as_mut().unwrap().init();
    });

    canvas.add_event_listener( move |event: PointerDownEvent| {
        CONTROLLER.with(|c| {
            c.borrow_mut().as_mut().unwrap().on_pointer_down(event.client_x(), event.client_y(), event.offset_x(), event.offset_y());
        });
    });
    canvas.add_event_listener( move |event: PointerMoveEvent| {
        CONTROLLER.with(|c| {
            c.borrow_mut().as_mut().unwrap().on_pointer_move(event.client_x(), event.client_y(), event.offset_x(), event.offset_y());
        });
    });
    canvas.add_event_listener( move |_event: PointerOutEvent| {
        CONTROLLER.with(|c| {
            c.borrow_mut().as_mut().unwrap().on_pointer_out();
        });
    });
    canvas.add_event_listener( move |_event: PointerUpEvent| {
        CONTROLLER.with(|c| {
            c.borrow_mut().as_mut().unwrap().on_pointer_up();
        });
    });

    //触摸事件
    let on_touch_event = |event_type:String, client_x:i32, client_y:i32, offset_x:f64, offset_y:f64| {
        CONTROLLER.with(|c|{
            match event_type.as_str(){
                "touchstart" => c.borrow_mut().as_mut().unwrap().on_pointer_down(client_x, client_y, offset_x, offset_y),
                "touchmove" => c.borrow_mut().as_mut().unwrap().on_pointer_move(client_x, client_y, offset_x, offset_y),
                "touchend" => c.borrow_mut().as_mut().unwrap().on_pointer_up(),
                "touchcancel" => c.borrow_mut().as_mut().unwrap().on_pointer_out(),
                _ => ()
            }
        });
    };
    js! {
        var on_touch_event = @{on_touch_event};
        var canvas = document.getElementById("canvas");
        canvas.addEventListener("touchstart", function(e){
            e.preventDefault();
            var touch = e.targetTouches[0];
            on_touch_event("touchstart", touch.clientX, touch.clientY, touch.clientX-canvas.offsetLeft, touch.clientY-canvas.offsetTop);
        }, false);
        canvas.addEventListener("touchmove", function(e){
            e.preventDefault();
            var touch = e.targetTouches[0];
            on_touch_event("touchmove", touch.clientX, touch.clientY, touch.clientX-canvas.offsetLeft, touch.clientY-canvas.offsetTop);
        },false);
        canvas.addEventListener("touchend", function(e){
            on_touch_event("touchend", 0, 0, 0, 0);
        },false);
        canvas.addEventListener("touchcancel", function(e){
            on_touch_event("touchcancel", 0, 0, 0, 0);
        },false);
    }
    
    stdweb::event_loop();
}