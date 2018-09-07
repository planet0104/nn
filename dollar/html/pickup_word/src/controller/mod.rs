mod pdollarplus;
const DATA:&[u8] = include_bytes!("../../stroke_data");
use bincode::deserialize;
use std::collections::HashMap;
use self::pdollarplus::{Point, resample};

/*

controller 功能:
1、画板控制
接收canvas的事件，调用canvas绘图。
2、后选区
设置后选区文本内容，已写字，当前字，未写字的显示等。
3、功能区
接收功能区按钮点击事件，切换状态

Window功能
1、提供canvas api
2、提供后选区操作api
3、其他界面操作功能

 */

pub trait Interface{
    fn set_font(&self, font: &str);
    fn set_fill_style_color(&self, color: &str);
    fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64);
    fn set_stroke_style_color(&self, color: &str);
    fn set_line_width(&self, line_width: f64);
    fn begin_path(&self, );
    fn move_to(&self, x: f64, y: f64);
    fn line_to(&self, x: f64, y: f64);
    fn stroke(&self);
    fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64);
    fn set_text_align(&self, text_align: &str);
    fn set_text_baseline(&self, text_baseline: &str);
    fn fill_text(&self, text: &str, x: f64, y: f64, max_width: Option<f64>);
    fn canvas_width(&self) -> u32;
    fn canvas_height(&self) -> u32;
    fn set_line_dash(&self, segments: Vec<f64>);
    fn save(&self);
    fn restore(&self);
    fn scale(&self, x: f64, y: f64);
    fn translate(&self, x: f64, y: f64);
    //控制画刷
    fn darw_brush(&self, x:f64, y:f64);
    fn brush_height(&self) -> f64;
    fn next_frame(&self, delay: u32);
    fn log(&self, str:&str);
}

pub struct Controller{
    interface: Box<Interface>,
    stroeks_map: HashMap<char, Vec<Vec<[i32;2]>>>,
    brush_anim: Vec<Point>,
    character: Option<char>,
    strokes: Vec<Vec<Point>>,
    stroke_index: usize,
}

impl Controller{
    pub fn new(interface: Box<Interface>) ->Controller{
        Controller{
            stroeks_map: deserialize(&DATA[..]).unwrap(),
            brush_anim: vec![],
            interface: interface,
            character: None,
            stroke_index: 0,
            strokes: vec![]
        }
    }

    //更新动画
    pub fn update(&mut self) -> bool{
        //let log = format!("controller::update 长度:{} {:?}", self.brush_anim.len(), self.brush_anim.get(0));
        //self.interface.log(&log);
        if self.brush_anim.len()>0{
            let _point = self.brush_anim.remove(0);
            true
        }else{
            self.on_animation_end();
            false
        }
    }

    //绘制
    pub fn render(&mut self){
        //self.interface.log("controller::render");
        let interface = &self.interface;
        interface.save();
        let (width, height) = (interface.canvas_width(), interface.canvas_height());
        let font_size = width as f64 * 0.9;
        interface.set_font(&format!("{}px FZKTJW", font_size as i32));
        //画田字格
        interface.set_fill_style_color("#eae4c6");
        interface.fill_rect(0.0, 0.0, width as f64, height as f64);
        interface.set_stroke_style_color("#c02c38");
        interface.set_line_width(4.0);
        interface.stroke_rect(0.0, 0.0, width as f64, height as f64);

        interface.begin_path();
        interface.set_line_width(1.5);
        interface.set_line_dash(vec![5.0, 5.0]);
        interface.move_to(0.0, 0.0);
        interface.line_to(width as f64, height as f64);
        interface.move_to(width as f64, 0.0);
        interface.line_to(0.0, height as f64);
        interface.move_to(width as f64/2.0, 0.0);
        interface.line_to(width as f64/2.0, height as f64);
        interface.move_to(0.0, height as f64/2.0);
        interface.line_to(width as f64, height as f64/2.0);
        interface.stroke();

        //画字
        interface.set_fill_style_color("#6674787a");
        interface.set_text_align("center");
        interface.set_text_baseline("middle");
        interface.fill_text(&self.character.unwrap().to_string(), width as f64/2.0, height as f64/2.0+font_size*0.045, None);

        //笔画路径
        //原始宽高 900x900, dx=180,dy=85
        //计算比例
        let scale = width as f64/900.0;

        //测试笔画
        interface.save();
        interface.set_stroke_style_color("#000088");
        interface.begin_path();
        interface.translate(scale*88.0, scale*48.0);
        interface.scale(scale, scale);
        // interface.set_line_dash(vec![]);
        // for points in &self.strokes{
        //     interface.move_to(points[0].x, points[0].y);
        //     for i in 1..points.len(){
        //         interface.line_to(points[i].x, points[i].y);
        //     }
        // }
        //interface.stroke();
        
        //绘制画笔
        if self.brush_anim.len()>0{
            interface.darw_brush(self.brush_anim[0].x, self.brush_anim[0].y - interface.brush_height());   
        }
        interface.restore();
        interface.restore();
    }

    pub fn init(&mut self){
        self.character = Some('了');
        self.create_strokes();
        self.brush_anim = self.strokes[0].clone();
        self.animate(30);
    }

    pub fn on_animation_end(&mut self){
        if self.stroke_index<self.strokes.len()-1{
            self.stroke_index += 1;
            self.brush_anim = self.strokes[self.stroke_index].clone();
            self.animate(30);
        }
    }

    pub fn animate(&mut self, delay: u32){
        self.render();
        let update = self.update();
        if update{
            self.interface.next_frame(delay);
        }
    }

    //创建笔画数组
    pub fn create_strokes(&mut self){
        let strokes:&Vec<Vec<[i32;2]>> = self.stroeks_map.get(&self.character.unwrap()).unwrap();
        self.strokes.clear();
        for i in 0..strokes.len(){
            self.strokes.push(
                resample(strokes[i].iter().map(|p|{
                    Point::new(p[0], p[1], i+1)
                }).collect(), 30)
            );
        }
    }
}

