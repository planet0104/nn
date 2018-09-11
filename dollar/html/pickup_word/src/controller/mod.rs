mod pdollarplus;
mod vector;
const DATA:&[u8] = include_bytes!("../../stroke_data");
use bincode::deserialize;
use std::collections::HashMap;
use self::pdollarplus::{Point, resample, PDollarPlusRecognizer};
//use std::f64::consts::PI;

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
    fn set_canvas_width(&self, width: u32);
    fn set_canvas_height(&self, height: u32);
    fn set_line_dash(&self, segments: Vec<f64>);
    fn save(&self);
    fn restore(&self);
    fn scale(&self, x: f64, y: f64);
    fn translate(&self, x: f64, y: f64);
    //控制画刷
    fn darw_brush(&self, x:f64, y:f64);
    fn brush_height(&self) -> f64;
    fn next_frame(&self, delay: u32);
    fn log(&self, s:&str);
    fn log_div(&self, s:&str);
    fn rotate(&self, angle: f64);
    fn window_width(&self) -> i32;
    fn window_height(&self) -> i32;
    fn show_error_icon(&self, x:f64, y:f64);
    fn show_ok_icon(&self, x:f64, y:f64);
}

pub struct Controller{
    interface: Box<Interface>,
    strokes_map: HashMap<char, Vec<Vec<[i32;2]>>>,
    stroke_anim: Vec<Point>, //当前动画的指示器，每当新的笔画开始，数组清空，每一帧的时候放入一个当前笔画的点，直到数组长度和当前笔画点数相等。
    character: usize, //当前第几个字
    drawing_ch: Option<char>, //当前绘制的字
    strokes: Vec<Vec<Point>>,
    stroke_index: usize,
    user_strokes: Vec<Vec<Point>>,//用户的笔画
    stroke_scores: Vec<f64>,
    writing: bool, //正在写入
    complete: bool, //当前文字已写完,
    homework: String,
    animating: bool,
    error_icon: Option<Point>,
    ok_icon: Option<Point>,
}

impl Controller{
    pub fn new(interface: Box<Interface>) ->Controller{
        Controller{
            strokes_map: deserialize(&DATA[..]).unwrap(),
            stroke_anim: vec![],
            interface: interface,
            character: 0,
            stroke_index: 0,
            strokes: vec![],
            user_strokes: vec![],
            writing: false,
            complete: false,
            homework: "静夜思 李白\n床前明月光，疑是地上霜。\n举头望明月，低头思故乡。".to_string(),
            //homework: "故乡。".to_string(),
            animating: false,
            drawing_ch: None,
            stroke_scores: vec![],
            error_icon: None,
            ok_icon: None,
        }
    }

    //更新动画
    pub fn update(&mut self) -> bool{
        //let log = format!("controller::update 长度:{}", self.stroke_anim.len());
        //self.interface.log(&log);
        let anim_len = self.stroke_anim.len();
        if anim_len < self.strokes[self.stroke_index].len(){
            //添加下一个笔画点
            self.stroke_anim.push(self.strokes[self.stroke_index][anim_len].clone());
            true
        }else{
            self.on_animation_end();
            self.stroke_anim.clear();
            self.render();
            false
        }
    }

    //绘制
    pub fn render(&mut self){
        //self.interface.log("controller::render");
        let interface = &self.interface;

        //----------------- 画田字格 --------------------------
        interface.set_line_dash(vec![]);
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
        interface.set_fill_style_color("rgb(116,120,122,0.5)");
        interface.set_text_align("center");
        interface.set_text_baseline("middle");
        let drawing_ch = if let Some(ch) = self.drawing_ch{
            ch
        }else{
            self.homework.chars().nth(self.character).unwrap()
        };
        interface.fill_text(&drawing_ch.to_string(), width as f64/2.0, height as f64/2.0+font_size*0.045, None);

        //------------------ 绘制用户的笔画 -----------------------------
        interface.begin_path();
        interface.set_line_dash(vec![]);
        interface.set_stroke_style_color("#777");
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

        //绘制画笔
        if self.stroke_anim.len()>0{
            interface.darw_brush(self.stroke_anim.last().unwrap().x, self.stroke_anim.last().unwrap().y - interface.brush_height());   
        }

        if let Some(pos) = &self.error_icon{
            interface.show_error_icon(pos.x, pos.y);
        }
        if let Some(pos) = &self.ok_icon{
            interface.show_ok_icon(pos.x, pos.y);
        }

        interface.restore();
    }

    pub fn init(&mut self){
        self.interface.set_canvas_width(self.interface.window_width() as u32);
        self.interface.set_canvas_height(self.interface.window_width() as u32);
        //爨、躞 的笔画是错的
        //辨的中间一笔画有问题
        //
        self.create_strokes();
        self.animate(30);
    }

    pub fn on_animation_end(&mut self){
        self.animating = false;
        self.interface.log(&format!("第{}笔动画结束", self.stroke_index));
    }

    pub fn animate(&mut self, delay: u32){
        self.animating = true;
        self.render();
        let update = self.update();
        if update{
            self.interface.next_frame(delay);
        }
    }

    //创建笔画数组
    pub fn create_strokes(&mut self){
        //self.interface.log(&format!("create_strokes character={}", self.character));
        let strokes:&Vec<Vec<[i32;2]>> = self.strokes_map.get(&self.homework.chars().nth(self.character).unwrap()).unwrap();
        //self.interface.log(&format!("一共{}笔", strokes.len()));
        self.strokes.clear();
        for i in 0..strokes.len(){
            //self.interface.log(&format!("第{}笔 {:?}", i, strokes[i]));
            self.strokes.push(
                resample(strokes[i].iter().map(|p|{
                    Point::new(p[0], p[1], i+1)
                }).collect(), 30)
            );
        }
    }

