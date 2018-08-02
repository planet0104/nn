use sdl2::rect::Point;
use nn::NeuralNetwork;
use vector_2d::{Vector2D, Float};
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::gfx::primitives::DrawRenderer;
use data::Data;

//内置模式总数
pub const NUM_PATTERNS:usize = 11;
//每个模式包含多少个向量
pub const NUM_VECTORS:usize = 12;
//公差
pub const MATCH_TOLERANCE:f32 = 0.96;
pub const LEARNING_RATE:f32 = 0.2;
pub const NUM_HIDDEN_NEURONS:usize = 6;

pub const WINDOW_WIDTH:i32 = 400;
pub const WINDOW_HEIGHT:i32 = 400;


pub struct Controller{
    drawing: bool,//是否正在绘制
	path: Vec<Point>,//鼠标手势路径 未处理和平滑处理过的
	smooth_path: Vec<Point>,
	vectors: Vec<f32>,//平滑路径转换成向量

    highest_output: f32, //网络过程最高输出. 最可能被选中的手势
    best_match: i32, //基于higest_output的最好的手势
    the_match: i32, //如果网络发现一个模式，这个是匹配
    
    num_smooth_points:usize,//未处理鼠标手势将被用下面个数的点平滑处理
    num_valid_patterns:usize, //数据库的模式数量

    net: NeuralNetwork, //神经网络
    data: Data
}

impl Controller{
    pub fn new()->Controller{

        Controller{
            path: vec![],
            smooth_path: vec![],
            vectors: vec![],
            drawing: false,
            highest_output: 0.0,
            best_match: -1,
            the_match: -1,
            num_smooth_points: NUM_VECTORS+1,
            num_valid_patterns: NUM_PATTERNS,
            net: NeuralNetwork::new(NUM_VECTORS*2, NUM_HIDDEN_NEURONS, NUM_PATTERNS, LEARNING_RATE),
            data: Data::new(NUM_PATTERNS, NUM_VECTORS)
        }
    }

    //清空鼠标向量
    pub fn clear(&mut self){
        self.path.clear();
        self.smooth_path.clear();
        self.vectors.clear();
    }

    /// 鼠标按下或释放时调用
    /// drawing = true 鼠标按下，原有数据清除
    /// drawing = false 手势已完成。测试它是否与已经存在的某个模式匹配。
    pub fn set_drawing(&mut self, drawing: bool){
        if drawing{
            self.clear();
        }else{
            if self.smooth(){
                //创建向量
                self.create_vectors();
                self.test_for_match();
            }
        }
        self.drawing = drawing;
    }

    /// 给出一系列点, 创建一个路径
    fn create_vectors(&mut self){
        for p in 1..self.smooth_path.len(){
            let x = self.smooth_path[p].x - self.smooth_path[p-1].x;
            let y = self.smooth_path[p].y - self.smooth_path[p-1].y;
            //let v1 = Vector2D::new(1.0, 0.0);
            let mut v2 = Vector2D::new(x as Float, y as Float);
            Vector2D::normalize(&mut v2);
            self.vectors.push(v2.x);
            self.vectors.push(v2.y);
        }
    }

