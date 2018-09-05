mod pdollarplus;
const DATA:&[u8] = include_bytes!("../../stroke_data");
use bincode::deserialize;
use std::collections::HashMap;

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

pub trait Context2D{
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
}

pub struct Controller{
    context: Box<Context2D>,
    stroeks_map: HashMap<char, Vec<Vec<[i32;2]>>>,
}

impl Controller{
    pub fn new(context: Box<Context2D>) ->Controller{
        Controller{
            context,
            stroeks_map: deserialize(&DATA[..]).unwrap()
        }
    }

    pub fn init(&mut self){
        let (width, height) = (self.context.canvas_width(), self.context.canvas_height());
        let font_size = width as f64 * 0.9;
        self.context.set_font(&format!("{}px FZKTJW", font_size as i32));
        //画田字格
        self.context.set_fill_style_color("#eae4c6");
        self.context.fill_rect(0.0, 0.0, width as f64, height as f64);
        self.context.set_stroke_style_color("#c02c38");
        self.context.set_line_width(4.0);
        self.context.stroke_rect(0.0, 0.0, width as f64, height as f64);

        self.context.begin_path();
        self.context.set_line_width(1.5);
        self.context.set_line_dash(vec![5.0, 5.0]);
        self.context.move_to(0.0, 0.0);
        self.context.line_to(width as f64, height as f64);
        self.context.move_to(width as f64, 0.0);
        self.context.line_to(0.0, height as f64);
        self.context.move_to(width as f64/2.0, 0.0);
        self.context.line_to(width as f64/2.0, height as f64);
        self.context.move_to(0.0, height as f64/2.0);
        self.context.line_to(width as f64, height as f64/2.0);
        self.context.stroke();

        //画字
        self.context.set_fill_style_color("#6674787a");
        self.context.set_text_align("center");
        self.context.set_text_baseline("middle");
        self.context.fill_text("灯", width as f64/2.0, height as f64/2.0+font_size*0.045, None);

        //笔画路径
        //原始笔画50x70 倍数20倍即 ????
        //宽度width
        let strokes:&Vec<Vec<[i32;2]>> = self.stroeks_map.get(&'灯').unwrap();
        self.context.save();
        self.context.begin_path();
        self.context.scale(0.4, 0.4);
        self.context.set_line_dash(vec![]);
        for points in strokes{
            self.context.move_to(points[0][0] as f64, points[0][1] as f64);
            for i in 1..points.len(){
                self.context.line_to(points[i][0] as f64, points[i][1] as f64);
            }
        }
        self.context.stroke();
        self.context.restore();
    }
}

