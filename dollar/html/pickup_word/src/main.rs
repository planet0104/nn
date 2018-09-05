#[macro_use]
extern crate stdweb;
extern crate bincode;

mod controller;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ClickEvent, IEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::html_element::InputElement;
use stdweb::web::INode;
use stdweb::web::{document, CanvasRenderingContext2d, Element};
use stdweb::web::TextAlign;
use stdweb::web::TextBaseline;

use controller::{Controller, Context2D};

struct CtrlContext2d{
    context: CanvasRenderingContext2d
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
}

fn main() {
    stdweb::initialize();
    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let mut controller = Controller::new(Box::new(CtrlContext2d{context: canvas.get_context().unwrap()}));
    controller.init();
    stdweb::event_loop();
}