    /// 将鼠标数据转换成一定数量的点
    fn smooth(&mut self) -> bool{
        if self.path.len() < self.num_smooth_points{
            false
        }else{

            //复制原始未加工的鼠标数据
            self.smooth_path = self.path.clone();

            //当点数过多时，通过对所有点的循环，找出最小的跨度，在它原有位置中间创建一个新点，并删除原有的点
            while self.smooth_path.len() > self.num_smooth_points{
                let mut shortest_so_far = 99999999.0;
                let mut point_marker = 0;
                //计算最短跨度(即相邻两点间的距离)
                for span_front in 2..self.smooth_path.len()-1{
                    //计算这些点之间的距离
                    let len =
                        (((self.smooth_path[span_front-1].x -
                        self.smooth_path[span_front].x) *
                        (self.smooth_path[span_front-1].x -
                        self.smooth_path[span_front].x) +
                        (self.smooth_path[span_front-1].y -
                        self.smooth_path[span_front].y) *
                        (self.smooth_path[span_front-1].y -
                        self.smooth_path[span_front].y)) as f32).sqrt();
                    if len < shortest_so_far{
                        shortest_so_far = len;
                        point_marker = span_front;
                    }
                }

                //找出最短跨度，然后计算跨度的中点，作为新点的插入位置，并删除跨度原来的两个点
                let mut new_point = Point::new(0, 0);
                new_point.x = (self.smooth_path[point_marker-1].x + self.smooth_path[point_marker].x)/2;
                new_point.y = (self.smooth_path[point_marker-1].y + self.smooth_path[point_marker].y)/2;
                self.smooth_path[point_marker-1] = new_point;
                self.smooth_path.remove(point_marker);
            }

            true
        }
    }

    //在先前学习好的手势中测试一个适合学模式的手势
    fn test_for_match(&mut self){
        //将平滑后的鼠标数据输入网络并找到匹配
        let output_matrx = self.net.query(&self.vectors);

        let values:&Vec<Vec<f32>> = output_matrx.matrix();

        self.highest_output = 0.0;
        self.best_match = 0;
        self.the_match = -1;

        for i in 1..values.len() {
            if values[i][0] > self.highest_output {
                self.highest_output = values[i][0];
                self.best_match = i as i32;
                if self.highest_output > MATCH_TOLERANCE{
                    self.the_match = self.best_match;
                }
            }
        }
    }

    pub fn add_point(&mut self, point: Point){
        self.path.push(point);
    }
    
    pub fn render(&self, canvas: &mut Canvas<Window>){
        if !self.drawing{
            //绘制最匹配的
            if self.highest_output>0.0{
                if self.smooth_path.len() > 1{
                    if self.highest_output<MATCH_TOLERANCE{
                        //如果最高输出小于容人值
                        println!("我猜是: {}", self.data.pattern_name(self.best_match as usize));
                    }else{
                        println!("结果是: {}", self.data.pattern_name(self.the_match as usize));
                    }
                }else{
                    println!("没有足够的点绘图，再试一次");
                }
            }
        }

        if self.path.len() < 1{
            return;
        }
        
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        //画路径
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_lines(self.path.as_slice()).unwrap();

        //将平滑的路径链接起来(画圈)
        if !self.drawing && self.smooth_path.len() > 0{
            for vtx in 1..self.smooth_path.len(){
                canvas.circle(self.smooth_path[vtx].x as i16, self.smooth_path[vtx].y as i16, 3, Color::RGB(0, 0, 0)).unwrap();
            }
        }

        canvas.present();
    }

    pub fn drawing(&self) -> bool{
        self.drawing
    }

    pub fn train(&mut self){
        let set_in = self.data.get_input_set();
        let set_out = self.data.get_output_set();

        assert!(set_in.len() == set_out.len()
            && set_in[0].len() == NUM_VECTORS*2
            && set_out[0].len() == NUM_PATTERNS);
        
        let mut error = 1.0;
        let mut epoch = 0;
        while error>=0.2{
            for i in 0..set_in.len(){
                self.net.train(&set_in[i], &set_out[i]);
            }
            
            error = 0.0;
            //计算误差
            for i in 0..set_in.len(){
                //查询网络
                let outputs = self.net.query(&set_in[i]);
                
                //找到最高的输出
                let values:Vec<f32> = outputs.matrix().iter().map(|o| o[0]).collect();
                //println!("outputs={:?}", values);
                let mut max_val = values[0];
                let mut max_index = 0;
                for i in 1..values.len() {
                    if values[i] > max_val {
                        max_val = values[i];
                        max_index = i;
                    }
                }
                
                error += 0.99 - values[max_index];
            }

            if epoch%1000 == 0{
                println!("error={}", error);
            }

            epoch += 1;
        }
    }
}