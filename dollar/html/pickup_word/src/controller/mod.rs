mod pdollarplus;
const DATA:&[u8] = include_bytes!("../../stroke_data");
use bincode::deserialize;
use std::collections::HashMap;
use self::pdollarplus::{Point, resample, PDollarPlusRecognizer};

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
    fn fill_circle(&self, x: f64, y: f64, radius: f64);
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
    strokes_map: HashMap<char, Vec<Vec<[i32;2]>>>,
    stroke_anim: Vec<Point>, //当前动画的指示器，每当新的笔画开始，数组清空，每一帧的时候放入一个当前笔画的点，直到数组长度和当前笔画点数相等。
    character: Option<char>,
    strokes: Vec<Vec<Point>>,
    stroke_index: usize,
    user_strokes: Vec<Vec<Point>>,//用户的笔画
    writing: bool, //正在写入
    complete: bool, //当前文字已写完
}

impl Controller{
    pub fn new(interface: Box<Interface>) ->Controller{
        Controller{
            strokes_map: deserialize(&DATA[..]).unwrap(),
            stroke_anim: vec![],
            interface: interface,
            character: None,
            stroke_index: 0,
            strokes: vec![],
            user_strokes: vec![],
            writing: false,
            complete: false
        }
    }

    //更新动画
    pub fn update(&mut self) -> bool{
        //let log = format!("controller::update 长度:{} {:?}", self.brush_anim.len(), self.brush_anim.get(0));
        //self.interface.log(&log);
        let anim_len = self.stroke_anim.len();
        if anim_len < self.strokes[self.stroke_index].len(){
            //添加下一个笔画点
            self.stroke_anim.push(self.strokes[self.stroke_index][anim_len].clone());
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

        //----------------- 画田字格 --------------------------
        interface.begin_path();
        let (width, height) = (interface.canvas_width(), interface.canvas_height());
        interface.set_fill_style_color("#eae4c6");
        interface.fill_rect(0.0, 0.0, width as f64, height as f64);
        interface.set_stroke_style_color("#c02c38");
        interface.set_line_width(4.0);
        interface.stroke_rect(0.0, 0.0, width as f64, height as f64);

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
        
        //--------------------- 画字 ----------------------------------
        let font_size = width as f64 * 0.9;
        interface.set_font(&format!("{}px FZKTJW", font_size as i32));
        interface.set_fill_style_color("#6674787a");
        interface.set_text_align("center");
        interface.set_text_baseline("middle");
        interface.fill_text(&self.character.unwrap().to_string(), width as f64/2.0, height as f64/2.0+font_size*0.045, None);

        //------------------ 绘制用户的笔画 -----------------------------
        interface.begin_path();
        interface.set_line_dash(vec![]);
        interface.set_stroke_style_color("#f00");
        interface.set_line_width(10.0);
        for points in &self.user_strokes{
            interface.move_to(points[0].x, points[0].y);
            for point in points{
                interface.line_to(point.x, point.y);
            }
        }
        interface.stroke();

        //--------------------- 画笔动画 --------------------------
        interface.set_stroke_style_color("#000088");
        interface.begin_path();
        interface.set_line_dash(vec![]);
        interface.set_line_width(1.0);
        interface.save();
        //测试笔画
        //笔画路径
        //原始宽高 900x900, dx=180,dy=85
        //计算比例
        let scale = width as f64/900.0;
        interface.translate(scale*88.0, scale*48.0);
        interface.scale(scale, scale);
        // let strokes:&Vec<Vec<[i32;2]>> = self.strokes_map.get(&self.character.unwrap()).unwrap();
        // for points in strokes{
        //     // if points.len()>6{
        //     //     interface.stroke_rect(points[6][0] as f64, points[6][1] as f64, (10) as f64, (10)as f64);
        //     // }
        //     interface.move_to(points[0][0] as f64, points[0][1] as f64);
        //     for i in 1..points.len(){
        //         interface.line_to(points[i][0] as f64, points[i][1] as f64);
        //     }
        // }
        // for points in &self.strokes{
        //     interface.move_to(points[0].x, points[0].y);
        //     for i in 1..points.len(){
        //         interface.line_to(points[i].x, points[i].y);
        //     }
        // }
        // interface.stroke();
        
        //绘制笔画指示器
        interface.set_fill_style_color("#773399cc");
        interface.set_stroke_style_color("#773399cc");
        interface.set_line_width(8.0);
        //在最后一个点画箭头
        
        for i in 0..self.stroke_anim.len(){
            let point = &self.stroke_anim[i];
            if i<=6{//画3个圆点
                if i%3 == 0{
                    self.interface.fill_circle(point.x, point.y, 6.0);
                }
            }
            if i==9{
                self.interface.begin_path();
                self.interface.move_to(self.stroke_anim[i].x, self.stroke_anim[i].y);
            }
            if i>9{
                self.interface.line_to(self.stroke_anim[i].x, self.stroke_anim[i].y);
            }
        }
        if self.stroke_anim.len()>9{
            self.interface.stroke();
        }

        interface.restore();
    }

    pub fn init(&mut self){
        self.character = Some('繁');
        //爨、躞 的笔画是错的
        //辨的中间一笔画有问题
        //
        self.create_strokes();
        self.animate(60);
    }

    pub fn on_animation_end(&mut self){
        self.interface.log(&format!("第{}笔动画结束", self.stroke_index));
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
        let strokes:&Vec<Vec<[i32;2]>> = self.strokes_map.get(&self.character.unwrap()).unwrap();
        self.interface.log(&format!("一共{}笔", strokes.len()));
        self.strokes.clear();
        for i in 0..strokes.len(){
            self.interface.log(&format!("第{}笔 {:?}", i, strokes[i]));
            self.strokes.push(
                resample(strokes[i].iter().map(|p|{
                    Point::new(p[0], p[1], i+1)
                }).collect(), 30)
            );
        }
    }

    fn calculate_score(&mut self){
        self.interface.log("计算当前笔画得分...");
        let mut recognizer = PDollarPlusRecognizer::new();
        //将当前笔画加入识别器
        recognizer.add_gesture("stroke", self.strokes[self.stroke_index].clone());
        //识别用户当前笔画
        let result = recognizer.recognize(self.user_strokes[self.stroke_index].clone());
        self.interface.log(&format!("笔画得分:{} > {}分", result.name, result.score));

        //检查是否写完
        if self.user_strokes.len() == self.strokes.len(){
            self.complete = true;

            //检查整个汉字是否正确
            recognizer.clear_gestures();
            let mut points = vec![];
            for stroke in &self.strokes{
                points.append(&mut stroke.clone());
            }
            //self.interface.log(&format!("原始:{:?}", points));
            recognizer.add_gesture(&self.character.unwrap().to_string(), points);
            
            let mut user_points = vec![];
            for i in 0..self.user_strokes.len(){
                for point in &self.user_strokes[i]{
                    user_points.push(point.clone());
                }
            }
            //self.interface.log(&format!("用户:{:?}", user_points));
            let result = recognizer.recognize(user_points);
            self.interface.log(&format!("整字得分:{} > {}分", result.name, result.score));

            self.interface.log("完成.");
        }

        self.writing = false;
        if !self.complete{
            //切换到下一笔画
            self.stroke_index += 1;
            self.stroke_anim.clear();
            self.animate(60);
        }
    }

    pub fn on_pointer_down(&mut self, client_x:i32, client_y:i32, offset_x:f64, offset_y:f64){
        if self.complete{
            self.interface.log("已完成.");
            return;
        }
        //创建新的笔画
        self.user_strokes.push(vec![Point::new(offset_x, offset_y, self.stroke_index+1)]);
        self.writing = true;
    }

    pub fn on_pointer_up(&mut self, client_x:i32, client_y:i32, offset_x:f64, offset_y:f64){
        if self.writing{
            self.calculate_score();
        }
    }

    pub fn on_pointer_move(&mut self, client_x:i32, client_y:i32, offset_x:f64, offset_y:f64){
        if self.writing{
            //加入画点
            self.user_strokes[self.stroke_index].push(Point::new(offset_x, offset_y, self.stroke_index+1));
            self.render();
        }
    }
    pub fn on_pointer_out(&mut self, client_x:i32, client_y:i32, offset_x:f64, offset_y:f64){
        if self.writing{
            self.calculate_score();
        }
    }
}