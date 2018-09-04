#[macro_use]
extern crate stdweb;

mod controller;

// use stdweb::traits::*;
// use stdweb::unstable::TryInto;
// use stdweb::web::event::{ClickEvent, IEvent};
// use stdweb::web::html_element::CanvasElement;
// use stdweb::web::html_element::InputElement;
// use stdweb::web::INode;
// use stdweb::web::{document, CanvasRenderingContext2d, Element};

const DATA:&[u8] = include_bytes!("../万.stroke");

fn main() {
    stdweb::initialize();
    let len = format!("DATA长度:{}",DATA.len());
    js!{
        console.log("DATA长度:", @{len});
    };

    stdweb::event_loop();
}