    /**
     * 低于50分不通过。
     * 笔画差距超过16不通过。
     */
    fn calculate_score(&mut self){
        self.writing = false;
        self.interface.log("计算当前笔画得分...");

        let mut recognizer = PDollarPlusRecognizer::new();

        //笔画少于10点重写
        if self.user_strokes[self.stroke_index].len()>5{
            //将当前笔画加入识别器
            recognizer.add_gesture("stroke", self.strokes[self.stroke_index].clone());
            //识别用户当前笔画
            let result = recognizer.recognize(self.user_strokes[self.stroke_index].clone());
            self.interface.log_div(&format!("笔画得分:{} > {}分", result.name, result.score));
            self.stroke_scores.push(result.score);
        }else{
            self.stroke_scores.push(100.0);
        }

        //检查是否写完
        if self.user_strokes.len() == self.strokes.len(){
            //检查整个汉字是否正确
            recognizer.clear_gestures();
            let mut points = vec![];
            for stroke in &self.strokes{
                points.append(&mut stroke.clone());
            }
            //self.interface.log(&format!("原始:{:?}", points));
            recognizer.add_gesture(&self.homework.chars().nth(self.character).unwrap().to_string(), points);
            
            let mut user_points = vec![];
            for i in 0..self.user_strokes.len(){
                for point in &self.user_strokes[i]{
                    user_points.push(point.clone());
                }
            }
            //self.interface.log(&format!("用户:{:?}", user_points));
            let result = recognizer.recognize(user_points);
            // 写3分算满分的话，10.0-3.0=7.0
            //写5分, 10.0-5.0=4.0, 得0.66分
            let score1 = (10.0-result.score)/5.0;
            //计算每个笔画的得分， 超出10分减分，小于10分加分

            //假设每笔写 1.0满分,笔画为3笔 那么满分为 10-1=9, 满分27
            let mut total_score = 0.0;
            for score in &self.stroke_scores{
                total_score += 10.0-score;
            }
            //1.5满分
            let score2 = (total_score/(8.5*self.strokes.len() as f64))*90.0;
            self.interface.log_div(&format!("整字得分:{}分", score1));
            
            self.interface.log_div(&format!("笔画得分:{}", score2));
            //最高100分
            let total = (score1*score2) as i32;
            self.interface.log_div(&format!("总分:{}", if total>100{100}else{total}));
            self.stroke_scores.clear();

            //如果得分低于50分，重写
            if total<50{
                //清空用户笔画和得分
                self.user_strokes.clear();
                self.stroke_index = 0;
                self.stroke_scores.clear();
                if self.animating{
                    self.stroke_anim.clear();
                }else{
                    self.animate(30);
                }
                self.interface.log("重写当前字");
            }else{
                //切换到下一个字
                if self.character >= self.homework.len(){
                    self.complete = true;
                }
                if !self.complete{
                    self.character += 1;
                    let mut ch = self.homework.chars().nth(self.character);
                    //跳过标点符号
                    while !ch.is_none() && self.strokes_map.get(&ch.unwrap()).is_none(){
                        self.interface.log(&format!("跳过{}", ch.unwrap()));
                        self.character += 1;
                        ch = self.homework.chars().nth(self.character);
                    }
                    if ch.is_none(){
                        self.complete = true;
                    }
                    if !self.complete{
                        self.drawing_ch = ch;
                        self.stroke_index = 0;
                        self.create_strokes();
                        self.user_strokes.clear();
                        if self.animating{
                            self.stroke_anim.clear();
                        }else{
                            self.animate(30);
                        }
                        self.interface.log("下一个字");
                    }
                }
                if self.complete{
                    self.interface.log("完成.");
                }
            }
        }else{
            if !self.complete{
                //笔画距离超过15不通过
                if self.stroke_scores[self.stroke_index] < 7.0{
                    //切换到下一笔画
                    self.stroke_index += 1;
                    //显示对勾
                    if self.ok_icon.is_some(){
                        self.ok_icon = None;
                    }
                    if self.error_icon.is_some(){
                        self.ok_icon = self.error_icon.clone();
                        self.error_icon = None;
                    }
                }else{
                    //删除当前笔画得分
                    self.stroke_scores.remove(self.stroke_index);
                    //清空当前错误的笔画
                    self.user_strokes.remove(self.stroke_index);
                    //显示错误图标
                    self.ok_icon = None;
                    self.error_icon = Some(self.strokes[self.stroke_index][0].clone());
                    self.interface.log("清空当前错误的笔画");
                }
                if self.animating{
                    self.stroke_anim.clear();
                }else{
                    self.animate(30);
                }
            }
        }
    }

    fn log_usize(&self, s:&str, val: usize){
        self.interface.log(&format!("{}{}", s, val));
    }

    pub fn on_pointer_down(&mut self, _client_x:i32, _client_y:i32, offset_x:f64, offset_y:f64){
        self.interface.log("按下");
        if self.complete{
            self.interface.log("已完成.");
            return;
        }
        //创建新的笔画
        self.user_strokes.push(vec![Point::new(offset_x, offset_y, self.stroke_index+1)]);
        self.writing = true;
    }

    pub fn on_pointer_up(&mut self){
        if self.writing{
            self.calculate_score();
        }
    }

    pub fn on_pointer_move(&mut self, _client_x:i32, _client_y:i32, offset_x:f64, offset_y:f64){
        if self.writing{
            //加入画点
            self.user_strokes[self.stroke_index].push(Point::new(offset_x, offset_y, self.stroke_index+1));
            self.render();
        }
    }
    pub fn on_pointer_out(&mut self){
        if self.writing{
            self.calculate_score();
        }
    }
}